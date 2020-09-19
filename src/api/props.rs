use ducc::{Ducc, Object, Invocation, Value, Error as DuccError};
use std::fs::read_to_string;
use log::*;

pub const API_KEY:&str = "api";
pub const GALE_KEY:&str = "_gale";
pub const WEB_ROOT_KEY:&str = "wr"; 
pub const APP_KEY:&str = "app"; 
pub const PROPS_API:&str = "props";
pub const BOX:&str = "box";

pub fn load(engine:&Ducc) -> bool{
    let api_res:Result<Value, _> = engine.globals().get(API_KEY);
    let api_obj = api_res.unwrap();                
    let api = api_obj.as_object().unwrap();

    let props = engine.create_object();
    props.set("get", engine.create_function(props_get)).unwrap();

    api.set(PROPS_API, props).unwrap();

    return true;
}

pub fn props_get(inv: Invocation) -> Result<Value, DuccError>{
    let engine = inv.ducc;
    let args = inv.args;
    if args.len() == 1{
        let robj:Object = engine.globals().get(GALE_KEY).unwrap();
        let web_root:String = robj.get(WEB_ROOT_KEY).unwrap();
        let app_name:String = robj.get(APP_KEY).unwrap();

        let fpath_res = args.get(0);
        if fpath_res.is_string(){
            let path = format!("{}/{}/{}/{}", web_root, app_name, BOX, fpath_res.as_string().unwrap().to_string().unwrap());
            let props = read_to_string(path.clone());
            if props.is_ok(){
                return Ok(Value::String(engine.create_string(props.unwrap().as_str()).unwrap()));
            }else{
                error!("Property file not found: {}", path.clone());
                return Ok(Value::Null);
            }
        }else{
            error!("Invalid argument, expected string");
            return Ok(Value::Null);
        }
    }else{
        error!("Invalid argument, expected 1 argument");
        return Ok(Value::Null);
    }
}
