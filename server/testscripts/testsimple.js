var a = 5;
var b = 5;

var msg = "Hello world! Sum: " + (a + b);
console.log(msg);


var resp = {
    code: 200,
    msg: msg
}

$g.response.headers["content-type"] = "application/json";
$g.response.body = JSON.stringify(resp);
