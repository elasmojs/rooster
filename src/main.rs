//#![windows_subsystem = "windows"]

use std::time::SystemTime;
use std::clone::Clone;
use std::fs::{metadata};
use std::io::{Cursor, Read, Error as IoError};
use std::{net::SocketAddr, path::Path};
use std::collections::HashMap;
use std::str;

use log::*;

use futures_util::future;
use futures_util::stream::TryStreamExt;

use hyper::{Body, Request, Response, Server, StatusCode, HeaderMap, header::HeaderValue};
use hyper::service::{make_service_fn, service_fn};
use hyper_staticfile::Static;
use hyper::server::conn::AddrStream;

use multipart::server::{Multipart};

mod logger;
mod strings;
mod props;
mod resources;
mod admin;
//mod rhaiscript;
mod jsscript;
mod api;


use strings::StringUtils;
use props::Props;

const ADMIN_ROUTE:&str = "/_gale";
//const RHAI_SCRIPT_EXTN:&str = ".rhai";
const JS_SCRIPT_EXTN:&str = ".js";
const F_SLASH:&str = "/";
const DOT:&str = ".";
const CONTENT_TYPE:&str = "content-type";
const MULTI_PART_CONTENT_TYPE:&str = "multipart/form-data";

#[derive(Clone)]
pub struct MPFile{
    pub name: String,
    pub file_name: String,
    pub file_content: Vec<u8>,
    pub content_type: String
}

impl MPFile{
    fn new(name: String, file_name: String, file_content: Vec<u8>, content_type: String) -> MPFile{
        return MPFile{
            name: name,
            file_name: file_name,
            file_content: file_content,
            content_type: content_type
        };
    }
}

#[derive(Clone)]
pub struct RequestData{
    pub method: String,
    pub path: String,
    pub query: String,
    pub headers: HashMap<String, String>,
    pub body: String,
    pub is_multipart: bool,
    pub fields: HashMap<String, String>,
    pub files: HashMap<String, MPFile>
}

impl RequestData{
    fn new(method: String, path: String, query: String, headers:HashMap<String, String>, body: String, is_multipart:bool) -> RequestData{
        return RequestData{
            method: method,
            path: path,
            query: query,
            headers: headers,
            body: body,
            is_multipart: is_multipart,
            fields: HashMap::new(),
            files: HashMap::new(),
        };
    }
}


async fn process_request(req: Request<Body>, static_:Static, props: Props) -> Result<Response<Body>, IoError> {
    
    let uri = req.uri().clone();
    let req_path = uri.path().clone();
    let query = match uri.query().clone(){
        Some(q) => String::from(q),
        None => String::from("")
    };

    let last_slash_idx = String::from(req_path).last_index_of(F_SLASH) as usize;
    let file_name = &req_path[last_slash_idx+1..];
    let not_file = String::from(file_name).last_index_of(DOT) == -1;
    //let mut is_rhai_script = false;
    let mut is_js_script = false;
    let mut is_folder = false;
    let mut is_admin = false; 

    if not_file{
        match metadata(props.web_root.clone() + req_path){
            Ok(md) => {
                if md.is_dir(){
                    is_folder = true;
                }
            }
            Err(_e) => {
                if req_path.contains(ADMIN_ROUTE){
                    is_admin = true;
                }else if Path::new(&(props.server_root.clone() + req_path + JS_SCRIPT_EXTN)).exists(){
                    is_js_script = true;
                }/*else if Path::new(&(props.web_root.clone() + req_path + RHAI_SCRIPT_EXTN)).exists(){
                    is_rhai_script = true;
                }*/
            }
        };
    }else{
        if req_path.to_lowercase().contains(ADMIN_ROUTE){
            is_admin = true;
        }/*else if req_path.to_lowercase().contains(JS_SCRIPT_EXTN){
            error!("Error serving request for path: {}", req_path);
            let mut response = Response::default();
            *response.status_mut() = StatusCode::FORBIDDEN;
            return Ok(response);
        }else if req_path.to_lowercase().contains(RHAI_SCRIPT_EXTN){
            error!("Error serving request for path: {}", req_path);
            let mut response = Response::default();
            *response.status_mut() = StatusCode::FORBIDDEN;
            return Ok(response);
        }*/
    }

    let stime = SystemTime::now();
    match req_path{
        req_path if is_folder =>{
            //process folders

            info!("Serving request for folder: {}", req_path);
            let tpath = req_path.trim_end();
            let lchar = String::from(tpath).pop().unwrap();
            let default_uri;
            if lchar == '/'{
                default_uri = format!("{}{}", req_path, props.web_default);
                let request = Request::get(default_uri)
                    .body(())
                    .unwrap();
                debug!("Time taken to serve {}: {} ms", req_path, stime.elapsed().unwrap().as_millis());
                return static_.clone().serve(request).await;
            }else{
                default_uri = format!("{}/", req_path);
                let query_opt = uri.query();
                let redir_url:String;
                if query_opt.is_some(){
                    redir_url = format!("{}?{}", default_uri, query_opt.unwrap());
                }else{
                    redir_url = format!("{}", default_uri);
                }
                
                let mut response = Response::default();
                let hmut = response.headers_mut();
                hmut.insert("location", HeaderValue::from_bytes(redir_url.as_bytes()).unwrap());
                *response.status_mut() = StatusCode::TEMPORARY_REDIRECT;
                return Ok(response);
            }
        },
        req_path if is_admin =>{
            //process gale admin
            info!("Serving admin request for path: {}", req_path);
            let admin_resp = admin::process(req_path, props).await;
            debug!("Time taken to serve {}: {} ms", req_path, stime.elapsed().unwrap().as_millis());
            return admin_resp;
        },
        req_path if is_js_script =>{
            //process js script
            info!("Serving script request for path: {}", req_path);
            let method = req.method().clone().to_string();
            let headers = get_headers(req.headers().clone()).await;
            let path = String::from(req_path);

            let mut is_multipart = false;
            let has_content_type = headers.contains_key(CONTENT_TYPE);
            if has_content_type{
                match headers.get(CONTENT_TYPE){
                    Some(ctype) => {
                        if ctype.contains(MULTI_PART_CONTENT_TYPE){
                            is_multipart = true;
                        }
                    },
                    None => is_multipart = false
                }
            }

            let mut req_data = RequestData::new(
                method.clone(),
                path.clone(),
                query.clone(),
                headers.clone(),
                String::from(""),
                false
            );

            if !is_multipart{
                req_data.body = get_body_str(req).await.clone();                
            }else{
               req_data.is_multipart = true;
                get_parts(req, &mut req_data).await;
            }
            let script_resp = jsscript::process(req_data, props).await;
            debug!("Time taken to serve {}: {} ms", req_path, stime.elapsed().unwrap().as_millis());
            return script_resp;
        },
        /* 
        req_path if is_rhai_script =>{
            //process rhai script
            info!("Serving script request for path: {}", req_path);
            let method = req.method().clone().to_string();
            let headers = get_headers(req.headers().clone()).await;
            let path = String::from(req_path);

            let mut is_multipart = false;
            let has_content_type = headers.contains_key(CONTENT_TYPE);
            if has_content_type{
                match headers.get(CONTENT_TYPE){
                    Some(ctype) => {
                        if ctype.contains(MULTI_PART_CONTENT_TYPE){
                            is_multipart = true;
                        }
                    },
                    None => is_multipart = false
                }
            }

            let mut req_data = RequestData::new(
                method.clone(),
                path.clone(),
                query.clone(),
                headers.clone(),
                String::from(""),
                false
            );
            info!("Serving script request for path: {}", req_path);
            if !is_multipart{
                req_data.body = get_body_str(req).await.clone();                
            }else{
               req_data.is_multipart = true;
                get_parts(req, &mut req_data).await;
            }
            return rhaiscript::process(req_data, props).await; 
        },
        */        
        _ => {
                info!("Serving file request for path: {}", req_path);
                let static_resp = static_.clone().serve(req).await;
                debug!("Time taken to serve {}: {} ms", req_path, stime.elapsed().unwrap().as_millis());
                return static_resp;
        }
    }
}

async fn get_headers(headers:HeaderMap) -> HashMap<String, String>{
    let mut headers_map = HashMap::new();
    for (key, value) in headers.iter() {
        let value_str = match value.clone().to_str(){
            Ok(v) => String::from(v),
            Err(_e) => String::from("")
        };
        headers_map.insert(key.clone().to_string().to_lowercase(), value_str);
    }
    return headers_map;
}

async fn get_body_str(req:Request<Body>) -> String{
    let body = req.into_body();
    let body_data = body.try_fold(Vec::new(), |mut data, chunk| async move {
        data.extend_from_slice(&chunk);
        return Ok(data);
    }).await;

    let body_vec = body_data.map(|body_data| {
        return body_data;
    });

    let body_str = match body_vec{
        Ok(vec) => {
            String::from(str::from_utf8(&vec).unwrap())
        },
        Err(e) => {
            format!("Error = {}", e)
        }
    };

    return body_str;
}

async fn get_parts(req:Request<Body>, req_data:&mut RequestData){
    let body = req.into_body();
    let body_data = body.try_fold(Vec::new(), |mut data, chunk| async move {
        data.extend_from_slice(&chunk);
        return Ok(data);
    }).await;

    let body_vec = match body_data.map(|body_data| {
        return body_data;
    }){
        Ok(data_vec) => data_vec,
        Err(_e) => vec![0]
    };

    let ctype = req_data.headers.get("content-type").unwrap().clone();
    let hyph_idx = (ctype.clone().last_index_of("-")+1) as usize;
    let bnd = String::from(&ctype[hyph_idx..]);

    let mut mp = Multipart::with_body(Cursor::new(body_vec.as_slice()), bnd.clone());
    let mut fields:HashMap<String, String> = HashMap::new();
    let mut files:HashMap<String, MPFile> = HashMap::new();

    match mp.foreach_entry(|field|{
        if field.is_text(){
            let mut data_str = String::new();
            let mut data = Box::new(field.data);
            let read_result = data.read_to_string(&mut data_str);
            let hyph_idx = data_str.clone().index_of("-") as usize;
            let value = (&data_str[..hyph_idx]).trim();
            if read_result.is_ok(){
                fields.insert(format!("{}",field.headers.name.clone()), String::from(value));
            }
        }else{
            let name = format!("{}", field.headers.name);
            let fname = field.headers.filename.unwrap().clone();
            let ctype = field.headers.content_type.unwrap().clone();

            let mut fdata:Vec<u8> = Vec::new();
            let mut data = Box::new(field.data);
            let read_result = data.read_to_end(&mut fdata);
            if read_result.is_ok(){
                let mpfile = MPFile::new(name.clone(), fname.clone(), fdata.clone(), format!("{}/{}", ctype.type_(), ctype.subtype()));
                files.insert(name.clone(), mpfile.clone());
            }
        }
    }){
        Ok(()) => info!("All parts processed"),
        Err(_e) => error!("Error processing parts - {}", _e)
    }
    req_data.fields = fields.clone();
    req_data.files = files.clone();
}

async fn shutdown_signal() {
    // Wait for the CTRL+C signal
    tokio::signal::ctrl_c()
        .await
        .expect("failed to install CTRL+C signal handler");
}

#[tokio::main]
async fn main() {
    let mut props = props::get_props();
    logger::init_log(props.clone());

    let net_port = props.net_port.clone();

    let static_ = Static::new(Path::new(&props.web_root));
    
    let service = make_service_fn(|socket: &AddrStream| {
        props.remote_addr = socket.remote_addr().ip().to_string();
        let static_ = static_.clone();
        let props_:Props = props.clone();
        future::ok::<_, hyper::Error>(service_fn(move |req| process_request(req, static_.clone(), props_.clone())))
    });

    let addr = SocketAddr::from(([0, 0, 0, 0], net_port as u16));
    let server = Server::bind(&addr).serve(service);
    let graceful = server.with_graceful_shutdown(shutdown_signal());
    info!("Gale JS server started at => http://localhost:{}", net_port);

    let website_url = format!("http://localhost:{}", net_port);
    let _browser_result = webbrowser::open(&website_url);

    if let Err(e) = graceful.await {
        eprintln!("server error: {}", e);
    } 
}
