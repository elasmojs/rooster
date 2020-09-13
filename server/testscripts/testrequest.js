//echo the request object back as response
$g.response.headers["content-type"] = "application/json";
$g.response.headers["custom-header"] = "custom value";
$g.response.body = JSON.stringify($g.request, true);

