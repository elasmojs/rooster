use base64;
use urlencoding;

pub struct Base64{
}

impl Base64{
    pub fn encode(input:String) -> String{
        return base64::encode(input);
    }

    pub fn decode(input:String) -> Option<String>{
        let res = base64::decode(input);
        if res.is_ok(){
            return Some(String::from_utf8(res.unwrap()).unwrap());
        }else{
            println!("Error: {}", res.unwrap_err());
            return None;
        } 
    }
}

pub struct URLEncode{}

impl URLEncode{
    pub fn encode(input:String) -> String{
        return urlencoding::encode(&input);
    }

    pub fn decode(input:String) -> Option<String>{
        let res = urlencoding::decode(&input);
        if res.is_ok(){
            return Some(res.unwrap());
        }else{
            println!("Error: {}", res.unwrap_err());
            return None;
        } 
    }
}
