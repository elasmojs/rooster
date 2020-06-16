#![windows_subsystem = "windows"]

use std::io::Error as IoError;
use std::{net::SocketAddr, path::Path};

use futures_util::future;

use hyper::{Body, Request, Response, Server};
use hyper::service::{make_service_fn, service_fn};
use hyper_staticfile::Static;

mod props;
mod admin;

async fn process_request(req: Request<Body>, static_:Static, web_default: &str) -> Result<Response<Body>, IoError> {
    let default_uri = format!("/{}", web_default);
    let req_path = req.uri().path();
    match req_path {
        "/" =>{
            let request = Request::get(default_uri)
                .body(())
                .unwrap();
            return static_.clone().serve(request).await;
        },
        req_path if req_path.contains("/_rooster") =>{
            return admin::process(req_path).await;
        },
        _ => {
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
    let port_num = props::get_port();
    let web_root = props::get_web_root();
    let web_default = props::get_web_default();

    let static_ = Static::new(Path::new(web_root));

    let service = make_service_fn(|_| {
        let static_ = static_.clone();
        future::ok::<_, hyper::Error>(service_fn(move |req| process_request(req, static_.clone(), web_default)))
    });

    let addr = SocketAddr::from(([127, 0, 0, 1], port_num));
    let server = Server::bind(&addr).serve(service);
    let graceful = server.with_graceful_shutdown(shutdown_signal());

    let website_url = format!("http://localhost:{}", port_num);
    let _browser_result = webbrowser::open(&website_url);

    if let Err(e) = graceful.await {
        eprintln!("server error: {}", e);
    }    
}
