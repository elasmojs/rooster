var html = require('html');

var resp = {
    "code": 200,
    "msg":{}
};

$r.response.headers["content-type"] = "application/json";

var url = 'https://news.ycombinator.com/'; 

var doc = html.fromURL(url);
if(doc != null){
    var elems = doc.select(".athing td.title a");
    if(elems != null && elems.length > 0){
        var resp = '<!DOCTYPE html><html><head><meta charset="utf-8"/></head><body><h1>Results from URL scrape</h1><h2>Hacker News</h2><ul>';
        for(var i=0;i<elems.length;i++){
            if(elems[i] != null)
                resp += '<li>' + elems[i].html() + '</li>';
        }
        resp += '</ul></body></html>';
        
        $r.response.headers["content-type"] = "text/html";
        $r.response.body = resp;
    }else{
        resp.msg = "Could not find selection";
        $r.response.body = JSON.stringify(resp, true);
    }
}else{
    resp.code = 500;
    resp.msg = "Could not parse HTML from URL";
    $r.response.body = JSON.stringify(resp, true);
}



