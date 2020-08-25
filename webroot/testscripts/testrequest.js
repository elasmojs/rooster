//echo the request object back as response
$r.response.headers["content-type"] = "application/json";
$r.response.headers["custom-header"] = "custom value";
$r.response.body = JSON.stringify($r.request, true);

