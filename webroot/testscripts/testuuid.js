var uuid = require('uuid');

var resp = {
    "code": 200,
    "msg":""
};

resp.msg += "UUID: " + uuid.get();

$r.response.headers["content-type"] = "application/json";
$r.response.body = JSON.stringify(resp);
