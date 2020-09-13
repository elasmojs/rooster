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
        "value":"John"
    },
    {
        "type":"text",
        "name":"lastname",
        "value":"Doe"
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

$g.response.headers["content-type"] = "application/json";
$g.response.body = JSON.stringify(resp);
