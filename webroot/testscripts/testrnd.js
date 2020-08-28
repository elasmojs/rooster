var rnd = require('rnd');

var resp = {
    "code": 200,
    "msg":""
};

resp.msg += "Random: " + rnd.get();
resp.msg += ", Random Float: " + rnd.float();
resp.msg += ", Range: " + rnd.range(0, 10);

$r.response.headers["content-type"] = "application/json";
$r.response.body = JSON.stringify(resp);
