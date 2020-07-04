use std::fs::File;
use std::io::{Error as IoError, Read};
use std::str;
use std::collections::HashMap;

use hyper::{Body, Request, Response, header::HeaderValue, StatusCode, };

use rhai::{Engine, OptimizationLevel, Scope, Dynamic, Map};

use crate::props::Props;
use crate::RequestData;

const SCRIPT_EXTN:&str = ".rhai";
const SCRIPT_MAIN:&str = "main";
const SERVER_ERROR:u16 = 500;

pub async fn process_multi_part(req_data:RequestData, props: Props, _req:Request<Body>) -> Result<Response<Body>, IoError>{
    return process(req_data, props).await;
}

pub async fn process_normal(req_data:RequestData, props: Props) -> Result<Response<Body>, IoError>{
    return process(req_data, props).await;
}

async fn process(req_data:RequestData, props:Props) -> Result<Response<Body>, IoError>{
    //TODO: process script
    
    let mut script_file = match File::open(&(props.web_root.clone() + &req_data.path.clone() + SCRIPT_EXTN)) {
        Err(_err) => {
            let mut response = Response::default();
            *response.status_mut() = StatusCode::NOT_FOUND;
            return Ok(response);
        }
        Ok(f) => f,
    };

    let mut contents = String::new();

    if let Err(_err) = script_file.read_to_string(&mut contents) {
        println!("{:?}", _err);
        return get_server_error_response();
    }

    let mut engine = Engine::new();

    #[cfg(not(feature = "no_optimize"))]
    engine.set_optimization_level(OptimizationLevel::Full);
    engine.set_max_expr_depths(500, 500);
    engine.set_max_call_levels(500);
    engine.set_max_modules(1000);
    engine.set_max_map_size(1500);
    engine.set_max_array_size(1500);
    engine.set_max_string_size(5000);
    
    
    let ast = engine.compile(&contents);
    if ast.is_err() {
        println!("{:?}", ast.unwrap_err());
        return get_server_error_response()
    }

    let mut request_map = Map::new();

    let headers_map = get_headers(req_data.headers);
    
    request_map.insert(String::from("method"), Dynamic::from(req_data.method.clone()));
    request_map.insert(String::from("path"), Dynamic::from(req_data.path.clone()));
    request_map.insert(String::from("query"), Dynamic::from(req_data.query.clone()));
    request_map.insert(String::from("headers"), Dynamic::from(headers_map.clone()));
    request_map.insert(String::from("body"), Dynamic::from(req_data.body.clone()));
    request_map.insert(String::from("is_multipart"), Dynamic::from(req_data.is_multipart));
    
    let mut scope = Scope::new();
    scope.push("request", request_map);
    
    let result:Dynamic = match engine.call_fn(&mut scope, &(ast.unwrap()), SCRIPT_MAIN, () ){
        Ok(result) => {
            result
        },
        Err(_err) => {
            println!("{:?}", _err);
            return get_server_error_response()
        }
    };

    let result_ = result.clone();
    let result_str:String = match result.try_cast::<String>(){
        Some(result_str) => {
            result_str
        },
        None => {
            println!("Error in return value: {:?}", result_);
            return get_server_error_response()
        }
    };
    
    let mut response = Response::new(Body::from(result_str));
    response.headers_mut().insert("Cache-Control", HeaderValue::from_static("no-cache"));
    return Ok(response);
}

fn get_headers(headers:HashMap<String, String>) -> Map{
    let mut headers_map = Map::new();
    for (key, value) in headers{
        headers_map.insert(key.clone(), Dynamic::from(value));
    }
    return headers_map;
}

fn get_server_error_response() -> Result<Response<Body>, IoError>{
    return get_err_response(StatusCode::from_u16(SERVER_ERROR).unwrap());
}

fn get_err_response(status:StatusCode) -> Result<Response<Body>, IoError>{
    let mut response = Response::default();
    *response.status_mut() = status;
    return Ok(response);
}