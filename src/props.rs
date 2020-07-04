use lazy_static::lazy_static;
use std::collections::HashMap;
use std::clone::Clone;

#[derive(Clone)]
pub struct Props{
    pub net_port:i32,
    pub web_root:String,
    pub web_default:String,
    pub remote_addr:String,
    
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
        max_expr_depths_local:usize, max_expr_depths_global:usize, 
        max_call_levels:usize, max_modules:usize,
        max_map_size:usize, max_array_size:usize,
        max_string_size:usize) -> Props{
        return Props{
            net_port: port,
            web_root: root,
            web_default: default,
            remote_addr: String::from(""),
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
        let parsed_result = dotproperties::parse_from_file("rooster.cfg");
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
    get_max_expr_depths_local(), get_max_expr_depths_global(),
    get_max_call_levels(), get_max_modules(),
    get_max_map_size(), get_max_array_size(),
    get_max_string_size());
}

pub fn get_port() -> i32 {
    let port_num_prop = PROPS.get("net.port");
    let mut port_num = 7070;
    if !port_num_prop.is_none(){
        port_num = port_num_prop.unwrap().trim().parse::<u16>().unwrap();
    }
    return port_num as i32;
}

pub fn get_web_root() -> String {
    let web_root_prop = PROPS.get("web.root");
    let mut web_root = ".";
    if !web_root_prop.is_none(){
        web_root = web_root_prop.unwrap().trim();
    }
    return String::from(web_root);
}

pub fn get_web_default() -> String {
    let web_default_prop = PROPS.get("web.default");
    let mut web_default = "index.html";
    if !web_default_prop.is_none(){
        web_default = web_default_prop.unwrap().trim();
    }
    return String::from(web_default);
}

pub fn get_max_expr_depths_global() -> usize {
    let max_expr_depths_global_prop = PROPS.get("script.maxexprdepths.global");
    let mut max_expr_depths_global = 500;
    if !max_expr_depths_global_prop.is_none(){
        max_expr_depths_global = max_expr_depths_global_prop.unwrap().trim().parse::<u16>().unwrap();
    }
    return max_expr_depths_global as usize;
}

pub fn get_max_expr_depths_local() -> usize {
    let max_expr_depths_local_prop = PROPS.get("script.maxexprdepths.local");
    let mut max_expr_depths_local = 500;
    if !max_expr_depths_local_prop.is_none(){
        max_expr_depths_local = max_expr_depths_local_prop.unwrap().trim().parse::<u16>().unwrap();
    }
    return max_expr_depths_local as usize;
}

pub fn get_max_call_levels() -> usize {
    let max_call_levels_prop = PROPS.get("script.maxcalllevels");
    let mut max_call_levels = 500;
    if !max_call_levels_prop.is_none(){
        max_call_levels = max_call_levels_prop.unwrap().trim().parse::<u16>().unwrap();
    }
    return max_call_levels as usize;
}

pub fn get_max_modules() -> usize {
    let max_modules_prop = PROPS.get("script.maxmodules");
    let mut max_modules = 1000;
    if !max_modules_prop.is_none(){
        max_modules = max_modules_prop.unwrap().trim().parse::<u16>().unwrap();
    }
    return max_modules as usize;
}

pub fn get_max_map_size() -> usize {
    let max_map_size_prop = PROPS.get("script.maxmapsize");
    let mut max_map_size = 1500;
    if !max_map_size_prop.is_none(){
        max_map_size = max_map_size_prop.unwrap().trim().parse::<u16>().unwrap();
    }
    return max_map_size as usize;
}

pub fn get_max_array_size() -> usize {
    let max_array_size_prop = PROPS.get("script.maxarraysize");
    let mut max_array_size = 1500;
    if !max_array_size_prop.is_none(){
        max_array_size = max_array_size_prop.unwrap().trim().parse::<u16>().unwrap();
    }
    return max_array_size as usize;
}

pub fn get_max_string_size() -> usize {
    let max_string_size_prop = PROPS.get("script.maxstringsize");
    let mut max_string_size = 5000;
    if !max_string_size_prop.is_none(){
        max_string_size = max_string_size_prop.unwrap().trim().parse::<u16>().unwrap();
    }
    return max_string_size as usize;
}