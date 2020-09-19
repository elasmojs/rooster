var fs = require('fs');

var resp = {
    "code": 200,
    "msg":""
};

var fpath = "test.txt";
if (fs.create(fs.BOX, fpath)){
    if (fs.writeText(fs.BOX, fpath, "Hello World!")){
        var str = fs.readText(fs.BOX, fpath);
        if (fs.appendText(fs.BOX, fpath, "How are you today?")){
            var str2 = fs.readText(fs.BOX, fpath);

            if (fs.remove(fs.BOX, fpath)){
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
