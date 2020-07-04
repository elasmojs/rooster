fn main(){
    let header_str = "";
    for key in request.headers.keys(){
        header_str += key + " : " + request.headers[key] + "\r\n";
    }
    
    return request.remote_addr + "\r\n\r\n" + request.method + " " + request.path + "?" + request.query + "\r\n\r\n" + header_str + "\r\n" + request.body + "\r\n\r\nMultipart: " + request.is_multipart;
}