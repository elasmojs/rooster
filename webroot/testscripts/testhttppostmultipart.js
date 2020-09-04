var http = require('http');

var resp = {
    "code": 200,
    "msg":""
};

var headers = {
    "my-mpheader": "my multipart post"
};

var parts = [
    {
        "type":"text",
        "name":"firstname",
        "value":"Vignesh"
    },
    {
        "type":"text",
        "name":"lastname",
        "value":"Swaminathan"
    },
    {
        "type":"file",
        "name":"spotlight.png",
        "value":"spotlight.png"
    }
];

var sresp = http.postMultiPart("https://postman-echo.com/post", headers, parts);
if(sresp != null){
    resp.msg = {
        code: sresp.code,
        headers: sresp.headers,
        body: sresp.body
    }
}else{
    resp.code = 500;
    resp.msg = "Could not fire HTTP POST request";
}

$r.response.headers["content-type"] = "application/json";
$r.response.body = JSON.stringify(resp);
