var moment = require('./lib/moment.js');
var mathlib = require('./lib/math.js');
var tinycolor = require('./lib/tinycolor.js');
var lodash = require('./lib/lodash-core.js');
var handlebars = require('./lib/handlebars.js');
var collectjs = require('./lib/collect.js');
var sugar = require('./lib/sugar-es5.js');
var underscore = require('./lib/underscore-min.js');
var voca = require('./lib/voca.js');
var datefns = require('./lib/date_fns.min.js');
var marked = require('./lib/marked.js');
var mime = require('./lib/mime.min.js');

var results = {};
try{
        results.tinycolor = "Tinycolor: Hex of red color: " + tinycolor("red").toHexString();
        results.moment = "Moment: " + moment("20111031", "YYYYMMDD").fromNow();
        results.mathjs = "MathJS: sqrt(-4) = " + mathlib.sqrt(-4);

        var array = [1];
        var other = lodash.concat(array, 2, [3], [[4]]);
        
        results.lodash = "Lodash: " + other;
        // => [1, 2, 3, [4]]
        
        results.lodash2 = "Lodash: " + array;
        // => [1]

        //console.log(typeof hand)
        var template = handlebars.compile("Handlebars <b>{{doesWhat}}</b>");
        // execute the compiled template and print the output to the console
        results.handlebars = "Handlebars: " + template({ doesWhat: "rocks!" });
    
       var collection = collect([
            {
              name: 'My story',
              pages: 176,
            },
            {
              name: 'Fantastic Beasts and Where to Find Them',
              pages: 1096,
            },
          ]);
         
        results.collectjs = "Collect JS: " + collection.avg('pages');
          
        results.sugarjs = "Sugar JS: " + sugar.Number.round(3.1415);

        results.underscorejs = "Underscore JS: " + underscore.map([1, 2, 3], function(num){ return num * 3; });

        results.vocajs = "Voca JS: " + voca.sprintf('%s costs $%.2f', 'Tea', 1.5);
        
        results.datefns = "Date-Fns JS: " + datefns.subDays(new Date(), 3);

        results.markedjs = "Marked JS: " + marked('# Marked in browser\n\nRendered by **marked**.');

        results.mimejs = "Mime JS: " + mime.getType('json');
}catch(e){
    results.error = e.toString();
}

JSON.stringify(results);
