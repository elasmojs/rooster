var uuid = require('uuid');

var resp = {
    "code": 200,
    "msg":""
};

resp.msg += "UUID: " + uuid.get();

$g.response.headers["content-type"] = "application/json";
$g.response.body = JSON.stringify(resp);
