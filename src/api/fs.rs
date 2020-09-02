use ducc::{Ducc, Object, Invocation, Value, Error as DuccError};
use log::*;

mod fileio;
mod folderio;

use fileio::FileIO;
use folderio::FolderIO;

pub const API_KEY:&str = "api";
pub const ROOSTER_KEY:&str = "_rooster";
pub const DATA_ROOT_KEY:&str = "dr"; 
pub const FILE_API:&str = "fs";

pub fn load(engine:&Ducc) -> bool{
    let api_res:Result<Value, _> = engine.globals().get(API_KEY);
    let api_obj = api_res.unwrap();                
    let api = api_obj.as_object().unwrap();

    let fileio = engine.create_object();
    fileio.set("create", engine.create_function(file_create)).unwrap();
    fileio.set("writeText", engine.create_function(file_write_text)).unwrap();
    fileio.set("readText", engine.create_function(file_read_text)).unwrap();
    fileio.set("appendText", engine.create_function(file_append_text)).unwrap();

    fileio.set("read", engine.create_function(file_read)).unwrap();
    fileio.set("write", engine.create_function(file_write)).unwrap();

    fileio.set("remove", engine.create_function(file_remove)).unwrap();

    fileio.set("createDir", engine.create_function(folder_create)).unwrap();
    fileio.set("createDirAll", engine.create_function(folder_create_all)).unwrap();
    fileio.set("removeDir", engine.create_function(folder_remove)).unwrap();
    fileio.set("removeDirAll", engine.create_function(folder_remove_all)).unwrap();

    api.set(FILE_API, fileio).unwrap();

    return true;
}


pub fn file_create(inv: Invocation) -> Result<Value, DuccError>{
    let engine = inv.ducc;
    let args = inv.args;
    if args.len() == 1{
        let robj:Object = engine.globals().get(ROOSTER_KEY).unwrap();
        let data_root:String = robj.get(DATA_ROOT_KEY).unwrap();

        let fpath_res = args.get(0);
        if fpath_res.is_string(){
            let fpath = format!("{}/{}", data_root, fpath_res.as_string().unwrap().to_string().unwrap());
            return Ok(Value::Boolean(FileIO::create(fpath)));
        }else{
            error!("Invalid argument, expected string");
            return Ok(Value::Boolean(false));    
        }
    }else{
        error!("Invalid argument, expected 1 argument");
        return Ok(Value::Boolean(false));
    }
}

pub fn file_write_text(inv: Invocation) -> Result<Value, DuccError> {
    let engine = inv.ducc;
    let args = inv.args;
    if args.len() == 2{
        let robj:Object = engine.globals().get(ROOSTER_KEY).unwrap();
        let data_root:String = robj.get(DATA_ROOT_KEY).unwrap();
        
        let fpath_res = args.get(0);
        let text_res = args.get(1);
        
        if fpath_res.is_string() && text_res.is_string(){
            let fpath = format!("{}/{}", data_root, fpath_res.as_string().unwrap().to_string().unwrap());
            let text = format!("{}", text_res.as_string().unwrap().to_string().unwrap());
            let res = FileIO::write_text(fpath, text.as_str());
            return Ok(Value::Boolean(res));
        }else{
            error!("Invalid argument, expected string arguments");
            return Ok(Value::Boolean(false));    
        }
    }else{
        error!("Invalid argument, expected 2 arguments");
        return Ok(Value::Boolean(false));
    }
}

pub fn file_append_text(inv: Invocation) -> Result<Value, DuccError> {
    let engine = inv.ducc;
    let args = inv.args;
    if args.len() == 2{
        let robj:Object = engine.globals().get(ROOSTER_KEY).unwrap();
        let data_root:String = robj.get(DATA_ROOT_KEY).unwrap();

        let fpath_res = args.get(0);
        let text_res = args.get(1);
        
        if fpath_res.is_string() && text_res.is_string(){
            let fpath = format!("{}/{}", data_root, fpath_res.as_string().unwrap().to_string().unwrap());
            let text = format!("{}", text_res.as_string().unwrap().to_string().unwrap());
            let res = FileIO::append_text(fpath, text.as_str());
            return Ok(Value::Boolean(res));
        }else{
            error!("Invalid argument, expected string arguments");
            return Ok(Value::Boolean(false));    
        }
    }else{
        error!("Invalid argument, expected 2 arguments");
        return Ok(Value::Boolean(false));
    }
}

pub fn file_read_text(inv: Invocation) -> Result<Value, DuccError> {
    let engine = inv.ducc;
    let args = inv.args;
    if args.len() == 1{
        let robj:Object = engine.globals().get(ROOSTER_KEY).unwrap();
        let data_root:String = robj.get(DATA_ROOT_KEY).unwrap();

        let fpath_res = args.get(0);
        if fpath_res.is_string(){
            let fpath = format!("{}/{}", data_root, fpath_res.as_string().unwrap().to_string().unwrap());
            let mut text = String::from("");
            let res = FileIO::read_text(fpath.clone(), &mut text);
            if res{
                return Ok(Value::String(engine.create_string(text.as_str()).unwrap()));
            }else{
                error!("Unable to read file: {}", fpath.clone());
                return Ok(Value::Null);
            }
        }else{
            error!("Invalid argument, expected string argument");
            return Ok(Value::Null);
        }
    }else{
        error!("Invalid argument, expected 1 argument");
        return Ok(Value::Null);
    }
}

pub fn file_read(inv: Invocation) -> Result<Value, DuccError> {
    let engine = inv.ducc;
    let args = inv.args;
    if args.len() == 1{
        let robj:Object = engine.globals().get(ROOSTER_KEY).unwrap();
        let data_root:String = robj.get(DATA_ROOT_KEY).unwrap();

        let fpath_res = args.get(0);
        if fpath_res.is_string(){
            let fpath = format!("{}/{}", data_root, fpath_res.as_string().unwrap().to_string().unwrap());
            let mut buf = Vec::<u8>::new();
            let res = FileIO::read_bytes(fpath.clone(), &mut buf);
            if res{
                return Ok(Value::Bytes(engine.create_bytes(&buf).unwrap()));
            }else{
                error!("Error reading file: {}", fpath.clone());
                return Ok(Value::Null);
            }
        }else{
            error!("Invalid argument, expected string argument");
            return Ok(Value::Null);    
        }
    }else{
        error!("Invalid argument, expected 1 argument");
        return Ok(Value::Null);
    }
}

pub fn file_write(inv: Invocation) -> Result<Value, DuccError> {
    let engine = inv.ducc;
    let args = inv.args;
    if args.len() == 2{
        let robj:Object = engine.globals().get(ROOSTER_KEY).unwrap();
        let data_root:String = robj.get(DATA_ROOT_KEY).unwrap();
        
        let fpath_res = args.get(0);
        let bytes_res = args.get(1);

        if fpath_res.is_string() && bytes_res.is_bytes(){
            let fpath = format!("{}/{}", data_root, fpath_res.as_string().unwrap().to_string().unwrap());
            let buf = bytes_res.as_bytes().unwrap().to_vec();
            let res = FileIO::write_bytes(fpath, &buf);
            return Ok(Value::Boolean(res));
        }else{
            error!("Unexpected arguments for write");
            return Ok(Value::Boolean(false));
        }
    }else{
        error!("Invalid argument, expected 2 arguments");
        return Ok(Value::Boolean(false));
    }
}

pub fn file_remove(inv: Invocation) -> Result<Value, DuccError>{
    let engine = inv.ducc;
    let args = inv.args;
    if args.len() == 1{
        let robj:Object = engine.globals().get(ROOSTER_KEY).unwrap();
        let data_root:String = robj.get(DATA_ROOT_KEY).unwrap();

        let fpath_res = args.get(0);
        if fpath_res.is_string(){
            let fpath = format!("{}/{}", data_root, fpath_res.as_string().unwrap().to_string().unwrap());
            return Ok(Value::Boolean(FileIO::remove(fpath)));
        }else{
            error!("Invalid argument, expected string argument");
            return Ok(Value::Boolean(false));
        }
    }else{
        error!("Invalid argument, expected 1 argument");
        return Ok(Value::Boolean(false));
    }
}

pub fn folder_create(inv: Invocation) -> Result<Value, DuccError>{
    let engine = inv.ducc;
    let args = inv.args;
    if args.len() == 1{
        let robj:Object = engine.globals().get(ROOSTER_KEY).unwrap();
        let data_root:String = robj.get(DATA_ROOT_KEY).unwrap();

        let fpath_res = args.get(0);
        if fpath_res.is_string(){
            let fpath = format!("{}/{}", data_root, fpath_res.as_string().unwrap().to_string().unwrap());
            return Ok(Value::Boolean(FolderIO::create(fpath)));
        }else{
            error!("Invalid argument, expected string argument");
            return Ok(Value::Boolean(false));
        }
    }else{
        error!("Invalid argument, expected 1 argument");
        return Ok(Value::Boolean(false));
    }
}

pub fn folder_create_all(inv: Invocation) -> Result<Value, DuccError>{
    let engine = inv.ducc;
    let args = inv.args;
    if args.len() == 1{
        let robj:Object = engine.globals().get(ROOSTER_KEY).unwrap();
        let data_root:String = robj.get(DATA_ROOT_KEY).unwrap();

        let fpath_res = args.get(0);
        if fpath_res.is_string(){
            let fpath = format!("{}/{}", data_root, fpath_res.as_string().unwrap().to_string().unwrap());
            return Ok(Value::Boolean(FolderIO::create_all(fpath)));
        }else{
            error!("Invalid argument, expected string argument");
            return Ok(Value::Boolean(false));
        }
    }else{
        error!("Invalid argument, expected 1 argument");
        return Ok(Value::Boolean(false));
    }
}

pub fn folder_remove(inv: Invocation) -> Result<Value, DuccError>{
    let engine = inv.ducc;
    let args = inv.args;
    if args.len() == 1{
        let robj:Object = engine.globals().get(ROOSTER_KEY).unwrap();
        let data_root:String = robj.get(DATA_ROOT_KEY).unwrap();

        let fpath_res = args.get(0);
        if fpath_res.is_string(){
            let fpath = format!("{}/{}", data_root, fpath_res.as_string().unwrap().to_string().unwrap());
            return Ok(Value::Boolean(FolderIO::remove(fpath)));
        }else{
            error!("Invalid argument, expected string argument");
            return Ok(Value::Boolean(false));
        }
    }else{
        error!("Invalid argument, expected 1 argument");
        return Ok(Value::Boolean(false));
    }
}

pub fn folder_remove_all(inv: Invocation) -> Result<Value, DuccError>{
    let engine = inv.ducc;
    let args = inv.args;
    if args.len() == 1{
        let robj:Object = engine.globals().get(ROOSTER_KEY).unwrap();
        let data_root:String = robj.get(DATA_ROOT_KEY).unwrap();

        let fpath_res = args.get(0);
        if fpath_res.is_string(){
            let fpath = format!("{}/{}", data_root, fpath_res.as_string().unwrap().to_string().unwrap());
            return Ok(Value::Boolean(FolderIO::remove_all(fpath)));
        }else{
            error!("Invalid argument, expected string argument");
            return Ok(Value::Boolean(false));
        }
    }else{
        error!("Invalid argument, expected 1 argument");
        return Ok(Value::Boolean(false));
    }
}
