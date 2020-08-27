var fs = require('fs');

var resp = {
    "code": 200,
    "msg":""
};

var fpath = "test.txt";
if (fs.create(fpath)){
    if (fs.writeText(fpath, "Hello World!")){
        var str = fs.readText(fpath);
        if (fs.appendText(fpath, "How are you today?")){
            var str2 = fs.readText(fpath);

            if (fs.remove(fpath)){
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

$r.response.headers["content-type"] = "application/json";
$r.response.body = JSON.stringify(resp);
