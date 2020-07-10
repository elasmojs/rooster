use std::fs::File;
use std::io::{Error as IoError, Read};
use std::str;
use std::collections::HashMap;

use log::*;

use hyper::{Body, Response, header::HeaderValue, StatusCode, };

use rhai::{Engine, OptimizationLevel, Scope, Dynamic, Map};

use crate::props::Props;
use crate::RequestData;
//use crate::MPFile;

const SCRIPT_EXTN:&str = ".rhai";
const SCRIPT_MAIN:&str = "main";
const SERVER_ERROR:u16 = 500;

pub async fn process(req_data:RequestData, props:Props) -> Result<Response<Body>, IoError>{
    //TODO: process script

    if req_data.is_multipart{
        for (key, val) in req_data.fields.clone(){
            info!("Field: {} - {}", key, val);
        }
        for (key, val) in req_data.files.clone(){
            info!("File: {} - {} - {}", key, val.file_name, val.content_type);
        }
    }
    
    let mut script_file = match File::open(&(props.web_root.clone() + &req_data.path.clone() + SCRIPT_EXTN)) {
        Err(err) => {
            error!("Could not open script file for path: {}, Error - {}", req_data.path, err);
            let mut response = Response::default();
            *response.status_mut() = StatusCode::NOT_FOUND;
            return Ok(response);
        }
        Ok(f) => f,
    };

    let mut contents = String::new();

    if let Err(err) = script_file.read_to_string(&mut contents) {
        error!("Could not read script file for path: {}, Error - {}", req_data.path, err);
        return get_server_error_response();
    }

    let mut engine = Engine::new();

    #[cfg(not(feature = "no_optimize"))]
    engine.set_optimization_level(OptimizationLevel::Full);
    engine.set_max_expr_depths(props.max_expr_depths_global, props.max_expr_depths_local);
    engine.set_max_call_levels(props.max_call_levels);
    engine.set_max_modules(props.max_modules);
    engine.set_max_map_size(props.max_map_size);
    engine.set_max_array_size(props.max_array_size);
    engine.set_max_string_size(props.max_string_size);

    let ast = engine.compile(&contents);
    if ast.is_err() {
        error!("Could not compile script file for path: {}, Error - {}", req_data.path, ast.err().unwrap());
        return get_server_error_response()
    }

    let mut request_map = Map::new();

    let headers_map = get_headers(req_data.headers.clone());
    
    request_map.insert(String::from("remote_addr"), Dynamic::from(props.remote_addr.clone()));
    request_map.insert(String::from("method"), Dynamic::from(req_data.method.clone()));
    request_map.insert(String::from("path"), Dynamic::from(req_data.path.clone()));
    request_map.insert(String::from("query"), Dynamic::from(req_data.query.clone()));
    request_map.insert(String::from("headers"), Dynamic::from(headers_map.clone()));
    request_map.insert(String::from("body"), Dynamic::from(req_data.body.clone()));
    request_map.insert(String::from("is_multipart"), Dynamic::from(req_data.is_multipart));
    
    let mut scope = Scope::new();
    scope.push("request", request_map.clone());
    
    debug!("Request: \r\n{}", get_request_data(req_data.clone()));
    let result:Dynamic = match engine.call_fn(&mut scope, &(ast.unwrap()), SCRIPT_MAIN, () ){
        Ok(result) => {
            result
        },
        Err(err) => {
            error!("Error in script processing for path: {}, Error - {}", req_data.path, err);
            return get_server_error_response()
        }
    };

    let result_str:String = match result.try_cast::<String>(){
        Some(result_str) => {
            result_str
        },
        None => {
            error!("Unexpected response from script file for path: {}", req_data.path);
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

fn get_request_data(req:RequestData) -> String{
    let mut req_data_str = format!("{} {}?{}\r\n", req.method, req.path, req.query);
    let mut headers_str = String::from("");
    for (key, value) in req.headers.clone().iter(){
        headers_str = format!("{}{}:{}\r\n", headers_str, key, value);
    }
    req_data_str = format!("{}{}\r\n{}\r\n\r\nMultipart: {}", req_data_str, headers_str, req.body, req.is_multipart);
    return req_data_str;
}

fn get_server_error_response() -> Result<Response<Body>, IoError>{
    return get_err_response(StatusCode::from_u16(SERVER_ERROR).unwrap());
}

fn get_err_response(status:StatusCode) -> Result<Response<Body>, IoError>{
    let mut response = Response::default();
    *response.status_mut() = status;
    return Ok(response);
}