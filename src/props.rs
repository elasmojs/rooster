use lazy_static::lazy_static;
use std::collections::HashMap;

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
        mapped
    };
}

pub fn get_port() -> u16 {
    let port_num_prop = PROPS.get("net.port");
    let mut port_num = 7070;
    if !port_num_prop.is_none(){
        port_num = port_num_prop.unwrap().trim().parse::<u16>().unwrap();
    }
    return port_num;
}

pub fn get_web_root() -> &'static str {
    let web_root_prop = PROPS.get("web.root");
    let mut web_root = ".";
    if !web_root_prop.is_none(){
        web_root = web_root_prop.unwrap().trim();
    }
    return web_root;
}

pub fn get_web_default() -> &'static str {
    let web_default_prop = PROPS.get("web.default");
    let mut web_default = "index.html";
    if !web_default_prop.is_none(){
        web_default = web_default_prop.unwrap().trim();
    }
    return web_default;
}