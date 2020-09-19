use ducc::{Ducc, Object, Invocation, Value, Error as DuccError};
use std::fs::{DirEntry};
use log::*;

mod fileio;
mod folderio;

use fileio::FileIO;
use folderio::FolderIO;

pub const API_KEY:&str = "api";
pub const GALE_KEY:&str = "_gale";
pub const WEB_ROOT_KEY:&str = "wr";
pub const APP_KEY:&str = "app";
pub const FILE_API:&str = "fs";
pub const BOX:&str = "box";

pub const BOX_SPACE:u8 = 1;
pub const WEB_SPACE:u8 = 2;

pub fn load(engine:&Ducc) -> bool{
    let api_res:Result<Value, _> = engine.globals().get(API_KEY);
    let api_obj = api_res.unwrap();                
    let api = api_obj.as_object().unwrap();

    let fileio = engine.create_object();
    fileio.set("BOX", BOX_SPACE).unwrap();
    fileio.set("WEB", WEB_SPACE).unwrap();

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

    fileio.set("list", engine.create_function(folder_list)).unwrap();
    fileio.set("listAll", engine.create_function(folder_list_all)).unwrap();
    fileio.set("listFiles", engine.create_function(folder_list_files)).unwrap();
    fileio.set("listDirs", engine.create_function(folder_list_dirs)).unwrap();

    api.set(FILE_API, fileio).unwrap();

    return true;
}

fn get_base_dir(engine:&Ducc, space:u8) -> String{
    let robj:Object = engine.globals().get(GALE_KEY).unwrap();
    let web_root:String = robj.get(WEB_ROOT_KEY).unwrap();
    let app_name:String = robj.get(APP_KEY).unwrap();
    match space{
        BOX_SPACE => format!("{}/{}/{}", web_root, app_name, BOX),
        WEB_SPACE => format!("{}/{}", web_root, app_name),
        _=> format!("{}/{}/{}", web_root, app_name, BOX)
    }
}

pub fn file_create(inv: Invocation) -> Result<Value, DuccError>{
    let engine = inv.ducc;
    let args = inv.args;
    if args.len() == 2{
        let space_res = args.get(0);
        let fpath_res = args.get(1);
        if space_res.is_number() && fpath_res.is_string(){
            let base_dir = get_base_dir(engine, space_res.as_number().unwrap() as u8);
            let fpath = format!("{}/{}", base_dir, fpath_res.as_string().unwrap().to_string().unwrap());
            return Ok(Value::Boolean(FileIO::create(fpath)));
        }else{
            error!("Invalid argument, expected number, string");
            return Ok(Value::Boolean(false));    
        }
    }else{
        error!("Invalid argument, expected 2 arguments");
        return Ok(Value::Boolean(false));
    }
}

pub fn file_write_text(inv: Invocation) -> Result<Value, DuccError> {
    let engine = inv.ducc;
    let args = inv.args;
    if args.len() == 3{
        let space_res = args.get(0);
        let fpath_res = args.get(1);
        let text_res = args.get(2);
        if space_res.is_number() && fpath_res.is_string() && text_res.is_string(){
            let base_dir = get_base_dir(engine, space_res.as_number().unwrap() as u8);
            let fpath = format!("{}/{}", base_dir, fpath_res.as_string().unwrap().to_string().unwrap());
            let text = format!("{}", text_res.as_string().unwrap().to_string().unwrap());
            let res = FileIO::write_text(fpath, text.as_str());
            return Ok(Value::Boolean(res));
        }else{
            error!("Invalid argument, expected number, string, string arguments");
            return Ok(Value::Boolean(false));    
        }
    }else{
        error!("Invalid argument, expected 3 arguments");
        return Ok(Value::Boolean(false));
    }
}

pub fn file_append_text(inv: Invocation) -> Result<Value, DuccError> {
    let engine = inv.ducc;
    let args = inv.args;
    if args.len() == 3{
        let space_res = args.get(0);
        let fpath_res = args.get(1);
        let text_res = args.get(2);
        if space_res.is_number() && fpath_res.is_string() && text_res.is_string(){
            let base_dir = get_base_dir(engine, space_res.as_number().unwrap() as u8);
            let fpath = format!("{}/{}", base_dir, fpath_res.as_string().unwrap().to_string().unwrap());
            let text = format!("{}", text_res.as_string().unwrap().to_string().unwrap());
            let res = FileIO::append_text(fpath, text.as_str());
            return Ok(Value::Boolean(res));
        }else{
            error!("Invalid argument, expected number, string, string arguments");
            return Ok(Value::Boolean(false));    
        }
    }else{
        error!("Invalid argument, expected 3 arguments");
        return Ok(Value::Boolean(false));
    }
}

pub fn file_read_text(inv: Invocation) -> Result<Value, DuccError> {
    let engine = inv.ducc;
    let args = inv.args;
    if args.len() == 2{
        let space_res = args.get(0);
        let fpath_res = args.get(1);
        if space_res.is_number() && fpath_res.is_string(){
            let base_dir = get_base_dir(engine, space_res.as_number().unwrap() as u8);
            let fpath = format!("{}/{}", base_dir, fpath_res.as_string().unwrap().to_string().unwrap());
            let mut text = String::from("");
            let res = FileIO::read_text(fpath.clone(), &mut text);
            if res{
                return Ok(Value::String(engine.create_string(text.as_str()).unwrap()));
            }else{
                error!("Unable to read file: {}", fpath.clone());
                return Ok(Value::Null);
            }
        }else{
            error!("Invalid argument, expected number, string argument");
            return Ok(Value::Null);
        }
    }else{
        error!("Invalid argument, expected 2 argument");
        return Ok(Value::Null);
    }
}

pub fn file_read(inv: Invocation) -> Result<Value, DuccError> {
    let engine = inv.ducc;
    let args = inv.args;
    if args.len() == 2{
        let space_res = args.get(0);
        let fpath_res = args.get(1);
        if space_res.is_number() && fpath_res.is_string(){
            let base_dir = get_base_dir(engine, space_res.as_number().unwrap() as u8);
            let fpath = format!("{}/{}", base_dir, fpath_res.as_string().unwrap().to_string().unwrap());
            let mut buf = Vec::<u8>::new();
            let res = FileIO::read_bytes(fpath.clone(), &mut buf);
            if res{
                return Ok(Value::Bytes(engine.create_bytes(&buf).unwrap()));
            }else{
                error!("Error reading file: {}", fpath.clone());
                return Ok(Value::Null);
            }
        }else{
            error!("Invalid argument, expected number, string argument");
            return Ok(Value::Null);    
        }
    }else{
        error!("Invalid argument, expected 2 argument");
        return Ok(Value::Null);
    }
}

pub fn file_write(inv: Invocation) -> Result<Value, DuccError> {
    let engine = inv.ducc;
    let args = inv.args;
    if args.len() == 3{
        let space_res = args.get(0);
        let fpath_res = args.get(1);
        let bytes_res = args.get(2);
        if space_res.is_number() && fpath_res.is_string() && bytes_res.is_bytes(){
            let base_dir = get_base_dir(engine, space_res.as_number().unwrap() as u8);
            let fpath = format!("{}/{}", base_dir, fpath_res.as_string().unwrap().to_string().unwrap());
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
    if args.len() == 2{
        let space_res = args.get(0);
        let fpath_res = args.get(1);
        if space_res.is_number() && fpath_res.is_string(){
            let base_dir = get_base_dir(engine, space_res.as_number().unwrap() as u8);
            let fpath = format!("{}/{}", base_dir, fpath_res.as_string().unwrap().to_string().unwrap());
            return Ok(Value::Boolean(FileIO::remove(fpath)));
        }else{
            error!("Invalid argument, expected number, string argument");
            return Ok(Value::Boolean(false));
        }
    }else{
        error!("Invalid arguments, expected 2 arguments");
        return Ok(Value::Boolean(false));
    }
}

pub fn folder_create(inv: Invocation) -> Result<Value, DuccError>{
    let engine = inv.ducc;
    let args = inv.args;
    if args.len() == 2{
        let space_res = args.get(0);
        let fpath_res = args.get(1);
        if space_res.is_number() && fpath_res.is_string(){
            let base_dir = get_base_dir(engine, space_res.as_number().unwrap() as u8);
            let fpath = format!("{}/{}", base_dir, fpath_res.as_string().unwrap().to_string().unwrap());
            return Ok(Value::Boolean(FolderIO::create(fpath)));
        }else{
            error!("Invalid argument, expected number, string argument");
            return Ok(Value::Boolean(false));
        }
    }else{
        error!("Invalid argument, expected 2 arguments");
        return Ok(Value::Boolean(false));
    }
}

pub fn folder_create_all(inv: Invocation) -> Result<Value, DuccError>{
    let engine = inv.ducc;
    let args = inv.args;
    if args.len() == 2{
        let space_res = args.get(0);
        let fpath_res = args.get(1);
        if space_res.is_number() && fpath_res.is_string(){
            let base_dir = get_base_dir(engine, space_res.as_number().unwrap() as u8);
            let fpath = format!("{}/{}", base_dir, fpath_res.as_string().unwrap().to_string().unwrap());
            return Ok(Value::Boolean(FolderIO::create_all(fpath)));
        }else{
            error!("Invalid argument, expected number, string argument");
            return Ok(Value::Boolean(false));
        }
    }else{
        error!("Invalid argument, expected 2 arguments");
        return Ok(Value::Boolean(false));
    }
}

pub fn folder_remove(inv: Invocation) -> Result<Value, DuccError>{
    let engine = inv.ducc;
    let args = inv.args;
    if args.len() == 2{
        let space_res = args.get(0);
        let fpath_res = args.get(1);
        if space_res.is_number() && fpath_res.is_string(){
            let base_dir = get_base_dir(engine, space_res.as_number().unwrap() as u8);
            let fpath = format!("{}/{}", base_dir, fpath_res.as_string().unwrap().to_string().unwrap());
            return Ok(Value::Boolean(FolderIO::remove(fpath)));
        }else{
            error!("Invalid argument, expected number, string argument");
            return Ok(Value::Boolean(false));
        }
    }else{
        error!("Invalid argument, expected 2 arguments");
        return Ok(Value::Boolean(false));
    }
}

pub fn folder_remove_all(inv: Invocation) -> Result<Value, DuccError>{
    let engine = inv.ducc;
    let args = inv.args;
    if args.len() == 2{
        let space_res = args.get(0);
        let fpath_res = args.get(1);
        if space_res.is_number() && fpath_res.is_string(){
            let base_dir = get_base_dir(engine, space_res.as_number().unwrap() as u8);
            let fpath = format!("{}/{}", base_dir, fpath_res.as_string().unwrap().to_string().unwrap());
            return Ok(Value::Boolean(FolderIO::remove_all(fpath)));
        }else{
            error!("Invalid argument, expected number, string argument");
            return Ok(Value::Boolean(false));
        }
    }else{
        error!("Invalid argument, expected 2 arguments");
        return Ok(Value::Boolean(false));
    }
}

pub fn folder_list(inv: Invocation) -> Result<Value, DuccError> {
    let engine = inv.ducc;
    let args = inv.args;
    if args.len() == 2{
        let space_res = args.get(0);
        let fpath_res = args.get(1);
        if space_res.is_number() && fpath_res.is_string(){
            let base_dir = get_base_dir(engine, space_res.as_number().unwrap() as u8);
            let fpath = format!("{}/{}", base_dir, fpath_res.as_string().unwrap().to_string().unwrap());
            
            let entries_res = FolderIO::list(fpath.clone());
            if entries_res.is_some(){
                return get_entries_arr(engine, entries_res.unwrap());
            }else{
                error!("Error listing dir entries for: {}", fpath.clone());
                return Ok(Value::Null);
            }
        }else{
            error!("Invalid argument, expected number, string argument");
            return Ok(Value::Null);    
        }
    }else{
        error!("Invalid argument, expected 2 argument");
        return Ok(Value::Null);
    }
}

pub fn folder_list_all(inv: Invocation) -> Result<Value, DuccError> {
    let engine = inv.ducc;
    let args = inv.args;
    if args.len() == 2{
        let space_res = args.get(0);
        let fpath_res = args.get(1);
        if space_res.is_number() && fpath_res.is_string(){
            let base_dir = get_base_dir(engine, space_res.as_number().unwrap() as u8);
            let fpath = format!("{}/{}", base_dir, fpath_res.as_string().unwrap().to_string().unwrap());
            
            let entries_res = FolderIO::list_all(fpath.clone());
            if entries_res.is_some(){
                return get_entries_arr(engine, entries_res.unwrap());
            }else{
                error!("Error listing dir entries for: {}", fpath.clone());
                return Ok(Value::Null);
            }
        }else{
            error!("Invalid argument, expected number, string argument");
            return Ok(Value::Null);    
        }
    }else{
        error!("Invalid argument, expected 2 argument");
        return Ok(Value::Null);
    }
}

pub fn folder_list_files(inv: Invocation) -> Result<Value, DuccError> {
    let engine = inv.ducc;
    let args = inv.args;
    if args.len() == 2{
        let space_res = args.get(0);
        let fpath_res = args.get(1);
        if space_res.is_number() && fpath_res.is_string(){
            let base_dir = get_base_dir(engine, space_res.as_number().unwrap() as u8);
            let fpath = format!("{}/{}", base_dir, fpath_res.as_string().unwrap().to_string().unwrap());
            
            let entries_res = FolderIO::list_files(fpath.clone());
            if entries_res.is_some(){
                return get_entries_arr(engine, entries_res.unwrap());
            }else{
                error!("Error listing dir entries for: {}", fpath.clone());
                return Ok(Value::Null);
            }
        }else{
            error!("Invalid argument, expected number, string argument");
            return Ok(Value::Null);    
        }
    }else{
        error!("Invalid argument, expected 2 argument");
        return Ok(Value::Null);
    }
}

pub fn folder_list_dirs(inv: Invocation) -> Result<Value, DuccError> {
    let engine = inv.ducc;
    let args = inv.args;
    if args.len() == 2{
        let space_res = args.get(0);
        let fpath_res = args.get(1);
        if space_res.is_number() && fpath_res.is_string(){
            let base_dir = get_base_dir(engine, space_res.as_number().unwrap() as u8);
            let fpath = format!("{}/{}", base_dir, fpath_res.as_string().unwrap().to_string().unwrap());
            
            let entries_res = FolderIO::list_dirs(fpath.clone());
            if entries_res.is_some(){
                return get_entries_arr(engine, entries_res.unwrap());
            }else{
                error!("Error listing dir entries for: {}", fpath.clone());
                return Ok(Value::Null);
            }
        }else{
            error!("Invalid argument, expected number, string argument");
            return Ok(Value::Null);    
        }
    }else{
        error!("Invalid argument, expected 2 argument");
        return Ok(Value::Null);
    }
}

fn get_entries_arr(engine:&Ducc, entries: Vec<DirEntry>) -> Result<Value, DuccError>{
    let entries_arr = engine.create_array();
    for entry in entries{
        let entry_obj = engine.create_object();
        entry_obj.set("name", entry.file_name().to_str().unwrap()).unwrap();
        let meta_res = entry.metadata();
        if meta_res.is_ok(){
            let meta = meta_res.unwrap();
            entry_obj.set("isFile", meta.is_file()).unwrap();
            entry_obj.set("isDir", meta.is_dir()).unwrap();
            entry_obj.set("len", meta.len()).unwrap();
            //entry_obj.set("permissions", meta.permissions()).unwrap();
            //entry_obj.set("created", format!("{}", meta.created().unwrap().)).unwrap();
            //entry_obj.set("accessed", format!("{}", meta.accessed().unwrap().)).unwrap();
            //entry_obj.set("modified", format!("{}", meta.modified().unwrap().)).unwrap();
        }else{
            entry_obj.set("isFile", Value::Null).unwrap();
            entry_obj.set("isDir", Value::Null).unwrap();
            entry_obj.set("len", Value::Null).unwrap();
            entry_obj.set("permissions", Value::Null).unwrap();
            entry_obj.set("created", Value::Null).unwrap();
            entry_obj.set("accessed", Value::Null).unwrap();
            entry_obj.set("modified", Value::Null).unwrap();
        }
                            
        entries_arr.push(entry_obj).unwrap();
    }
    return Ok(Value::Array(entries_arr));
}
