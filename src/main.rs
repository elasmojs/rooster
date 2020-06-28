#![windows_subsystem = "windows"]

use std::clone::Clone;
use std::fs::{metadata};
use std::io::Error as IoError;
use std::{net::SocketAddr, path::Path};

use futures_util::future;

use hyper::{Body, Request, Response, Server, StatusCode};
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


async fn process_request(req: Request<Body>, static_:Static, props: Props) -> Result<Response<Body>, IoError> {
    
    let req_path = req.uri().path();
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
        _req_path if is_script =>{
            //process script call
            return scripts::process(req, props);
        },        
        _ => {
            //Process static files
            return static_.clone().serve(req).await;
        }
    }
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
