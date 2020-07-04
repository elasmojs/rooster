use std::thread;
use std::time::Duration;
use std::process::exit;
use std::io::Error as IoError;

use::log::*;

use hyper::{Response, Body, header::HeaderValue, StatusCode};

use crate::resources;
use crate::props::Props;

pub async fn process(req_path: &str, props: Props) -> Result<Response<Body>, IoError>{
    match req_path{
        "/_rooster/shutdown" => {
            //Allow shutdown only when issued from the localhost
            info!("Serving shut down request");
            if !props.remote_addr.contains("127.0.0.1"){
                error!("Illegal shutdown request from: {}", props.remote_addr);
                let mut response = Response::default();
                *response.status_mut() = StatusCode::FORBIDDEN;
                return Ok(response);
            }else{
                info!("Shutting down...bye! bye!");
                return shutdown().await;
            }
        },
        "/_rooster/about" => {
            info!("Serving about request");
            return about().await;
        },
        _ => {
            error!("Illegal request to admin route: {}", req_path);
            let mut response = Response::default();
            *response.status_mut() = StatusCode::NOT_FOUND;
            return Ok(response);
        }
    }
}

async fn shutdown() -> Result<Response<Body>, IoError>{
    let shutdown_resp = resources::get("shutdown.html");
    let mut response = Response::new(Body::from(shutdown_resp));
    response.headers_mut().insert("Cache-Control", HeaderValue::from_static("no-cache"));
    exit_rooster().await;
    return Ok(response);
}

async fn about() -> Result<Response<Body>, IoError>{
    let about_resp = resources::get("about.html");
    let response = Response::new(Body::from(about_resp.as_bytes()));
    //response.headers_mut().insert("Cache-Control", HeaderValue::from_static("no-cache"));
    return Ok(response);
}

async fn exit_rooster(){
    thread::spawn(|| {
        thread::sleep(Duration::from_millis(100));
        exit(0);
    });
}
