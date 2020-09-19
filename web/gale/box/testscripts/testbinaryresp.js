var fs = require('fs');

var img = fs.read(fs.BOX, 'spotlight.png');
if(img != null){
    if(fs.create(fs.BOX, 'out.png')){
        var copyRes = fs.write(fs.BOX, 'out.png', img);
        if(copyRes){
            var imgCopy = fs.read(fs.BOX, 'out.png');
            if(imgCopy != null){
                fs.remove(fs.BOX, 'out.png');
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
