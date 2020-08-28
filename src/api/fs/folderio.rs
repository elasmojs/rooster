use std::path::Path;
use std::fs::{create_dir, create_dir_all, remove_dir, remove_dir_all};

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
    
    fn log_error(err:String ){
        error!("Error: {}", err);
    }
}
