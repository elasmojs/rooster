use std::fs::File;
use std::io::{Error as IoError, Read};

use hyper::{Body, Request, Response, header::HeaderValue, StatusCode};

use rhai::{Engine, OptimizationLevel, Scope, Dynamic, Map};

use crate::props::Props;

const SCRIPT_EXTN:&str = ".rhai";
const SCRIPT_MAIN:&str = "main";
const SERVER_ERROR:u16 = 500;

pub fn process(req: Request<Body>, props: Props) -> Result<Response<Body>, IoError>{
    //TODO: process script
    
    let req_path = req.uri().path();
    
    let mut script_file = match File::open(&(props.web_root.clone() + req_path + SCRIPT_EXTN)) {
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

    /*
    TODO 
    define custom type to pass request details and other necessary properties
    define script api functions
    enable script api option auto response
  
    E.g. of custom type registration below
    engine.register_type::<Props>();
    engine.register_get("net_port", Props::get_net_port);
    engine.register_get("web_root", Props::get_net_port);
    engine.register_get("web_default", Props::get_net_port);
    engine.register_get("remote_addr", Props::get_net_port);
    */

    let mut header_keys = req.headers().keys();
    let mut headers_map = Map::new();
    for x in 0..header_keys.len() {
        let key:String = match header_keys.nth(x){
            Some(k) => k.to_string(),
            None => String::from("")
        };
        let key_ = key.clone();
        if key.len() > 0 {
            let value:String = match req.headers().get(key){
                Some(v) => String::from(v.to_str().unwrap()),
                None => String::from("")
            };
            headers_map.insert(key_.clone(), Dynamic::from(value));
        }
    }

    let mut scope = Scope::new();
    scope.push("headers", headers_map);
    
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

fn get_server_error_response() -> Result<Response<Body>, IoError>{
    return get_err_response(StatusCode::from_u16(SERVER_ERROR).unwrap());
}

fn get_err_response(status:StatusCode) -> Result<Response<Body>, IoError>{
    let mut response = Response::default();
    *response.status_mut() = status;
    return Ok(response);
}