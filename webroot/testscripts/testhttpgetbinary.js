var http = require('http');

var resp = {
    "code": 200,
    "msg":""
};

var sresp = http.get("https://upload.wikimedia.org/wikipedia/commons/thumb/3/30/Amurtiger-Zoo-Muenster.jpg/1024px-Amurtiger-Zoo-Muenster.jpg", {});
if(sresp != null){
    $r.response.body = sresp.body;
}else{
    resp.code = 500;
    resp.msg = "Could not fire HTTP GET request";
    $r.response.headers["content-type"] = "application/json";
    $r.response.body = JSON.stringify(resp);
}
