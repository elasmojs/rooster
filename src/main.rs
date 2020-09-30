//#![windows_subsystem = "windows"]

use std::time::SystemTime;
use std::clone::Clone;
use std::fs;
use std::fs::{metadata};
use std::io::{Cursor, Read, Error as IoError};
use std::{net::SocketAddr, path::Path};
use std::collections::HashMap;

use std::{str, fmt};
use std::error::Error;
use std::convert::Infallible;
use std::{io, sync};
use std::pin::Pin;

use log::*;

use futures_util::future;
use futures_util::stream::TryStreamExt;

use hyper::{Body, Request, Response, Server, StatusCode, HeaderMap, header::HeaderValue};
use hyper::service::{make_service_fn, service_fn};
use hyper_staticfile::Static;
use hyper::server::conn::AddrStream;

use futures_util::{
    future::TryFutureExt,
    stream::{Stream, StreamExt},
};

use core::task::{Context, Poll};

use tokio::net::{TcpListener, TcpStream};
use tokio_rustls::server::TlsStream;
use tokio_rustls::TlsAcceptor;
use rustls::internal::pemfile;

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
const SAND_BOX:&str = "box";
const GALE_ADMIN_APP:&str = "_gale";

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
    let r_path = uri.path().clone();
    let query = match uri.query().clone(){
        Some(q) => String::from(q),
        None => String::from("")
    };

    let path_split = r_path.clone().split(F_SLASH);
    let path_vec = path_split.clone().collect::<Vec<&str>>();
    let path_str = path_vec.clone().join(F_SLASH);

    //Set default app
    let req_path:String;
    let script_path:String;
    let mut app_name:String = String::from(*path_vec.get(1).unwrap());
    if app_name.len() == 0 || app_name.last_index_of(DOT) != -1{
        app_name = format!("{}", props.default_app);
        req_path = format!("/{}{}", props.default_app.clone(), r_path.clone());
        script_path = format!("{}/{}/{}{}", props.web_root, props.default_app.clone(), SAND_BOX, r_path.clone());
    }else{
        let uri_str = path_str.replace(format!("/{}", app_name).as_str(), "");
        if props.is_app(app_name.clone()){
            req_path = String::from(path_str.clone());
            script_path = format!("{}/{}/{}{}", props.web_root, app_name.clone(), SAND_BOX, uri_str.clone());
        }else{
            app_name = format!("{}", props.default_app);
            req_path = format!("/{}{}", props.default_app.clone(), r_path.clone());
            script_path = format!("{}/{}/{}{}", props.web_root, props.default_app.clone(), SAND_BOX, r_path.clone());
        }
    }
    let f_path = format!("{}{}", props.web_root.clone(), req_path.clone());
    
    let file_name = *path_vec.clone().last().unwrap();
    //let base_path = f_path.replace(file_name, "");
    let not_file = String::from(file_name).last_index_of(DOT) == -1;
    //let mut is_rhai_script = false;
    let mut is_js_script = false;
    let mut is_folder = false;
    let mut is_admin = false; 

    if not_file{
        match metadata(f_path.clone()){
            Ok(md) => {
                if md.is_dir(){
                    is_folder = true;
                }
            }
            Err(_e) => {
                if req_path.clone().contains(ADMIN_ROUTE){
                    is_admin = true;
                }else if Path::new(&(script_path.clone() + JS_SCRIPT_EXTN)).exists(){
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
                default_uri = format!("{}/", r_path.clone());
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
            info!("Serving admin request for path: {}", req_path.clone());
            let admin_resp = admin::process(req_path.clone().as_str(), props).await;
            debug!("Time taken to serve {}: {} ms", req_path, stime.elapsed().unwrap().as_millis());
            return admin_resp;
        },
        _req_path if is_js_script =>{
            //process js script
            let path = format!("{}", r_path);
            info!("Serving script request for path: {}", path);
            let method = req.method().clone().to_string();
            let headers = get_headers(req.headers().clone()).await;

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
            let script_resp = jsscript::process(req_data, props, script_path.clone(), app_name.clone(), String::from(file_name.clone())).await;
            debug!("Time taken to serve {}: {} ms", path, stime.elapsed().unwrap().as_millis());
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
                let forbidden = format!("/{}/", SAND_BOX);
                if req_path.to_lowercase().contains(forbidden.as_str()){
                    warn!("Forbidden sandbox access from {} for {}", props.remote_addr, req_path);
                    let mut response = Response::default();
                    *response.status_mut() = StatusCode::NOT_FOUND;
                    return Ok(response);
                }
                let request = Request::get(req_path.clone())
                    .body(())
                    .unwrap();
                let static_resp = static_.clone().serve(request).await;
                debug!("Time taken to serve {}: {} ms", req_path.clone(), stime.elapsed().unwrap().as_millis());
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

fn get_app_names(root:String) -> Vec<String>{
    let dir_res = fs::read_dir(root.clone());
    let mut name_vec:Vec<String> = Vec::new();
    name_vec.push(String::from(GALE_ADMIN_APP));
    if dir_res.is_ok(){
        let dir = dir_res.unwrap();
        for entry_res in dir{
            if entry_res.is_ok(){
                let entry = entry_res.unwrap();
                if entry.metadata().unwrap().is_dir(){
                    let is_hidden = entry.file_name().to_str().unwrap().starts_with(".");
                    if !is_hidden{
                        name_vec.push(String::from(entry.file_name().to_str().unwrap()));
                    }
                }
            }
        }
    }
    return name_vec;
}


fn main() {
    let props = props::get_props();
    logger::init_log(props.clone());

    if props.net_http_enabled{
        std::thread::spawn(||{
            init_http();
        });
        let website_url = format!("http://localhost:{}", props.net_port);
        let _browser_result = webbrowser::open(&website_url);
    }else{
        info!("HTTP service is not enabled, check 'net.http.enabled' property in gale.cfg");
    }
    
    if props.net_ssl_enabled{
        std::thread::spawn(||{
            init_ssl();
        });
    }else{
        info!("HTTPS service is not enabled, to use set 'net.ssl.enabled' to 'true' in gale.cfg");
    }

    if props.net_http_enabled || props.net_ssl_enabled{
        loop{
            std::thread::sleep(std::time::Duration::from_millis(3000));
        }
    }
}

#[tokio::main]
async fn init_http(){
    let mut props = props::get_props();
    props.apps = get_app_names(props.web_root.clone());

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
    let start_msg = format!("Gale JS server started at => http://localhost:{}", net_port);
    info!("{}", start_msg);
    println!("{}", start_msg);

    if let Err(e) = graceful.await {
        eprintln!("server error: {}", e);
    } 
}

#[tokio::main]
async fn init_ssl(){
    let mut props = props::get_props();
    props.apps = get_app_names(props.web_root.clone());
    let net_ssl_port = props.net_ssl_port.clone();
    let net_ssl_cert = props.net_ssl_cert.clone();
    let net_ssl_pkey = props.net_ssl_pkey.clone();

    let static_ = Static::new(Path::new(&props.web_root));

    let service = make_service_fn(|conn:&TlsStream<TcpStream>| {
        let (stream, _session) = conn.get_ref();
        let remote_res = stream.peer_addr();
        if remote_res.is_ok(){
            let socket_addr = remote_res.unwrap();
            props.remote_addr = socket_addr.ip().to_string();
        }else{
            props.remote_addr = "".to_string();
        }
        let static_ = static_.clone();
        let props_:Props = props.clone();
        async { Ok::<_, Infallible>(service_fn(move |req| process_request(req, static_.clone(), props_.clone()))) }
    });

    
    let addr = format!("0.0.0.0:{}", net_ssl_port);
    
    // Build TLS configuration.
    let tls_cfg_opt = {
        // Load public certificate.
        let certs_res = load_certs(net_ssl_cert.as_str());
        if certs_res.is_err(){
            error!("Could not load ssl certificate: {}, error: {}", net_ssl_cert, certs_res.unwrap_err());
            None
        }else{
            let certs = certs_res.unwrap();

            // Load private key.
            let key_res = load_private_key(net_ssl_pkey.as_str());
            if key_res.is_err(){
                error!("Could not load ssl pkey: {}, error: {}", net_ssl_pkey, key_res.unwrap_err());
                None
            }else{
                let key = key_res.unwrap();

                // Do not use client certificate authentication.
                let mut cfg = rustls::ServerConfig::new(rustls::NoClientAuth::new());
                // Select a certificate to use.
                let res = cfg.set_single_cert(certs, key)
                    .map_err(|e| error(format!("{}", e)));
                if res.is_err(){
                    error!("Could not init ssl certificate, error: {}", res.unwrap_err());
                    None
                }else{
                    // Configure ALPN to accept HTTP/2, HTTP/1.1 in that order.
                    cfg.set_protocols(&[b"h2".to_vec(), b"http/1.1".to_vec()]);
                    Some(sync::Arc::new(cfg))
                }
            }                
        }
    };
    if tls_cfg_opt.is_none(){
        error!("Could not start the Gale server for SSL");
        println!("Could not start the Gale server for SSL, please check logs!");
        return;
    }
    let tls_cfg = tls_cfg_opt.unwrap();

    // Create a TCP listener via tokio.
    let tcp_res = TcpListener::bind(&addr).await;
    if tcp_res.is_err(){
        error!("Could not bind TCP socket, error: {}", tcp_res.unwrap_err());
        return;
    }
    let mut tcp = tcp_res.unwrap();
    let tls_acceptor = TlsAcceptor::from(tls_cfg);
    // Prepare a long-running future stream to accept and serve cients.
    
    let incoming_tls_stream = tcp
        .incoming()
        .map_err(|err|{
            warn!("SSL Client Incoming Error: {}", err);
            SSLError::from(err)
        })
        .and_then(move |s|{
            tls_acceptor.accept(s).map_err(|err|{
                warn!("SSL Client Accept Error: {}", err);
                SSLError::from(err)
            })
        })
        .boxed();
    
    let server = Server::builder(HyperAcceptor {
        acceptor: incoming_tls_stream,
    })
    .serve(service);

    let start_msg = format!("Gale JS server started at => https://localhost:{}", net_ssl_port);
    info!("{}", start_msg);
    println!("{}", start_msg);

    
    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}

struct HyperAcceptor<'a> {
    acceptor: Pin<Box<dyn Stream<Item = Result<TlsStream<TcpStream>, SSLError>> + 'a>>,
}

impl hyper::server::accept::Accept for HyperAcceptor<'_> {
    type Conn = TlsStream<TcpStream>;
    type Error = SSLError;

    fn poll_accept(
        mut self: Pin<&mut Self>,
        cx: &mut Context,
    ) -> Poll<Option<Result<Self::Conn, Self::Error>>> {
        let poll_result = Pin::new(&mut self.acceptor).poll_next(cx);
        match poll_result{
            Poll::Ready(prslt) => {
                match prslt{
                    Some(res) => {
                        if res.is_err(){
                            warn!("SSL Client Error: {}", res.unwrap_err());
                            Pin::new(&mut self.acceptor).poll_next(cx)
                        }else{
                            Poll::Ready(Some(res))
                        }
                    },
                    None => Poll::Ready(None)
                }
            },
            Poll::Pending => Poll::Pending
        }
    }
}

struct SSLError{
    kind: String,
    message: String
}

impl fmt::Display for SSLError{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        write!(f, "{}", self.message)
    }
}

impl fmt::Debug for SSLError{
    fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result{
        write!(f, "Kind: {}, message: {}", self.kind, self.message)
    }
}

impl Error for SSLError{
}

impl From<io::Error> for SSLError{
    fn from(error: io::Error) -> Self{
        SSLError{
            kind: String::from("IO Error"),
            message: error.to_string()
        }
    }
}


fn error(err: String) -> io::Error {
    io::Error::new(io::ErrorKind::Other, err)
}

// Load public certificate from file.
fn load_certs(filename: &str) -> io::Result<Vec<rustls::Certificate>> {
    // Open certificate file.
    let certfile = fs::File::open(filename)
        .map_err(|e| error(format!("failed to open {}: {}", filename, e)))?;
    let mut reader = io::BufReader::new(certfile);

    // Load and return certificate.
    pemfile::certs(&mut reader).map_err(|_| error("failed to load certificate".into()))
}

// Load private key from file.
fn load_private_key(filename: &str) -> io::Result<rustls::PrivateKey> {
    // Open keyfile.
    let keyfile = fs::File::open(filename)
        .map_err(|e| error(format!("failed to open {}: {}", filename, e)))?;
    let mut reader = io::BufReader::new(keyfile);

    // Load and return a single private key.
    let keys = pemfile::pkcs8_private_keys(&mut reader)
        .map_err(|_| error("failed to load private key".into()))?;
    
    if keys.len() != 1 {
        return Err(error("expected a single private key".into()));
    }
    Ok(keys[0].clone())
}
