use lazy_static::lazy_static;
use std::collections::HashMap;
use std::clone::Clone;

const GALE_CFG_FILE:&str = "./gale.cfg";

const DEFAULT_PORT:u16 = 7070;
const DEFAULT_WEB_ROOT:&str = ".";
const DEFAULT_WEB_DEFAULT:&str = "index.html";

const DEFAULT_DATA_ROOT:&str = "./data";

const DEFAULT_LOG_LEVEL:&str = "ERROR";
const DEFAULT_LOG_FOLDER_PATH:&str = "./logs";
const DEFAULT_LOG_TO_CONSOLE:bool = false;
const DEFAULT_LOG_TIME_FORMAT:&str = "%m-%d-%Y %T";


#[derive(Clone)]
pub struct Props{
    pub net_port:i32,
    pub remote_addr:String,

    pub web_root:String,
    pub web_default:String,
    
    pub data_root:String,

    pub log_folder_path:String,
    pub log_level:String,
    pub log_to_console:bool,
    pub log_time_format:String
}

impl Props{
    fn new(port:i32, root:String, default:String, data_root:String, 
        log_folder_path:String, log_level:String, log_to_console:bool, log_time_format:String,
        ) -> Props{
        return Props{
            net_port: port,
            remote_addr: String::from(""),

            web_root: root,
            web_default: default,
            
            data_root: data_root,

            log_folder_path: log_folder_path,
            log_level: log_level,
            log_to_console: log_to_console,
            log_time_format: log_time_format
        };
    }
}

lazy_static! {
    static ref PROPS: HashMap<std::string::String, std::string::String> = {
        let parsed_result = dotproperties::parse_from_file(GALE_CFG_FILE);
        let mut mapped = HashMap::new();
        let _parsed = match parsed_result{
            Ok(_parsed) => {
                mapped = _parsed.into_iter().collect();
            }
            Err(_e) => {
                
            }
        };
        return mapped;
    };
}

pub fn get_props() -> Props{
    return Props::new(get_port(), get_web_root(), get_web_default(), get_data_root(), 
    get_log_folder_path(), get_log_level(), get_log_to_console(), get_log_time_format());
}

pub fn get_port() -> i32 {
    let port_num_prop = PROPS.get("net.port");
    let mut port_num = DEFAULT_PORT;
    if !port_num_prop.is_none(){
        port_num = match port_num_prop.unwrap().trim().parse::<u16>(){
            Ok(port) => port,
            Err(_e) => DEFAULT_PORT
        };
    }
    return port_num as i32;
}

pub fn get_web_root() -> String {
    let web_root_prop = PROPS.get("web.root");
    let mut web_root = DEFAULT_WEB_ROOT;
    if !web_root_prop.is_none(){
        web_root = web_root_prop.unwrap().trim();
    }
    return String::from(web_root);
}

pub fn get_web_default() -> String {
    let web_default_prop = PROPS.get("web.default");
    let mut web_default = DEFAULT_WEB_DEFAULT;
    if !web_default_prop.is_none(){
        web_default = web_default_prop.unwrap().trim();
    }
    return String::from(web_default);
}

pub fn get_data_root() -> String {
    let data_root_prop = PROPS.get("data.root");
    let mut data_root = DEFAULT_DATA_ROOT;
    if !data_root_prop.is_none(){
        data_root = data_root_prop.unwrap().trim();
    }
    return String::from(data_root);
}

pub fn get_log_folder_path() -> String {
    let log_folder_path_prop = PROPS.get("log.folder");
    let mut log_folder_path = DEFAULT_LOG_FOLDER_PATH;
    if !log_folder_path_prop.is_none(){
        log_folder_path = log_folder_path_prop.unwrap().trim();
    }
    return String::from(log_folder_path);
}

pub fn get_log_level() -> String {
    let log_level_prop = PROPS.get("log.level");
    let mut log_level = DEFAULT_LOG_LEVEL;
    if !log_level_prop.is_none(){
        log_level = log_level_prop.unwrap().trim();
    }
    return String::from(log_level).to_uppercase();
}

pub fn get_log_to_console() -> bool {
    let log_to_console_prop = PROPS.get("log.console");
    let mut log_to_console = DEFAULT_LOG_TO_CONSOLE;
    if !log_to_console_prop.is_none(){
        log_to_console = match log_to_console_prop.unwrap().trim().parse::<bool>(){
            Ok(tc) => tc,
            Err(_e) => DEFAULT_LOG_TO_CONSOLE
        }
    }
    return log_to_console;
}

pub fn get_log_time_format() -> String {
    let log_time_format_prop = PROPS.get("log.timeformat");
    let mut log_time_format = DEFAULT_LOG_TIME_FORMAT;
    if !log_time_format_prop.is_none(){
        log_time_format = log_time_format_prop.unwrap().trim();
    }
    return String::from(log_time_format);
}
