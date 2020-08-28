use ducc::{Ducc, Object, Value};

mod fs;
mod uuid;
mod rnd;

use fs::load as loadfs;
use self::uuid::load as loaduuid;
use rnd::load as loadrnd;

pub const API_KEY:&str = "api";
pub const FILE_API:&str = "fs";
pub const UUID_API:&str = "uuid";
pub const RND_API:&str = "rnd";

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
            //Adding File API
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
        //TODO: Add all other APIs
        _ => {
            //API not found
            return false;
        }
    }
}

