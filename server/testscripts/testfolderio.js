var fs = require('fs');

var resp = {
    "code": 200,
    "msg":""
};

var fpath = "testfolder";
if (fs.createDir(fpath)){
    if(fs.removeDir(fpath)){
        fpath = "testfolder/nextfolder/superfolder";
        if(fs.createDirAll(fpath)){
            if(fs.removeDirAll("testfolder")){
                resp.msg = "Successfully tested!";
            }else{
                resp.code = 500;
                resp.msg = "Could not remove folder tree!";
            }
        }else{
            resp.code = 500;
            resp.msg = "Could not create folder tree!";
        }
    }else{
        resp.code = 500;
        resp.msg = "Could not remove folder!";    
    }
}else{
    resp.code = 500;
    resp.msg = "Could not create folder!";
}

$g.response.headers["content-type"] = "application/json";
$g.response.body = JSON.stringify(resp);
