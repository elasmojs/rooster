use ducc::{Ducc, Object, Invocation, Value, Error as DuccError};
use log::*;

mod zipio;

use zipio::Zip;

pub const API_KEY:&str = "api";
pub const ROOSTER_KEY:&str = "_rooster";
pub const DATA_ROOT_KEY:&str = "dr"; 
pub const ZIP_API:&str = "zip";

pub fn load(engine:&Ducc) -> bool{
    let api_res:Result<Value, _> = engine.globals().get(API_KEY);
    let api_obj = api_res.unwrap();                
    let api = api_obj.as_object().unwrap();

    let zipio = engine.create_object();
    zipio.set("create", engine.create_function(zip_create)).unwrap();
    zipio.set("extract", engine.create_function(zip_extract)).unwrap();

    api.set(ZIP_API, zipio).unwrap();

    return true;
}


pub fn zip_create(inv: Invocation) -> Result<Value, DuccError>{
    let engine = inv.ducc;
    let args = inv.args;
    if args.len() == 2{
        let robj:Object = engine.globals().get(ROOSTER_KEY).unwrap();
        let data_root:String = robj.get(DATA_ROOT_KEY).unwrap();

        let srcpath_res = args.get(0);
        let destpath_res = args.get(1);

        if srcpath_res.is_string() && destpath_res.is_string(){
            let srcpath = format!("{}/{}", data_root, srcpath_res.as_string().unwrap().to_string().unwrap());
            let destpath = format!("{}/{}", data_root, destpath_res.as_string().unwrap().to_string().unwrap());
            return Ok(Value::Boolean(Zip::create(srcpath, destpath)));
        }else{
            error!("Invalid argument for zip create, expected string");
            return Ok(Value::Boolean(false));    
        }
    }else{
        error!("Invalid argument for zip create, expected 2 arguments");
        return Ok(Value::Boolean(false));
    }
}

pub fn zip_extract(inv: Invocation) -> Result<Value, DuccError>{
    let engine = inv.ducc;
    let args = inv.args;
    if args.len() == 2{
        let robj:Object = engine.globals().get(ROOSTER_KEY).unwrap();
        let data_root:String = robj.get(DATA_ROOT_KEY).unwrap();

        let srcpath_res = args.get(0);
        let destpath_res = args.get(1);

        if srcpath_res.is_string() && destpath_res.is_string(){
            let srcpath = format!("{}/{}", data_root, srcpath_res.as_string().unwrap().to_string().unwrap());
            let destpath = format!("{}/{}", data_root, destpath_res.as_string().unwrap().to_string().unwrap());
            return Ok(Value::Boolean(Zip::extract(srcpath, destpath)));
        }else{
            error!("Invalid argument for zip create, expected string");
            return Ok(Value::Boolean(false));    
        }
    }else{
        error!("Invalid argument for zip create, expected 2 arguments");
        return Ok(Value::Boolean(false));
    }
}
