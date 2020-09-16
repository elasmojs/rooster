var fs = require('fs');

var resp = {
    "code": 200,
    "msg":""
};

var fpath = "test.txt";
if (fs.create(fs.DATA_SPACE, fpath)){
    if (fs.writeText(fs.DATA_SPACE, fpath, "Hello World!")){
        var str = fs.readText(fs.DATA_SPACE, fpath);
        if (fs.appendText(fs.DATA_SPACE, fpath, "How are you today?")){
            var str2 = fs.readText(fs.DATA_SPACE, fpath);

            if (fs.remove(fs.DATA_SPACE, fpath)){
                resp.msg = "Data Read: " + str + " ,  Data after append: " + str2 + " , file removed successfully";
            }else{
                resp.code = 500;
                resp.msg = "Could not create file!";
            }
        }else{
            resp.code = 500;
            resp.msg = "Could not create file!";    
        }
    }else{
        resp.code = 500;
        resp.msg = "Could not create file!";
    }
}else{
    resp.code = 500;
    resp.msg = "Could not create file!";
}

$g.response.headers["content-type"] = "application/json";
$g.response.body = JSON.stringify(resp);
