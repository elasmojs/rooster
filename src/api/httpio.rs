use ducc::{Ducc, Object, Invocation, Value, Elements, Error as DuccError};
use std::collections::HashMap;
use std::path::Path;
use log::*;

mod httpio;

use self::httpio::*;

pub const API_KEY:&str = "api";
pub const GALE_KEY:&str = "_gale";
pub const WEB_ROOT_KEY:&str = "wr"; 
pub const APP_KEY:&str = "app";
pub const BOX:&str = "box";
pub const HTTP_API:&str = "http";

pub fn load(engine:&Ducc) -> bool{
    let api_res:Result<Value, _> = engine.globals().get(API_KEY);
    let api_obj = api_res.unwrap();                
    let api = api_obj.as_object().unwrap();

    let httpio = engine.create_object();
    httpio.set("get", engine.create_function(http_get)).unwrap();
    httpio.set("post", engine.create_function(http_post)).unwrap();
    httpio.set("postMultiPart", engine.create_function(http_post_multipart)).unwrap();

    api.set(HTTP_API, httpio).unwrap();

    return true;
}

fn get_headers(hobj:Object) -> HashMap<String, String>{
    let mut headers:HashMap<String, String> = HashMap::new();
    for entry_res in hobj.properties(){
        if entry_res.is_ok(){
            let tuple = entry_res.unwrap();
            headers.insert(tuple.0, tuple.1);
        }
    }
    return headers;
}

pub fn http_get(inv: Invocation) -> Result<Value, DuccError>{
    let engine = inv.ducc;
    let args = inv.args;
    if args.len() == 2{
        let url_res = args.get(0);
        let headers_res = args.get(1);

        if url_res.is_string() && headers_res.is_object(){
            let url = url_res.as_string().unwrap().to_string().unwrap();
            let hobj = headers_res.as_object().unwrap().clone();
            let headers = get_headers(hobj);
                        
            let req = Request::new(url, headers);
            let resp = Client::get(req);

            let robj = engine.create_object();
            robj.set("code", resp.code).unwrap();
            
            let rs_headers = engine.create_object();
            for (key, value) in resp.headers{
                rs_headers.set(engine.create_string(key.as_str()).unwrap(), value).unwrap();
            }
            robj.set("headers", rs_headers).unwrap();

            match std::str::from_utf8(&resp.body){
                Ok(body) => robj.set("body", body).unwrap(),
                Err(_e) => robj.set("body", engine.create_bytes(&resp.body).unwrap()).unwrap()
            };
            
            return Ok(Value::Object(robj));
        }else{
            error!("Invalid argument for http get, expected string");
            return Ok(Value::Null);    
        }
    }else{
        error!("Invalid argument for http get, expected 2 arguments");
        return Ok(Value::Null);
    }
}

pub fn http_post(inv: Invocation) -> Result<Value, DuccError>{
    let engine = inv.ducc;
    let args = inv.args;
    if args.len() == 3{
        let url_res = args.get(0);
        let headers_res = args.get(1);
        let body_res = args.get(2);

        if url_res.is_string() && headers_res.is_object() && body_res.is_string(){
            let url = url_res.as_string().unwrap().to_string().unwrap();
            let hobj = headers_res.as_object().unwrap().clone();
            let headers = get_headers(hobj);
            let body_str = body_res.as_string().unwrap().to_string().unwrap();
                        
            let mut req = Request::new(url, headers);
            let mut body = Body::default();
            body.body = Some(Vec::from(body_str.as_bytes()));
            req.body = Some(body);

            let resp = Client::post(req);

            let robj = engine.create_object();
            robj.set("code", resp.code).unwrap();
            
            let rs_headers = engine.create_object();
            for (key, value) in resp.headers{
                rs_headers.set(engine.create_string(key.as_str()).unwrap(), value).unwrap();
            }
            robj.set("headers", rs_headers).unwrap();

            match std::str::from_utf8(&resp.body){
                Ok(body) => robj.set("body", body).unwrap(),
                Err(_e) => robj.set("body", engine.create_bytes(&resp.body).unwrap()).unwrap()
            };
            
            return Ok(Value::Object(robj));
        }else{
            error!("Invalid argument for zip create, expected string");
            return Ok(Value::Null);    
        }
    }else{
        error!("Invalid argument for zip create, expected 2 arguments");
        return Ok(Value::Null);
    }
}

pub fn http_post_multipart(inv: Invocation) -> Result<Value, DuccError>{
    let engine = inv.ducc;
    let args = inv.args;
    if args.len() == 3{
        let robj:Object = engine.globals().get(GALE_KEY).unwrap();
        let web_root:String = robj.get(WEB_ROOT_KEY).unwrap();
        let app_name:String = robj.get(APP_KEY).unwrap();

        let url_res = args.get(0);
        let headers_res = args.get(1);
        let parts_res = args.get(2);

        if url_res.is_string() && headers_res.is_object() && parts_res.is_array(){
            let url = url_res.as_string().unwrap().to_string().unwrap();
            let hobj = headers_res.as_object().unwrap().clone();
            let headers = get_headers(hobj);
            let parr = parts_res.as_array().unwrap().clone();
            
            let mut req = Request::new(url, headers);
            let mut body = Body::default();
            body.is_multipart = true;
            let mut mp_body = req.get_multipart_body();
            let mut res = Ok(&mut mp_body);

            let parts:Elements<Value> = parr.elements();
            for parts_res in parts{
                if parts_res.is_ok(){
                    let partobj:Value = parts_res.unwrap();
                    if partobj.is_object(){
                        let part = partobj.as_object().unwrap().clone();
                        if part.contains_key("type").is_ok() && 
                        part.contains_key("name").is_ok() && 
                        part.contains_key("value").is_ok(){
                            let ptype:Value = part.get("type").unwrap();
                            let name:Value = part.get("name").unwrap();
                            let value:Value = part.get("value").unwrap();
                            
                            if ptype.is_string() && name.is_string() && value.is_string(){
                                if ptype.as_string().unwrap().to_string().unwrap() == "text"{
                                    //Add text part
                                    res = res.unwrap().write_text(name.as_string().unwrap().to_string().unwrap(), value.as_string().unwrap().to_string().unwrap());
                                }else{
                                    //Add file part
                                    let fpath = format!("{}/{}/{}/{}", web_root, app_name, BOX, value.as_string().unwrap().to_string().unwrap());
                                    res = res.unwrap().write_file(name.as_string().unwrap().to_string().unwrap(), Path::new(fpath.as_str()));
                                }
                                if res.is_err(){
                                    error!("Could not write part ({}) in HTTP multipart request", name.as_string().unwrap().to_string().unwrap());
                                }
                            }else{
                                error!("Invalid input for part in HTTP multipart request");
                            }
                        }else{
                            error!("Could not get part in HTTP multipart request");    
                        }
                    }else{
                        error!("Could not get part in HTTP multipart request");    
                    }
                }else{
                    error!("Could not get part in HTTP multipart request");
                }
            }
            
            body.mp_body = Some(mp_body);
            req.body = Some(body);

            let resp = Client::post(req);

            let robj = engine.create_object();
            robj.set("code", resp.code).unwrap();
            
            let rs_headers = engine.create_object();
            for (key, value) in resp.headers{
                rs_headers.set(engine.create_string(key.as_str()).unwrap(), value).unwrap();
            }
            robj.set("headers", rs_headers).unwrap();

            match std::str::from_utf8(&resp.body){
                Ok(body) => robj.set("body", body).unwrap(),
                Err(_e) => robj.set("body", engine.create_bytes(&resp.body).unwrap()).unwrap()
            };
            
            return Ok(Value::Object(robj));
        }else{
            error!("Invalid argument for http multipart post, expected string");
            return Ok(Value::Null);    
        }
    }else{
        error!("Invalid argument for http multipart post, expected 2 arguments");
        return Ok(Value::Null);
    }
}
