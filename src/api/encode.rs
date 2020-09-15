use ducc::{Ducc, Invocation, Value, Error as DuccError};
use log::*;

mod encode;

use self::encode::Base64;
use self::encode::URLEncode;

pub const API_KEY:&str = "api";
pub const ENCODE_API:&str = "encode";


pub fn load(engine:&Ducc) -> bool{
    let api_res:Result<Value, _> = engine.globals().get(API_KEY);
    let api_obj = api_res.unwrap();                
    let api = api_obj.as_object().unwrap();

    let base64 = engine.create_object();
    base64.set("encode", engine.create_function(b64_encode)).unwrap();
    base64.set("encodeBytes", engine.create_function(b64_encode_bytes)).unwrap();
    base64.set("decode", engine.create_function(b64_decode)).unwrap();
    
    let urlencode = engine.create_object();
    urlencode.set("encode", engine.create_function(url_encode)).unwrap();
    urlencode.set("decode", engine.create_function(url_decode)).unwrap();
    
    let encode = engine.create_object();
    encode.set("base64", base64).unwrap();
    encode.set("url", urlencode).unwrap();
    api.set(ENCODE_API, encode).unwrap();

    return true;
}

pub fn b64_encode(inv: Invocation) -> Result<Value, DuccError>{
    let engine = inv.ducc;
    let args = inv.args;
    if args.len() == 1{
        let input_str = args.get(0);
        if input_str.is_string(){
            let input = input_str.as_string().unwrap().to_string().unwrap();
            return Ok(Value::String(engine.create_string(Base64::encode(input).as_str()).unwrap()));
        }else{
            error!("Invalid argument for base64 encode, expected string");
            return Ok(Value::Null);    
        }
    }else{
        error!("Invalid argument for base64 encode, expected 1 argument");
        return Ok(Value::Null);
    }
}

pub fn b64_encode_bytes(inv: Invocation) -> Result<Value, DuccError>{
    let engine = inv.ducc;
    let args = inv.args;
    if args.len() == 1{
        let input_str = args.get(0);
        if input_str.is_bytes(){
            let input = input_str.as_bytes().unwrap().to_vec();
            let encoded = Base64::encode_bytes(input);
            return Ok(Value::String(engine.create_string(encoded.as_str()).unwrap()));
        }else{
            error!("Invalid argument for base64 encode, expected byte array");
            return Ok(Value::Null);    
        }
    }else{
        error!("Invalid argument for base64 encode, expected 1 argument");
        return Ok(Value::Null);
    }
}

pub fn b64_decode(inv: Invocation) -> Result<Value, DuccError>{
    let engine = inv.ducc;
    let args = inv.args;
    if args.len() == 1{
        let input_str = args.get(0);
        if input_str.is_string(){
            let input = input_str.as_string().unwrap().to_string().unwrap();
            let res = Base64::decode(input);
            if res.is_some(){
                return Ok(Value::String(engine.create_string(res.unwrap().as_str()).unwrap()));
            }else{
                error!("Unable to decode base64");
                return Ok(Value::Null);
            }
        }else{
            error!("Invalid argument for base64 decode, expected string");
            return Ok(Value::Null);    
        }
    }else{
        error!("Invalid argument for base64 decode, expected 1 argument");
        return Ok(Value::Null);
    }
}

pub fn url_encode(inv: Invocation) -> Result<Value, DuccError>{
    let engine = inv.ducc;
    let args = inv.args;
    if args.len() == 1{
        let input_str = args.get(0);
        if input_str.is_string(){
            let input = input_str.as_string().unwrap().to_string().unwrap();
            return Ok(Value::String(engine.create_string(URLEncode::encode(input).as_str()).unwrap()));
        }else{
            error!("Invalid argument for URL encode, expected string");
            return Ok(Value::Null);    
        }
    }else{
        error!("Invalid argument for URL encode, expected 1 argument");
        return Ok(Value::Null);
    }
}

pub fn url_decode(inv: Invocation) -> Result<Value, DuccError>{
    let engine = inv.ducc;
    let args = inv.args;
    if args.len() == 1{
        let input_str = args.get(0);
        if input_str.is_string(){
            let input = input_str.as_string().unwrap().to_string().unwrap();
            let res = URLEncode::decode(input);
            if res.is_some(){
                return Ok(Value::String(engine.create_string(res.unwrap().as_str()).unwrap()));
            }else{
                error!("Unable to decode URL");
                return Ok(Value::Null);
            }
        }else{
            error!("Invalid argument for URL decode, expected string");
            return Ok(Value::Null);    
        }
    }else{
        error!("Invalid argument for URL decode, expected 1 argument");
        return Ok(Value::Null);
    }
}
