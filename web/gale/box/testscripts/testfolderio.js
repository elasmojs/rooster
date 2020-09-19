var fs = require('fs');

var resp = {
    "code": 200,
    "msg":{}
};

var fpath = "foldertest";
if (fs.createDir(fs.WEB, fpath)){
    if(fs.removeDir(fs.WEB, fpath)){
        fpath = "foldertest/nextfolder/superfolder";
        if(fs.createDirAll(fs.WEB, fpath)){
            if(fs.removeDirAll(fs.WEB, "foldertest")){
                resp.msg.io = "All OK!";
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
    resp.msg.ioerror = "Could not create folder!";
}

resp.msg.list = fs.list(fs.WEB, ".");
resp.msg.listAll = fs.listAll(fs.WEB, ".");
resp.msg.listFiles = fs.listFiles(fs.WEB, ".");
resp.msg.listDirs = fs.listDirs(fs.WEB, ".");

$g.response.headers["content-type"] = "application/json";
$g.response.body = JSON.stringify(resp);
