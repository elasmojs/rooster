fn main(){
    let header_str = "";
    for key in headers.keys(){
        header_str += key + " : " + headers[key] + "\r\n";
    }
    
    return header_str;
}