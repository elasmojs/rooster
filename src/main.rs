#![windows_subsystem = "windows"]

use std::clone::Clone;
use std::fs::{metadata};
use std::io::Error as IoError;
use std::{net::SocketAddr, path::Path};
use std::collections::HashMap;
use std::str;

use futures_util::future;
use futures_util::stream::TryStreamExt;

use hyper::{Body, Request, Response, Server, StatusCode, HeaderMap};
use hyper::service::{make_service_fn, service_fn};
use hyper_staticfile::Static;
use hyper::server::conn::AddrStream;

mod strings;
mod props;
mod resources;
mod admin;
mod scripts;


use strings::{_last_index_of, _last_char};
use props::Props;

const ADMIN_ROUTE:&str = "/_rooster";
const SCRIPT_EXTN:&str = ".rhai";
const F_SLASH:&str = "/";
const DOT:&str = ".";
const CONTENT_TYPE:&str = "content-type";
const MULTI_PART_CONTENT_TYPE:&str = "multipart/form-data";

pub struct RequestData{
    pub method: String,
    pub path: String,
    pub query: String,
    pub headers: HashMap<String, String>,
    pub body: String,
    pub is_multipart: bool
}

impl RequestData{
    fn new(method: String, path: String, query: String, headers:HashMap<String, String>, body: String, is_multipart:bool) -> RequestData{
        return RequestData{
            method: method,
            path: path,
            query: query,
            headers: headers,
            body: body,
            is_multipart: is_multipart
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
    let last_slash_idx = _last_index_of(String::from(req_path), F_SLASH) as usize;
    let file_name = &req_path[last_slash_idx+1..];
    let not_file = _last_index_of(String::from(file_name), DOT) == -1;
    let mut is_script = false;
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
                }else if Path::new(&(props.web_root.clone() + req_path + SCRIPT_EXTN)).exists(){
                    is_script = true;
                }
            }
        };
    }else{
        if req_path.to_lowercase().contains(ADMIN_ROUTE){
            is_admin = true;
        }else if req_path.to_lowercase().contains(SCRIPT_EXTN){
            let mut response = Response::default();
            *response.status_mut() = StatusCode::FORBIDDEN;
            return Ok(response);
        }
    }

    match req_path{
        req_path if is_folder =>{
            //process folders
            let default_uri;
            if _last_char(String::from(req_path)).contains(F_SLASH){
                default_uri = format!("{}{}", req_path, props.web_default);
            }else{
                default_uri = format!("{}/{}", req_path, props.web_default);
            }
            
            let request = Request::get(default_uri)
                    .body(())
                    .unwrap();
            return static_.clone().serve(request).await;
        },
        req_path if is_admin =>{
            //process rooster admin
            return admin::process(req_path, props).await;
        },
        req_path if is_script =>{
            //process script
            let method = req.method().clone().to_string();
            let headers = get_headers(req.headers().clone()).await;
            let path = String::from(req_path);

            let mut body = String::from("");
            
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

            if !is_multipart{
                body = get_body_str(req).await;
                let req_data = RequestData::new(
                    method.clone(),
                    path.clone(),
                    query.clone(),
                    headers.clone(),
                    body.clone(),
                    false
                );
                return scripts::process_normal(req_data, props).await;
            }else{
                let req_data = RequestData::new(
                    method.clone(),
                    path.clone(),
                    query.clone(),
                    headers.clone(),
                    body.clone(),
                    true
                );
                return scripts::process_multi_part(req_data, props, req).await;
            } 
        },        
        _ => {
                return static_.clone().serve(req).await;
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

    let body_vec = body_data.map(|v| {
        return v;
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

async fn shutdown_signal() {
    // Wait for the CTRL+C signal
    tokio::signal::ctrl_c()
        .await
        .expect("failed to install CTRL+C signal handler");
}

#[tokio::main]
async fn main() {
    let mut props = props::get_props();
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

    let website_url = format!("http://localhost:{}", net_port);
    let _browser_result = webbrowser::open(&website_url);

    if let Err(e) = graceful.await {
        eprintln!("server error: {}", e);
    }    
}
