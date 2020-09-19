var crypto = require('crypto');

var resp = {
    "code": 200,
    "msg":""
};

var inputStr = "Gale JS Whooshhh!";

resp.msg += "Input: " + inputStr;
resp.msg += ", CRC32: " + crypto.md5(inputStr);
resp.msg += ", MD5: " + crypto.md5(inputStr);
resp.msg += ", SHA2: " + crypto.sha2(inputStr);
resp.msg += ", SHA3: " + crypto.sha3(inputStr);

$g.response.headers["content-type"] = "application/json";
$g.response.body = JSON.stringify(resp);
