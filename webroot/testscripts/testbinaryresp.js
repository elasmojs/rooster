var fs = require('fs');

var img = fs.read('spotlight.png');
if(img != null){
    if(fs.create('out.png')){
        var copyRes = fs.write('out.png', img);
        if(copyRes){
            var imgCopy = fs.read('out.png');
            if(imgCopy != null){
                fs.remove('out.png');
                $r.response.headers["content-type"] = "image/png";
                $r.response.body = imgCopy;
            }else{
                sendError("Could not read image copy");
            }
        }else{
            sendError("Could not write to file");
        }
    }else{
        sendError('Could not create file to copy');
    }
}else{
    sendError("Could not read file");
}

function sendError(errMsg){
    $r.response.headers["content-type"] = "application/json";
    var err = {
        code: 500,
        msg: errMsg
    }
    $r.response.body = JSON.stringify(err);
}
