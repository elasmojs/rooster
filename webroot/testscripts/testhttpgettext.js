var http = require('http');

var resp = {
    "code": 200,
    "msg":""
};

var headers = {
    "My-Header": "My Header Value"
}

var sresp = http.get("https://postman-echo.com/get?foo1=bar1&foo2=bar2", headers);
if(sresp != null){
    resp.msg = {
        code: sresp.code,
        headers: sresp.headers,
        body: sresp.body
    }
}else{
    resp.code = 500;
    resp.msg = "Could not fire HTTP GET request";
}

$g.response.headers["content-type"] = "application/json";
$g.response.body = JSON.stringify(resp);
