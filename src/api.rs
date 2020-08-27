use ducc::{Ducc, Object, Invocation, Value, Error as DuccError};
use log::*;

mod fileio;

use fileio::FileIO;

pub const FILE_API:&str = "fs";
const APP_DATA_ROOT:&str = "data";

pub fn load_core_api(name:&str, engine:&Ducc) -> bool{
    let api_obj:Value;
    let api_ref:Object;
    let api:&Object;
    if !engine.globals().contains_key("api").unwrap(){
        api_ref = engine.create_object();
        api = &api_ref;
        engine.globals().set("api", api.to_owned()).unwrap();
    }else{
        let api_res:Result<Value, _> = engine.globals().get("api");
        api_obj = api_res.unwrap();                
        api = api_obj.as_object().unwrap();
    }

    match name{
        FILE_API => {
            //Adding File API
            if !api.contains_key(FILE_API).unwrap(){
                let fileio = engine.create_object();
                fileio.set("create", engine.create_function(file_create)).unwrap();
                fileio.set("writeText", engine.create_function(file_write_text)).unwrap();
                fileio.set("readText", engine.create_function(file_read_text)).unwrap();
                fileio.set("appendText", engine.create_function(file_append_text)).unwrap();
                fileio.set("remove", engine.create_function(file_remove)).unwrap();
                //TODO: Add all other fileio API
                api.set(FILE_API, fileio).unwrap();
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

pub fn file_create(inv: Invocation) -> Result<Value, DuccError>{
    let args = inv.args;
    if args.len() == 1{
        let fpath_res = args.get(0);
        let fpath = format!("{}/{}", APP_DATA_ROOT, fpath_res.as_string().unwrap().to_string().unwrap());
        return Ok(Value::Boolean(FileIO::create(fpath)));
    }else{
        error!("Invalid argument, expected 1 argument");
        return Ok(Value::Boolean(false));
    }
}

pub fn file_write_text(inv: Invocation) -> Result<Value, DuccError> {
    let args = inv.args;
    if args.len() == 2{
        let fpath_res = args.get(0);
        let fpath = format!("{}/{}", APP_DATA_ROOT, fpath_res.as_string().unwrap().to_string().unwrap());
        let text_res = args.get(1);
        let text = format!("{}", text_res.as_string().unwrap().to_string().unwrap());
        let res = FileIO::write_text(fpath, text.as_str());
        return Ok(Value::Boolean(res));
    }else{
        error!("Invalid argument, expected 2 arguments");
        return Ok(Value::Boolean(false));
    }
}

pub fn file_append_text(inv: Invocation) -> Result<Value, DuccError> {
    let args = inv.args;
    if args.len() == 2{
        let fpath_res = args.get(0);
        let fpath = format!("{}/{}", APP_DATA_ROOT, fpath_res.as_string().unwrap().to_string().unwrap());
        let text_res = args.get(1);
        let text = format!("{}", text_res.as_string().unwrap().to_string().unwrap());
        let res = FileIO::append_text(fpath, text.as_str());
        return Ok(Value::Boolean(res));
    }else{
        error!("Invalid argument, expected 2 arguments");
        return Ok(Value::Boolean(false));
    }
}

pub fn file_read_text(inv: Invocation) -> Result<Value, DuccError> {
    let args = inv.args;
    if args.len() == 1{
        let fpath_res = args.get(0);
        let fpath = format!("{}/{}", APP_DATA_ROOT, fpath_res.as_string().unwrap().to_string().unwrap());
        let mut text = String::from("");
        let res = FileIO::read_text(fpath, &mut text);
        if res{
            return Ok(Value::String(inv.ducc.create_string(text.as_str()).unwrap()));
        }else{
            return Ok(Value::Null);
        }
    }else{
        error!("Invalid argument, expected 1 argument");
        return Ok(Value::Null);
    }
}

pub fn file_remove(inv: Invocation) -> Result<Value, DuccError>{
    let args = inv.args;
    if args.len() == 1{
        let fpath_res = args.get(0);
        let fpath = format!("{}/{}", APP_DATA_ROOT, fpath_res.as_string().unwrap().to_string().unwrap());
        return Ok(Value::Boolean(FileIO::remove(fpath)));
    }else{
        error!("Invalid argument, expected 1 argument");
        return Ok(Value::Boolean(false));
    }
}
