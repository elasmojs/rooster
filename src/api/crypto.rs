use ducc::{Ducc, Invocation, Value, Error as DuccError};
use log::*;

mod crypto;

use self::crypto::Crypto;

pub const API_KEY:&str = "api";
pub const CRYPTO_API:&str = "crypto";


pub fn load(engine:&Ducc) -> bool{
    let api_res:Result<Value, _> = engine.globals().get(API_KEY);
    let api_obj = api_res.unwrap();                
    let api = api_obj.as_object().unwrap();

    let crypto = engine.create_object();
    crypto.set("crc32", engine.create_function(crypto_crc32)).unwrap();
    crypto.set("md5", engine.create_function(crypto_md5)).unwrap();
    crypto.set("sha2", engine.create_function(crypto_sha2)).unwrap();
    crypto.set("sha3", engine.create_function(crypto_sha3)).unwrap();
    
    api.set(CRYPTO_API, crypto).unwrap();

    return true;
}

pub fn crypto_crc32(inv: Invocation) -> Result<Value, DuccError>{
    let engine = inv.ducc;
    let args = inv.args;
    if args.len() == 1{
        let input_str = args.get(0);
        if input_str.is_string(){
            let input = input_str.as_string().unwrap().to_string().unwrap();
            return Ok(Value::String(engine.create_string(Crypto::crc32(input).as_str()).unwrap()));
        }else{
            error!("Invalid argument for crc32, expected string");
            return Ok(Value::Null);    
        }
    }else{
        error!("Invalid argument for crc32, expected 1 argument");
        return Ok(Value::Null);
    }
}

pub fn crypto_md5(inv: Invocation) -> Result<Value, DuccError>{
    let engine = inv.ducc;
    let args = inv.args;
    if args.len() == 1{
        let input_str = args.get(0);
        if input_str.is_string(){
            let input = input_str.as_string().unwrap().to_string().unwrap();
            return Ok(Value::String(engine.create_string(Crypto::md5(input).as_str()).unwrap()));
        }else{
            error!("Invalid argument for md5, expected string");
            return Ok(Value::Null);    
        }
    }else{
        error!("Invalid argument for md5, expected 1 argument");
        return Ok(Value::Null);
    }
}

pub fn crypto_sha2(inv: Invocation) -> Result<Value, DuccError>{
    let engine = inv.ducc;
    let args = inv.args;
    if args.len() == 1{
        let input_str = args.get(0);
        if input_str.is_string(){
            let input = input_str.as_string().unwrap().to_string().unwrap();
            return Ok(Value::String(engine.create_string(Crypto::sha2(input).as_str()).unwrap()));
        }else{
            error!("Invalid argument for sha2, expected string");
            return Ok(Value::Null);
        }
    }else{
        error!("Invalid argument for md5, expected 1 argument");
        return Ok(Value::Null);
    }
}

pub fn crypto_sha3(inv: Invocation) -> Result<Value, DuccError>{
    let engine = inv.ducc;
    let args = inv.args;
    if args.len() == 1{
        let input_str = args.get(0);
        if input_str.is_string(){
            let input = input_str.as_string().unwrap().to_string().unwrap();
            return Ok(Value::String(engine.create_string(Crypto::sha3(input).as_str()).unwrap()));
        }else{
            error!("Invalid argument for sha3, expected string");
            return Ok(Value::Null);
        }
    }else{
        error!("Invalid argument for sha3, expected 1 argument");
        return Ok(Value::Null);
    }
}
