var fs = require('fs');

var resp = fs.readText("testfolder/appref.html");
if(resp == null)
    resp = "Could not read file";

$g.response.headers["content-type"] = "text/html";
$g.response.body = resp;
