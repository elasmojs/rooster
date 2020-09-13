var fs = require('fs');

var img = fs.read('spotlight.png');
if(img != null){
    if(fs.create('out.png')){
        var copyRes = fs.write('out.png', img);
        if(copyRes){
            var imgCopy = fs.read('out.png');
            if(imgCopy != null){
                fs.remove('out.png');
                $g.response.headers["content-type"] = "image/png";
                $g.response.body = imgCopy;
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
    $g.response.headers["content-type"] = "application/json";
    var err = {
        code: 500,
        msg: errMsg
    }
    $g.response.body = JSON.stringify(err);
}
