use lazy_static::lazy_static;
use std::collections::HashMap;
use std::clone::Clone;

#[derive(Clone)]
pub struct Props{
    pub net_port:i32,
    pub web_root:String,
    pub web_default:String,
    pub remote_addr:String
}

impl Props{
    fn new(port:i32, root:String, default:String) -> Props{
        return Props{
            net_port: port,
            web_root: root,
            web_default: default,
            remote_addr: String::from("")
        };
    }

    /*
    pub fn get_net_port(&mut self) -> i32 {
        self.net_port.clone()
    }

    pub fn get_web_root(&mut self) -> String {
        self.web_root.clone()
    }

    pub fn get_web_default(&mut self) -> String {
        self.web_default.clone()
    }

    pub fn get_remote_addr(&mut self) -> String {
        self.remote_addr.clone()
    }
    */
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
    return Props::new(get_port(), get_web_root(), get_web_default());
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