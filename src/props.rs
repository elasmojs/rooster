use lazy_static::lazy_static;
use std::collections::HashMap;
use std::clone::Clone;

const ROOSTER_CFG_FILE:&str = "./rooster.cfg";

const DEFAULT_PORT:u16 = 7070;
const DEFAULT_WEB_ROOT:&str = ".";
const DEFAULT_WEB_DEFAULT:&str = "index.html";

const DEFAULT_LOG_LEVEL:&str = "ERROR";
const DEFAULT_LOG_FILE_PATH:&str = "./rooster.log";
const DEFAULT_LOG_TO_CONSOLE:bool = false;
const DEFAULT_LOG_TIME_FORMAT:&str = "%m-%d-%Y %T";

const DEFAULT_MAX_EXPR_DEPTH:u16 = 500;
const DEFAULT_MAX_CALL_LEVELS:u16 = 500;
const DEFAULT_MAX_MODULES:u16 = 1000;
const DEFAULT_MAX_MAP_SIZE:u16 = 1500;
const DEFAULT_MAX_ARRAY_SIZE:u16 = 1500;
const DEFAULT_MAX_STRING_SIZE:u16 = 5000;

#[derive(Clone)]
pub struct Props{
    pub net_port:i32,
    pub web_root:String,
    pub web_default:String,
    pub remote_addr:String,

    pub log_file_path:String,
    pub log_level:String,
    pub log_to_console:bool,
    pub log_time_format:String,
    
    pub max_expr_depths_global:usize,
    pub max_expr_depths_local:usize,
    pub max_call_levels:usize,
    pub max_modules:usize,
    pub max_map_size:usize,
    pub max_array_size:usize,
    pub max_string_size:usize
}

impl Props{
    fn new(port:i32, root:String, default:String, 
        log_file_path:String, log_level:String, log_to_console:bool, log_time_format:String,
        max_expr_depths_local:usize, max_expr_depths_global:usize, 
        max_call_levels:usize, max_modules:usize,
        max_map_size:usize, max_array_size:usize,
        max_string_size:usize) -> Props{
        return Props{
            net_port: port,
            web_root: root,
            web_default: default,
            remote_addr: String::from(""),
            log_file_path: log_file_path,
            log_level: log_level,
            log_to_console: log_to_console,
            log_time_format: log_time_format,
            max_expr_depths_local: max_expr_depths_local,
            max_expr_depths_global: max_expr_depths_global,
            max_call_levels: max_call_levels,
            max_modules: max_modules,
            max_map_size: max_map_size,
            max_array_size: max_array_size,
            max_string_size:max_string_size
        };
    }
}

lazy_static! {
    static ref PROPS: HashMap<std::string::String, std::string::String> = {
        let parsed_result = dotproperties::parse_from_file(ROOSTER_CFG_FILE);
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
    return Props::new(get_port(), get_web_root(), get_web_default(),
    get_log_file_path(), get_log_level(), get_log_to_console(), get_log_time_format(),
    get_max_expr_depths_local(), get_max_expr_depths_global(),
    get_max_call_levels(), get_max_modules(),
    get_max_map_size(), get_max_array_size(),
    get_max_string_size());
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

pub fn get_log_file_path() -> String {
    let log_file_path_prop = PROPS.get("log.file");
    let mut log_file_path = DEFAULT_LOG_FILE_PATH;
    if !log_file_path_prop.is_none(){
        log_file_path = log_file_path_prop.unwrap().trim();
    }
    return String::from(log_file_path);
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

pub fn get_max_expr_depths_global() -> usize {
    let max_expr_depths_global_prop = PROPS.get("script.maxexprdepths.global");
    let mut max_expr_depths_global = 500;
    if !max_expr_depths_global_prop.is_none(){
        max_expr_depths_global = match max_expr_depths_global_prop.unwrap().trim().parse::<u16>(){
            Ok(medg) => medg,
            Err(_e) => DEFAULT_MAX_EXPR_DEPTH
        };
    }
    return max_expr_depths_global as usize;
}

pub fn get_max_expr_depths_local() -> usize {
    let max_expr_depths_local_prop = PROPS.get("script.maxexprdepths.local");
    let mut max_expr_depths_local = DEFAULT_MAX_EXPR_DEPTH;
    if !max_expr_depths_local_prop.is_none(){
        max_expr_depths_local = match max_expr_depths_local_prop.unwrap().trim().parse::<u16>(){
            Ok(medl) => medl,
            Err(_e) => DEFAULT_MAX_EXPR_DEPTH
        };
    }
    return max_expr_depths_local as usize;
}

pub fn get_max_call_levels() -> usize {
    let max_call_levels_prop = PROPS.get("script.maxcalllevels");
    let mut max_call_levels = DEFAULT_MAX_CALL_LEVELS;
    if !max_call_levels_prop.is_none(){
        max_call_levels = match max_call_levels_prop.unwrap().trim().parse::<u16>(){
            Ok(mcl) => mcl,
            Err(_e) => DEFAULT_MAX_CALL_LEVELS
        };
    }
    return max_call_levels as usize;
}

pub fn get_max_modules() -> usize {
    let max_modules_prop = PROPS.get("script.maxmodules");
    let mut max_modules = DEFAULT_MAX_MODULES;
    if !max_modules_prop.is_none(){
        max_modules = match max_modules_prop.unwrap().trim().parse::<u16>(){
            Ok(mm) => mm,
            Err(_e) => DEFAULT_MAX_MODULES
        };
    }
    return max_modules as usize;
}

pub fn get_max_map_size() -> usize {
    let max_map_size_prop = PROPS.get("script.maxmapsize");
    let mut max_map_size = DEFAULT_MAX_MAP_SIZE;
    if !max_map_size_prop.is_none(){
        max_map_size = match max_map_size_prop.unwrap().trim().parse::<u16>(){
            Ok(mms) => mms,
            Err(_e) => DEFAULT_MAX_MAP_SIZE
        };
    }
    return max_map_size as usize;
}

pub fn get_max_array_size() -> usize {
    let max_array_size_prop = PROPS.get("script.maxarraysize");
    let mut max_array_size = DEFAULT_MAX_ARRAY_SIZE;
    if !max_array_size_prop.is_none(){
        max_array_size = match max_array_size_prop.unwrap().trim().parse::<u16>(){
            Ok(mas) => mas,
            Err(_e) => DEFAULT_MAX_ARRAY_SIZE
        };
    }
    return max_array_size as usize;
}

pub fn get_max_string_size() -> usize {
    let max_string_size_prop = PROPS.get("script.maxstringsize");
    let mut max_string_size = DEFAULT_MAX_STRING_SIZE;
    if !max_string_size_prop.is_none(){
        max_string_size = match max_string_size_prop.unwrap().trim().parse::<u16>(){
            Ok(mss) => mss,
            Err(_e) => DEFAULT_MAX_STRING_SIZE
        };
    }
    return max_string_size as usize;
}