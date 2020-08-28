use std::fs::{File, OpenOptions, remove_file};
use std::io::{Read, Write};

use log::*;
pub struct FileIO{}

impl FileIO{
    pub fn create(filepath:String) -> bool{
        let res = File::create(&filepath);
        if res.is_ok(){
            return true;
        }else{
            FileIO::log_error(res.unwrap_err().to_string());
            return false;
        }
    }

    pub fn read_text(filepath:String, text:&mut String) -> bool{
        let file = File::open(&filepath);
        if file.is_ok(){
            let read_res = file.unwrap().read_to_string(text);
            if read_res.is_ok(){
                return true;
            }else{
                FileIO::log_error(read_res.unwrap_err().to_string());
                return false;
            }
        }else{
            FileIO::log_error(file.unwrap_err().to_string());
            return false;
        }
    }

    pub fn write_text(filepath:String, text:&str) -> bool{
        let file = OpenOptions::new().write(true).open(&filepath);
        if file.is_ok(){
            let write_res = file.unwrap().write_all(text.as_bytes());
            if write_res.is_ok(){
                return true;
            }else{
                FileIO::log_error(write_res.unwrap_err().to_string());
                return false;
            }
        }else{
            FileIO::log_error(file.unwrap_err().to_string());
            return false;
        }
    }

    pub fn append_text(filepath:String, text:&str) -> bool{
        let file = OpenOptions::new().append(true).open(&filepath);
        if file.is_ok(){
            let write_res = file.unwrap().write_all(text.as_bytes());
            if write_res.is_ok(){
                return true;
            }else{
                FileIO::log_error(write_res.unwrap_err().to_string());
                return false;
            }
        }else{
            FileIO::log_error(file.unwrap_err().to_string());
            return false;
        }
    }

    pub fn write_bytes(filepath:String, buf:&[u8]) -> bool{
        let file = OpenOptions::new().write(true).open(&filepath);
        if file.is_ok(){
            let write_res = file.unwrap().write_all(&buf);
            if write_res.is_ok(){
                return true;
            }else{
                FileIO::log_error(write_res.unwrap_err().to_string());
                return false;
            }
        }else{
            FileIO::log_error(file.unwrap_err().to_string());
            return false;
        }
    }

    pub fn read_bytes(filepath:String, mut buf:&mut Vec<u8>) ->bool{
        let file = File::open(&filepath);
        if file.is_ok(){
            let read_res = file.unwrap().read_to_end(&mut buf);
            if read_res.is_ok(){
                return true;
            }else{
                FileIO::log_error(read_res.unwrap_err().to_string());
                return false;
            }
        }else{
            FileIO::log_error(file.unwrap_err().to_string());
            return false;
        }
    }
    

    pub fn remove(filepath:String) -> bool{
        let res = remove_file(&filepath);
        if res.is_ok(){
            return true;
        }else{
            FileIO::log_error(res.unwrap_err().to_string());
            return false;
        }
    }

    fn log_error(err:String ){
        error!("Error: {}", err);
    }
}
