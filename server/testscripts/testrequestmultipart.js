var encoder = require('encode');

var req = $g.request;

var resp = {
    code: 200,
    msg: null
}

if(req.isMultipart){
    var parts = req.parts;
    console.log(parts.length);
    for(partName in parts){
        var part = parts[partName];
        if(!part.isText){
            part.data = encoder.base64.encodeBytes(part.data);
        }
        $g.request.parts[part.name] = part;
    }
    resp.msg = req;
}else{
    resp.code = 500;
    resp.msg = "Invalid multi part request"
}


$g.response.headers["content-type"] = "application/json";
$g.response.body = JSON.stringify(resp, true);

