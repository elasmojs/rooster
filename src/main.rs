#![windows_subsystem = "windows"]

use std::io::Error as IoError;
use std::thread;
use std::time::Duration;
use std::{net::SocketAddr, path::Path, process::exit};

use futures_util::future;

use hyper::{Body, Request, Response, Server, header::HeaderValue};
use hyper::service::{make_service_fn, service_fn};
use hyper_staticfile::Static;

mod props;

async fn process_request(req: Request<Body>, static_:Static, web_default: &str) -> Result<Response<Body>, IoError> {
    let default_uri = format!("/{}", web_default);
    
    match req.uri().path() {
        "/" =>{
            let request = Request::get(default_uri)
                .body(())
                .unwrap();
            static_.clone().serve(request).await
        },
        "/_rooster/shutdown" =>{
            let shutdown_resp = "<html><body><img style=\"vertical-align: middle;\" href=\"http://icons8.com\" src=\"data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAFRklEQVR42t2bf8hfUxzHP4vaI9bItNWUp0wjtClla5NnRU0pClGspmgrK4RStvaVFX8oz5plsuVZEUKmiGLtUYSiyFb7g3oUoaxW82QrK59Xn3M913Xv/d4f55x7v3vXu74/7j3f83nfcz7n8/mc850n9bFcuUz5QYN7e4d5De55UjlQ3jZEhEPKHcqDBd8j5Jjyu1ET4BXlRuWfylXKIwXX/apcpLxb+U7O9x8qb1Q+rNxd0MZ8MRH3KH/smwDgByfCsZzrGAETyr+VW5QvZb7/XnmVe40IO3PauEv5hmvjObGRd6pPAoDPlOuVs0Ouw5CtYk9yhfLbzPV7nFCn3fuzlNPKtalrjoqNKG/TpokATztD0sgT4XXX2Sx+Vl5c0DYGTok98XuVK3OuYeptVr7WlQCbxJ5WFofFnuCnyiudKOf76GQB8A3buhDgOuWXJd8fD2x4GpPKR2ILcK4z8uxIRg5Dq5HQRADwhZj37wvwNW/GFIBgaEfXVqfAiMRh/hRLgEvFYoA+4W3lnbEEAERy67u2OgNGQa0YoY0AN4gFKn1C7VWhjQDgXbGkqC8gFrk6pgBLlV8rl3RtucNJ5TkxBQBrlB8pz+vaerHV4ILYAgCiwwPS/UiYVq7rQgBwofJZsSRmrOQ6ssAlEkYscpHddW7wKUBaiFvEIkVKZ4TMv4nlD3Ag1ZbP951YtLWywvWH3W9eLnPTkcyztJASQoAikCEecKIMA+luuoDysvL+IfcQmJFmZ0cfKTbFlH1dCnCr8lWp5igx5LLMZ4yqP1r2gYIM0/N0+sMYAmwXG/ZVwTRZnfmM2uBJD33hIWyIJQCd3iumeh1QDWLOp4utRUWYKmAK4INm3HuSuH/9QigBFovN96YpMx0euM5TOX5CqtcfmEIkRsQmBGmzZReHEGC5+/HxAG2XAcFxdp/Xucm3AGtcRxZFNp4IkNL6/ro3+hTgdjEnM9a2oRbA07Nczla9wZcAD4kNvz7UCalG4zcqbaD4EADjJ7u2OoMp5X0xBKi7xsfEHZK/J+lNgOfFHE8ssDTmJVDkCy+IrTrsNbKVhhNmCb0ilADPiK3NsYB/ofSdt6V23Bmc3lPEB2x1fSxdFpsIEHvOb3GGlwm+tsBQArLffQpAUkOUFcvbYzQh8IyUb7cNlE81+YE6AhDhEVrGKn0ldf4HxeZ4GYg8bw4pAPOKLO3aSMYzrylsMHw/FpvTZWDLfEFIAXj6RyMZD3B6j7vXf0m16HJcAm+NnZB4w58s8ivlJTKXxg7DhNjZhGACvCUWXMQAT/xU3wS4RyzZiYGkXwvFSmFVVp1xCTwFOBhBNBZjGrDu/+JeVxl5RIPXNPmhunFArPCXMlpyCGqxzO0l5IESGqtE7eHfRAA6Q609dCA0Lf/d4WEPkuk3kbmOlYlI8WClVj0IAF4Uq9uHRnoUJOBgBrEIT53aX+vzgk0EwDHNSPiTYMlZwUZnf0IKANqUqeuCMhd7jkEOVY/SERmGPLkIUeI3fRAAh8iGZOwKcBIldi4A4JzQJxK3GIrgx1q34kkAENMfEBVe5LNBX2XxWAESo+2mPgoAdokFJSHR+nB0SAEomlC5CRkk5QVHvREgQcjpQNn7SOtWAgsAcIyMBp+rQ+0zgF0KAFgiKWz6ihPY87t+lAQAZHFTMryoWQXpOuHICJDgUbGjKW22zjmT/N6oCgCo701K88PVXiPALgRIgG8YyP+LG2XIOzo3sgKkhXhMLKMctlpw2uyBM02ABDjKjWJTo2jnqfGfokZBgDRIsVc5josVQhkdeX/N9YJ/AKlt8UF5Z0KmAAAAAElFTkSuQmCC\" alt=\"\" /><span>Cock-a-doodle-doo! Bye bye!</span></body></html>";
            let mut response = Response::new(Body::from(shutdown_resp.as_bytes()));
            response.headers_mut().insert("Cache-Control", HeaderValue::from_static("no-cache"));
            exit_rooster().await;
            Ok(response)
        },
        _ => {
            static_.clone().serve(req).await
        }
    }
}

async fn shutdown_signal() {
    // Wait for the CTRL+C signal
    tokio::signal::ctrl_c()
        .await
        .expect("failed to install CTRL+C signal handler");
}

async fn exit_rooster(){
    thread::spawn(|| {
        thread::sleep(Duration::from_millis(100));
        exit(0);
    });
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
