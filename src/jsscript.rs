use std::time::SystemTime;
use std::fs;
use std::env;
use std::fs::File;
use std::io::{Error as IoError, Read};
use std::str;

use log::*;

use relative_path::RelativePath;
use hyper::{Body, Response, header::HeaderValue, StatusCode, };
use ducc::{Ducc, ExecSettings, Value, Error as DuccError, Invocation, Object};

use crate::props::Props;
use crate::RequestData;
//use crate::MPFile;

const SCRIPT_EXTN:&str = ".js";
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
    
    let file_name = props.web_root.clone() + &req_data.path.clone() + SCRIPT_EXTN;
    let fslashidx = file_name.as_str().rfind("/").unwrap();
    let script_path = &file_name[..fslashidx];
   
    let mut script_file = match File::open(&file_name) {
        Err(err) => {
            error!("Could not open script file for path: {}, Error - {}", file_name, err);
            let mut response = Response::default();
            *response.status_mut() = StatusCode::NOT_FOUND;
            return Ok(response);
        }
        Ok(f) => f,
    };

    let mut contents = String::new();

    if let Err(err) = script_file.read_to_string(&mut contents) {
        let errmsg = format!("Could not read script file for path: {}, Error - {}", file_name, err);
        error!("{}", errmsg);
        return get_server_error_response(errmsg);
    }

    let engine = Ducc::new();
    //Adding console object
    init_console(&engine);

    //Adding rooster object
    let robj = engine.create_object();
    robj.set("wr", props.web_root.clone()).unwrap();
    robj.set("csp", script_path).unwrap();
    engine.globals().set("_rooster", robj).unwrap();

    //Adding module import
    let requirefn = engine.create_function(require);
    engine.globals().set("require", requirefn).unwrap();
    
    let module = engine.create_object();
    let exports = engine.create_object();
    module.set("exports", exports).unwrap();
    engine.globals().set("module", module).unwrap(); //module.exports
    engine.globals().set("exports", engine.create_object()).unwrap(); //exports
    engine.globals().set("global", engine.globals()).unwrap(); //global reference

    /*
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
    */
    debug!("Request: \r\n{}", get_request_data(req_data.clone()));
    let evalresult:Result<Value, DuccError> = engine.exec(contents.as_str(), Some(&file_name), ExecSettings::default());
    if evalresult.is_ok(){
        let val = evalresult.unwrap();
        let resp_body;
        if val.is_string(){
            resp_body = val.as_string().unwrap().to_string().unwrap();
        }else if val.is_number(){
            resp_body = format!("{}", val.as_number().unwrap());
        }else if val.is_boolean(){
            resp_body = format!("{}", val.as_boolean().unwrap());
        }else if val.is_object(){
            resp_body = format!("{:?}", val.as_object().unwrap());
        }else if val.is_null(){
            resp_body = format!("null");
        }else if val.is_undefined(){
            resp_body = format!("undefined");
        }else{
            resp_body = format!("Unknown value: {:?}", val);
        }

        let mut response = Response::new(Body::from(resp_body.clone()));
        response.headers_mut().insert("Cache-Control", HeaderValue::from_static("no-cache"));
        return Ok(response);
    }else{
        let errmsg = format!("Error loading script file for path: {}, Error - {}", req_data.path, evalresult.unwrap_err());
        error!("{}", errmsg);
        return get_server_error_response(errmsg);
    }
}
/*
fn get_headers(headers:HashMap<String, String>) -> Map{
    let mut headers_map = Map::new();
    for (key, value) in headers{
        headers_map.insert(key.clone(), Dynamic::from(value));
    }
    return headers_map;
}
*/

fn get_request_data(req:RequestData) -> String{
    let mut req_data_str = format!("{} {}?{}\r\n", req.method, req.path, req.query);
    let mut headers_str = String::from("");
    for (key, value) in req.headers.clone().iter(){
        headers_str = format!("{}{}:{}\r\n", headers_str, key, value);
    }
    req_data_str = format!("{}{}\r\n{}\r\n\r\nMultipart: {}", req_data_str, headers_str, req.body, req.is_multipart);
    return req_data_str;
}

fn get_server_error_response(body:String) -> Result<Response<Body>, IoError>{
    return get_err_response(body, StatusCode::from_u16(SERVER_ERROR).unwrap());
}

fn get_err_response(body:String, status:StatusCode) -> Result<Response<Body>, IoError>{
    let mut response = Response::new(Body::from(body.clone()));
    response.headers_mut().insert("Cache-Control", HeaderValue::from_static("no-cache"));
    *response.status_mut() = status;
    return Ok(response);
}

fn init_console(engine:&Ducc){
    let cobj = engine.create_object();
    engine.globals().set("console", cobj).unwrap();
    add_console_log_fn(engine);
    add_console_debug_fn(engine);
    add_console_error_fn(engine);
}

fn add_console_log_fn(engine:&Ducc){
    let fnobj = engine.create_function(|inv| -> Result<Value, DuccError>{
        if inv.args.len() > 0 {
            let v = inv.args.get(0);
            if v.is_boolean(){
                info!("{}", v.as_boolean().unwrap());
            }else if v.is_string(){
                info!("{}", v.as_string().unwrap().to_string().unwrap());
            }else if v.is_number(){
                info!("{}", v.as_number().unwrap());
            }else if v.is_undefined(){
                info!("undefined");
            }else if v.is_null(){
                info!("null");
            }else if v.is_object(){
                info!("{:?}", v.as_object().unwrap());
            }else{
                info!("{:?}", v.as_string());
            }
        }
        Ok(Value::Number(0.0))
    });
    let cobj:Object = engine.globals().get("console").unwrap();
    cobj.set("log", fnobj).unwrap();
}

fn add_console_debug_fn(engine:&Ducc){
    let fnobj = engine.create_function(|inv| -> Result<Value, DuccError>{
        if inv.args.len() > 0 {
            let v = inv.args.get(0);
            if v.is_boolean(){
                debug!("{}", v.as_boolean().unwrap());
            }else if v.is_string(){
                debug!("{}", v.as_string().unwrap().to_string().unwrap());
            }else if v.is_number(){
                debug!("{}", v.as_number().unwrap());
            }else if v.is_undefined(){
                debug!("undefined");
            }else if v.is_null(){
                debug!("null");
            }else if v.is_object(){
                debug!("{:?}", v.as_object().unwrap());
            }else{
                debug!("{:?}", v.as_string());
            }
        }
        Ok(Value::Number(0.0))
    });
    let cobj:Object = engine.globals().get("console").unwrap();
    cobj.set("debug", fnobj).unwrap();
}

fn add_console_error_fn(engine:&Ducc){
    let fnobj = engine.create_function(|inv| -> Result<Value, DuccError>{
        if inv.args.len() > 0 {
            let v = inv.args.get(0);
            if v.is_boolean(){
                error!("{}", v.as_boolean().unwrap());
            }else if v.is_string(){
                error!("{}", v.as_string().unwrap().to_string().unwrap());
            }else if v.is_number(){
                error!("{}", v.as_number().unwrap());
            }else if v.is_undefined(){
                error!("undefined");
            }else if v.is_null(){
                error!("null");
            }else if v.is_object(){
                error!("{:?}", v.as_object().unwrap());
            }else{
                error!("{:?}", v.as_string());
            }
        }
        Ok(Value::Number(0.0))
    });
    let cobj:Object = engine.globals().get("console").unwrap();
    cobj.set("error", fnobj).unwrap();
}

fn require(inv: Invocation) -> Result<Value, DuccError>{
    let stime = SystemTime::now();
    let engine = inv.ducc;
    let arg = &inv.args.get(0).as_string().unwrap().to_string().unwrap();
    info!("Loading script: {}", arg);
    
    let robj:Object = engine.globals().get("_rooster").unwrap();
    
    let webroot:String = robj.get("wr").unwrap();
    let rwpath = RelativePath::new(webroot.as_str());
    let wpath = rwpath.to_path(env::current_dir().unwrap().to_str().unwrap());
    let wpathres = fs::canonicalize(wpath.clone());
    if wpathres.is_err(){
        error!("Web root not found!");
        return Ok(Value::Null);
    }

    let spath:String = robj.get("csp").unwrap();
    let rpath = RelativePath::new(arg.as_str());
    let file_path = rpath.to_path(spath.clone());
    let file_name = file_path.to_str().unwrap();

    let script_path = arg.as_str();
    let abspathres = fs::canonicalize(file_path.clone());
    if abspathres.is_err(){
        error!("Script not found: {}", script_path.clone());
        return Ok(Value::Null);
    }

    let abspath = abspathres.unwrap();
    let absfile = abspath.to_str().unwrap();
    let wfolderpath = wpathres.unwrap();
    let wfolder = wfolderpath.to_str().unwrap();
    let script_path = arg.as_str();

    //Do not permit script references outside the assigned web root folder for security reasons
    if !absfile.contains(wfolder){
        error!("Forbidden access for script: {} in script file for path: {}", script_path, spath.clone());
        return Ok(Value::Null);
    }
    
    let mut script_file = match File::open(file_name) {
        Err(err) => {
            error!("Could not open script file for path: {}, Error - {}", script_path, err);
            return Ok(Value::Null);
        }
        Ok(f) => f,
    };

    let mut contents = String::new();

    if let Err(err) = script_file.read_to_string(&mut contents) {
        error!("Could not read script file for path: {}, Error - {}", file_name, err);
        return Ok(Value::Null);
    }else{
        let fret:Result<(), _> = engine.exec(contents.as_str(), Some(arg), ExecSettings::default());
        if fret.is_err(){
            let err = fret.unwrap_err().context.concat();
            error!("Error loading script file for path: {}, Error - {}", file_name, err);
            return Ok(Value::Null);
        }
        let module = engine.globals().get("module");
        if module.is_ok(){
            let module:Value = module.unwrap();
            let mobj = module.as_object().unwrap();
            let exports = mobj.get("exports");
            if exports.is_ok(){
                debug!("Dependency load time for {} : {} ms", file_name, stime.elapsed().unwrap().as_millis());
                return Ok(exports.unwrap());
            }else{
                error!("Error loading script file for path: {}, Error - No exports found", file_name);
                return Ok(Value::Null);   
            }
        }else{
            error!("Error loading script file for path: {}, Error - Module object not found", file_name);
            return Ok(Value::Null); 
        }
    }
}
