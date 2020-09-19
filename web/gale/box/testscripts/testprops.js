var props = require('props');

var props_str = props.get("testscripts/testprops.json");

var resp = {
    code: 200,
    msg: ""
}

if(props_str != null){
    var props_obj = JSON.parse(props_str);
    resp.msg = props_obj;
}else{
    resp.code = 500;
    resp.msg = "Could not get properties file";
}

$g.response.headers["content-type"] = "application/json";
$g.response.body = JSON.stringify(resp);
