var zip = require('zip');

var resp = {
    "code": 200,
    "msg":""
};

var zipSrcPath = "testzip.zip";
var zipExtractPath = "zipout";
var zipFilePath = "out.zip";

if (zip.extract(zipSrcPath, zipExtractPath)){
    if (zip.create(zipExtractPath, zipFilePath)){
        resp.msg = "Zip I/O tested successfully!";
    }else{
        resp.code = 500;
        resp.msg = "Error creating zip file";    
    }
}else{
    resp.code = 500;
    resp.msg = "Error extracting zip file";
}


$r.response.headers["content-type"] = "application/json";
$r.response.body = JSON.stringify(resp);
