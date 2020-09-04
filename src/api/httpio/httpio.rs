use std::collections::HashMap;
use std::io::Read;
use ureq::{Request as UReqRequest};
use multipart::client::Multipart;
use multipart::mock::{ClientRequest, HttpBuffer};
use log::*;

const GET_METHOD:u16 = 0;
const POST_METHOD:u16 = 1;
//const PUT_METHOD:u16 = 2;
//const DELETE_METHOD:u16 = 3;

pub struct Body{
    pub is_multipart: bool,
    pub mp_body: Option<Multipart<HttpBuffer>>,
    pub body: Option<Vec<u8>>
}

impl Body{
    pub fn default() -> Body{
        return Body{
            is_multipart: false,
            mp_body: None,
            body: None,
        };
    }
}

pub struct Request{
    pub url: String,
    pub headers: HashMap<String, String>,
    pub body: Option<Body>,
    pub is_multipart: bool
}

impl Request{
    pub fn new(url:String, headers: HashMap<String, String>) -> Request{
        return Request{
            url: url.clone(),
            headers: headers.clone(),
            body: None,
            is_multipart: false
        };
    }

    pub fn get_multipart_body(&self) -> Multipart<HttpBuffer>{
        let mp_result = Multipart::from_request(ClientRequest::default());
        return mp_result.unwrap();
    }
}

#[derive(Clone)]
pub struct Response{
    pub code: u16,
    pub headers: HashMap<String, String>,
    pub body: Vec<u8>
}

impl Response{
    pub fn new(code: u16, headers: HashMap<String, String>, body: Vec<u8>) -> Response{
        return Response{
            code: code,
            headers: headers.clone(),
            body: body.clone()
        };
    }

    pub fn default() -> Response{
        return Response{
            code: 200,
            headers: HashMap::new(),
            body: Vec::new()
        };
    }
}

pub struct Client{}

impl Client{
    pub fn get(req: Request) -> Response{
        let mut request = ureq::get(req.url.clone().as_str());
        Client::set_headers(req.headers, &mut request);
        Client::send(GET_METHOD, &mut request, req.body)        
    }

    pub fn post(req: Request) -> Response{
        let mut request = ureq::post(req.url.clone().as_str());
        Client::set_headers(req.headers, &mut request);
        Client::send(POST_METHOD, &mut request, req.body)
    }

    fn set_headers(headers:HashMap<String, String>, request:&mut UReqRequest){
        for (header, value) in headers{
            request.set(header.as_str(), value.as_str());
        }
    }

    fn send(method:u16, request:&mut UReqRequest, body_opt: Option<Body>) -> Response{
        let resp_opt = match method{
            GET_METHOD => Some(request.call()),
            POST_METHOD => {
                if body_opt.is_none(){
                    //POST method called without body
                    Some(request.call())
                }else{
                    let body = body_opt.unwrap();
                    if !body.is_multipart{
                        if body.body.is_some(){
                            Some(request.send_bytes(&body.body.unwrap()))
                        }else{
                            None
                        }
                    }else{
                        if body.mp_body.is_some(){
                            let mp_body = body.mp_body.unwrap();
                            match mp_body.send(){
                                Ok(http_buf) =>{
                                    let ctype = format!("multipart/form-data; boundary={}", http_buf.boundary);
                                    request.set("content-type", &ctype);
                                    Some(request.send_bytes(&(http_buf.buf.clone())))
                                }
                                Err(_e) => None
                            }
                        }else{
                            None
                        }
                    }
                }
            },
            _ => None
        };
        if resp_opt.is_none(){
            return Response::default();
        }

        let resp = resp_opt.unwrap();
        let code = resp.status();

        let mut headers = HashMap::new();
        for header in resp.headers_names(){
            headers.insert(header.clone(), String::from(resp.header(header.as_str()).unwrap()));
        }

        let mut reader = resp.into_reader();
        let mut bytes = vec![];
        let body_res = reader.read_to_end(&mut bytes);
        if body_res.is_err(){
            error!("Error: reading HTTP/HTTPS response body");
        }

        return Response::new(code, headers.clone(), bytes.clone());
    }
}
