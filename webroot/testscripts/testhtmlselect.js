var html = require('html');

var resp = {
    "code": 200,
    "msg":{}
};

var htmlStr = '<html><body><div class="container"><ul myattr="My Attribute" attr2="One more Attribute" class="list numberlist"><li>One</li><li>Two</li><li>Three</li></ul></div></body></html>'; 

var doc = html.fromString(htmlStr);
if(doc != null){    
    var results = {};
    results.rootElement = doc.rootElement().html();
    
    var divElems = doc.select("div ul");
    if(divElems != null){
        
        var elem = divElems[0];

        //name
        results.name = elem.name();
        
        //html
        results.html = elem.html();

        var liElems = elem.select("li");
        if(liElems != null){
            results.select2 = []
            for(var i=0;i<liElems.length;i++)
                results.select2.push(liElems[i].text());
        }else
            results.select2 = null;
        
        
        //innerHtml()
        results.innerHTML = elem.innerHtml();
        
        
        //text()
        results.text = elem.text();
        
        
        //ancestors()
        var ancestorsArr = [];
        var ancestors = elem.ancestors();
        if(ancestors != null){
            for(var i=0; i<ancestors.length;i++){
                if(ancestors[i] != null)
                    ancestorsArr.push(ancestors[i].html());
                else
                    ancestorsArr.push(null);
            }
            results.ancestors = ancestorsArr;
        }else{
            results.ancestors = null;
        }

        
        //children()
        var childArr = [];
        var children = elem.children();
        if(children != null){
            for(var i=0; i<children.length;i++){
                if(children[i] != null)
                    childArr.push(children[i].html());
                else
                    childArr.push(null);
            }
            results.children = childArr;
        }else{
            results.children = null;
        }

        
        //descendants()
        var descendantsArr = [];
        var descendants = elem.descendants();
        if(descendants != null){
            for(var i=0; i<descendants.length;i++){
                if(descendants[i] != null)
                    descendantsArr.push(descendants[i].html());
                else
                    descendantsArr.push(null);
            }
            results.descendants = descendantsArr;
        }else{
            results.descendants = null;
        }

        
        //firstChild()
        results.firstChild = elem.firstChild() != null? elem.firstChild().html(): null;

        
        //firstChildren()
        var firstChildArr = [];
        var firstChildren = elem.firstChildren();
        if(firstChildren != null){
            for(var i=0; i<firstChildren.length;i++){
                if(firstChildren[i] != null)
                    firstChildArr.push(firstChildren[i].text());
                else
                    firstChildArr.push(null);
            }
            results.firstChildren = firstChildArr;
        }else{
            results.firstChildren = null;
        }

        
        //hasChildren()
        results.hasChildren = elem.hasChildren();

        
        //hasSiblings()
        results.hasSiblings = elem.hasSiblings();

        
        //lastChild()
        results.lastChild = elem.lastChild() != null? elem.lastChild().html(): null;

        
        //lastChildren()
        var lastChildArr = [];
        var lastChildren = elem.lastChildren();
        if(lastChildren != null){
            for(var i=0; i<lastChildren.length;i++){
                if(lastChildren[i] != null)
                    lastChildArr.push(lastChildren[i].html());
                else
                    lastChildArr.push(null);
            }
            results.lastChildren = lastChildArr;
        }else{
            results.lastChildren = null;
        }

        
        //nextSibling()
        results.nextSibling = elem.firstChild().nextSibling() != null? elem.firstChild().nextSibling().html() : null;
        
        //nextSiblings()
        var nextSiblingsArr = [];
        var nextSiblings = elem.firstChild().nextSiblings();
        if(nextSiblings != null){
            for(var i=0; i<nextSiblings.length;i++){
                if(nextSiblings[i] != null)
                    nextSiblingsArr.push(nextSiblings[i].html());
                else
                    nextSiblingsArr.push(null);
            }
            results.nextSiblings = nextSiblingsArr;
        }else{
            results.nextSiblings = null;
        }
        
        //prevSibling()
        results.prevSibling = elem.lastChild().prevSibling() != null? elem.lastChild().prevSibling().html() : null;

        //prevSiblings()
        var prevSiblingsArr = [];
        var prevSiblings = elem.lastChild().prevSiblings();
        if(prevSiblings != null){
            for(var i=0; i<prevSiblings.length;i++){
                if(prevSiblings[i] != null)
                    prevSiblingsArr.push(prevSiblings[i].html());
                else
                    prevSiblingsArr.push(null);
            }
            results.prevSiblings = prevSiblingsArr;
        }else{
            results.prevSiblings = null;
        }

        //attr()
        results.attr = elem.attr("myattr");

        //attrs()
        results.attrs = elem.attrs();

        //hasClass()
        results.hasClass = elem.hasClass("list");

        //attrs()
        results.classes = elem.classes();
    }
    resp.code = 200;
    resp.msg = results;
}else{
    resp.code = 500;
    resp.msg = "Could not parse HTML";
}

$g.response.headers["content-type"] = "application/json";
$g.response.body = JSON.stringify(resp, true);



