use std::path::Path;
use std::fs::{create_dir, create_dir_all, remove_dir, remove_dir_all, read_dir, DirEntry};

use log::*;

pub struct FolderIO{}

impl FolderIO{

    pub fn create(folderpath:String) -> bool{
        let path = Path::new(&folderpath);
        if path.exists(){
            return true;
        }
        let res = create_dir(path);
        if res.is_ok(){
            return true;
        }else{
            FolderIO::log_error(res.unwrap_err().to_string());
            return false;
        }
    }

    pub fn create_all(folderpath:String) -> bool{
        let path = Path::new(&folderpath);
        if path.exists(){
            return true;
        }
        let res = create_dir_all(path);
        if res.is_ok(){
            return true;
        }else{
            FolderIO::log_error(res.unwrap_err().to_string());
            return false;
        }
    }

    pub fn remove(folderpath:String) -> bool{
        let path = Path::new(&folderpath);
        if !path.exists(){
            return true;
        }
        let res = remove_dir(path);
        if res.is_ok(){
            return true;
        }else{
            FolderIO::log_error(res.unwrap_err().to_string());
            return false;
        }
    }

    pub fn remove_all(folderpath:String) -> bool{
        let path = Path::new(&folderpath);
        if !path.exists(){
            return true;
        }
        let res = remove_dir_all(path);
        if res.is_ok(){
            return true;
        }else{
            FolderIO::log_error(res.unwrap_err().to_string());
            return false;
        }
    }

    pub fn list(folderpath:String) -> Option<Vec<DirEntry>>{
        let dir_res = read_dir(folderpath.clone());
        if dir_res.is_ok(){
            let mut entry_vec:Vec<DirEntry> = Vec::new();
            let dir = dir_res.unwrap();
            for entry_res in dir{
                if entry_res.is_ok(){
                    let entry = entry_res.unwrap();
                    let is_hidden = entry.file_name().to_str().unwrap().starts_with(".");
                    if !is_hidden{
                        entry_vec.push(entry);      
                    }              
                }
            }
            return Some(entry_vec);
        }else{
            error!("Could not read dir - {}", folderpath);
            return None;
        }
    }

    pub fn list_all(folderpath:String) -> Option<Vec<DirEntry>>{
        let dir_res = read_dir(folderpath.clone());
        if dir_res.is_ok(){
            let mut entry_vec:Vec<DirEntry> = Vec::new();
            let dir = dir_res.unwrap();
            for entry_res in dir{
                if entry_res.is_ok(){
                    let entry = entry_res.unwrap();
                    entry_vec.push(entry);              
                }
            }
            return Some(entry_vec);
        }else{
            error!("Could not read dir - {}", folderpath);
            return None;
        }
    }

    pub fn list_files(folderpath:String) -> Option<Vec<DirEntry>>{
        let dir_res = read_dir(folderpath.clone());
        if dir_res.is_ok(){
            let mut entry_vec:Vec<DirEntry> = Vec::new();
            let dir = dir_res.unwrap();
            for entry_res in dir{
                if entry_res.is_ok(){
                    let entry = entry_res.unwrap();
                    if entry.metadata().unwrap().is_file(){
                        entry_vec.push(entry);
                    }         
                }
            }
            return Some(entry_vec);
        }else{
            error!("Could not read dir - {}", folderpath);
            return None;
        }
    }

    pub fn list_dirs(folderpath:String) -> Option<Vec<DirEntry>>{
        let dir_res = read_dir(folderpath.clone());
        if dir_res.is_ok(){
            let mut entry_vec:Vec<DirEntry> = Vec::new();
            let dir = dir_res.unwrap();
            for entry_res in dir{
                if entry_res.is_ok(){
                    let entry = entry_res.unwrap();
                    if entry.metadata().unwrap().is_dir(){
                        entry_vec.push(entry);
                    }              
                }
            }
            return Some(entry_vec);
        }else{
            error!("Could not read dir - {}", folderpath);
            return None;
        }
    }

    
    fn log_error(err:String ){
        error!("Error: {}", err);
    }
}
