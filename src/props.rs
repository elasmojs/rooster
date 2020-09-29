use lazy_static::lazy_static;
use std::collections::HashMap;
use std::clone::Clone;

const GALE_CFG_FILE:&str = "./gale.cfg";

const DEFAULT_PORT:u16 = 7070;
const DEFAULT_HTTP_ENABLED:bool = true;
const DEFAULT_SSL_PORT:u16 = 4443;
const DEFAULT_SSL_ENABLED:bool = false;
const DEFAULT_SSL_CERT:&str = "./conf/server.crt";
const DEFAULT_SSL_PKEY:&str = "./conf/server.key";

const DEFAULT_WEB_ROOT:&str = "./web";
const DEFAULT_WEB_DEFAULT:&str = "index.html";

const DEFAULT_APP:&str = "gale";

const DEFAULT_LOG_LEVEL:&str = "ERROR";
const DEFAULT_LOG_FOLDER_PATH:&str = "./logs";
const DEFAULT_LOG_TO_CONSOLE:bool = false;
const DEFAULT_LOG_TIME_FORMAT:&str = "%m-%d-%Y %T";


#[derive(Clone)]
pub struct Props{
    pub net_http_enabled:bool,
    pub net_port:i32,
    pub net_ssl_port:i32,
    pub net_ssl_enabled:bool,
    pub net_ssl_cert:String,
    pub net_ssl_pkey:String,

    pub remote_addr:String,

    pub web_root:String,
    pub web_default:String,
    
    pub default_app:String,
    pub apps:Vec<String>,

    pub log_folder_path:String,
    pub log_level:String,
    pub log_to_console:bool,
    pub log_time_format:String
}

impl Props{
    fn new(http_enabled:bool, port:i32, ssl_port:i32, ssl_enabled:bool, ssl_cert:String, ssl_pkey:String, 
        root:String, default:String, default_app:String,  
        log_folder_path:String, log_level:String, log_to_console:bool, log_time_format:String,
        ) -> Props{
        return Props{
            net_http_enabled: http_enabled,
            net_port: port,
            net_ssl_port: ssl_port,
            net_ssl_enabled: ssl_enabled,
            net_ssl_cert: ssl_cert,
            net_ssl_pkey: ssl_pkey,

            remote_addr: String::from(""),

            web_root: root,
            web_default: default,
            
            default_app: default_app,
            apps: Vec::new(),

            log_folder_path: log_folder_path,
            log_level: log_level,
            log_to_console: log_to_console,
            log_time_format: log_time_format
        };
    }

    pub fn is_app(&self, name: String) -> bool{
        for app_name in self.apps.clone(){
            if app_name.eq(&name){
                return true;
            }
        }
        return false;
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
    return Props::new(get_http_enabled(), get_port(), get_ssl_port(), get_ssl_enabled(), get_ssl_cert(), get_ssl_pkey(), 
    get_web_root(), get_web_default(), get_default_app(), 
    get_log_folder_path(), get_log_level(), get_log_to_console(), get_log_time_format());
}

pub fn get_http_enabled() -> bool {
    let http_enabled_prop = PROPS.get("http.enabled");
    let mut http_enabled = DEFAULT_HTTP_ENABLED;
    if !http_enabled_prop.is_none(){
        http_enabled = match http_enabled_prop.unwrap().trim().parse::<bool>(){
            Ok(enabled) => enabled,
            Err(_e) => DEFAULT_HTTP_ENABLED
        };
    }
    return http_enabled;
}

pub fn get_port() -> i32 {
    let port_num_prop = PROPS.get("http.port");
    let mut port_num = DEFAULT_PORT;
    if !port_num_prop.is_none(){
        port_num = match port_num_prop.unwrap().trim().parse::<u16>(){
            Ok(port) => port,
            Err(_e) => DEFAULT_PORT
        };
    }
    return port_num as i32;
}

pub fn get_ssl_port() -> i32 {
    let ssl_port_num_prop = PROPS.get("ssl.port");
    let mut ssl_port_num = DEFAULT_SSL_PORT;
    if !ssl_port_num_prop.is_none(){
        ssl_port_num = match ssl_port_num_prop.unwrap().trim().parse::<u16>(){
            Ok(port) => port,
            Err(_e) => DEFAULT_PORT
        };
    }
    return ssl_port_num as i32;
}

pub fn get_ssl_enabled() -> bool {
    let ssl_enabled_prop = PROPS.get("ssl.enabled");
    let mut ssl_enabled = DEFAULT_SSL_ENABLED;
    if !ssl_enabled_prop.is_none(){
        ssl_enabled = match ssl_enabled_prop.unwrap().trim().parse::<bool>(){
            Ok(enabled) => enabled,
            Err(_e) => DEFAULT_SSL_ENABLED
        };
    }
    return ssl_enabled;
}

pub fn get_ssl_cert() -> String {
    let ssl_cert_prop = PROPS.get("ssl.cert");
    let mut ssl_cert = DEFAULT_SSL_CERT;
    if !ssl_cert_prop.is_none(){
        ssl_cert = ssl_cert_prop.unwrap().trim();
    }
    return String::from(ssl_cert);
}

pub fn get_ssl_pkey() -> String {
    let ssl_pkey_prop = PROPS.get("ssl.pkey");
    let mut ssl_pkey = DEFAULT_SSL_PKEY;
    if !ssl_pkey_prop.is_none(){
        ssl_pkey = ssl_pkey_prop.unwrap().trim();
    }
    return String::from(ssl_pkey);
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

pub fn get_default_app() -> String {
    let default_app_prop = PROPS.get("app.default");
    let mut default_app = DEFAULT_APP;
    if !default_app_prop.is_none(){
        default_app = default_app_prop.unwrap().trim();
    }
    return String::from(default_app);
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
