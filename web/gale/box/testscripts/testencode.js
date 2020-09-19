var encoder = require('encode');

var resp = {
    "code": 200,
    "msg":""
};

var inputStr = "Gale JS!";
var b64Encode = encoder.base64.encode(inputStr);
var b64Decode = encoder.base64.decode(b64Encode);

var inputURI = 'https://mozilla.org/?x=шеллы';
var uriEncode = encoder.url.encode(inputURI);
var uriDecode = encoder.url.decode(uriEncode);

resp.msg += "Input: " + inputStr;
resp.msg += ", Encode: " + b64Encode;
resp.msg += ", Decode: " + b64Decode;

resp.msg += "URI Input: " + inputURI;
resp.msg += ", Encode: " + uriEncode;
resp.msg += ", Decode: " + uriDecode;

$g.response.headers["content-type"] = "application/json";
$g.response.body = JSON.stringify(resp);
