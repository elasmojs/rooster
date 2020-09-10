use ducc::{Ducc, Object, Value};

mod fs;
mod uuid;
mod rnd;
mod crypto;
mod encode;
mod zip;
mod httpio;
mod html;

use fs::load as loadfs;
use self::uuid::load as loaduuid;
use rnd::load as loadrnd;
use crypto::load as loadcrypto;
use encode::load as loadencode;
use self::zip::load as loadzip;
use self::httpio::load as loadhttp;
use self::html::{load as loadhtml, set_user_data as html_user_data};

pub const API_KEY:&str = "api";
pub const FILE_API:&str = "fs";
pub const UUID_API:&str = "uuid";
pub const RND_API:&str = "rnd";
pub const CRYPTO_API:&str = "crypto";
pub const ENCODE_API:&str = "encode";
pub const ZIP_API:&str = "zip";
pub const HTTP_API:&str = "http";
pub const HTML_API:&str = "html";

pub fn init_user_data(engine:&mut Ducc){
    html_user_data(engine);   
}

pub fn load_core_api(name:&str, engine:&Ducc) -> bool{
    let api_obj:Value;
    let api_ref:Object;
    let api:&Object;
    if !engine.globals().contains_key(API_KEY).unwrap(){
        api_ref = engine.create_object();
        api = &api_ref;
        engine.globals().set(API_KEY, api.to_owned()).unwrap();
    }else{
        let api_res:Result<Value, _> = engine.globals().get(API_KEY);
        api_obj = api_res.unwrap();                
        api = api_obj.as_object().unwrap();
    }

    match name{
        FILE_API => {
            if !api.contains_key(FILE_API).unwrap(){
                loadfs(engine);
            }            
            return true;
        },
        UUID_API => {
            if !api.contains_key(UUID_API).unwrap(){
                loaduuid(engine);
            }            
            return true;
        },
        RND_API => {
            if !api.contains_key(RND_API).unwrap(){
                loadrnd(engine);
            }            
            return true;
        },
        CRYPTO_API => {
            if !api.contains_key(CRYPTO_API).unwrap(){
                loadcrypto(engine);
            }            
            return true;
        },
        ENCODE_API => {
            if !api.contains_key(ENCODE_API).unwrap(){
                loadencode(engine);
            }            
            return true;
        },
        ZIP_API => {
            if !api.contains_key(ZIP_API).unwrap(){
                loadzip(engine);
            }            
            return true;
        },
        HTTP_API => {
            if !api.contains_key(HTTP_API).unwrap(){
                loadhttp(engine);
            }            
            return true;
        },
        HTML_API => {
            if !api.contains_key(HTML_API).unwrap(){
                loadhtml(engine);
            }            
            return true;
        },
        //TODO: Add all other APIs
        _ => {
            //API not found
            return false;
        }
    }
}

