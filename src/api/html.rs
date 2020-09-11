use ducc::{Ducc, Object, Array, Invocation, Value, Error as DuccError};

use std::fs::File;
use std::io::Read;
use std::cell::RefCell;
use std::collections::HashMap;
use ureq;
use uuid::Uuid;

use ego_tree::{NodeId, NodeRef};
use selectors::attr::CaseSensitivity;
use scraper::{Html, Selector, ElementRef, Node};
use log::*;

pub const API_KEY:&str = "api";
pub const ROOSTER_KEY:&str = "_rooster";
pub const DATA_ROOT_KEY:&str = "dr"; 
pub const HTML_API:&str = "html";

pub const HTML_REF_MAP:&str = "html_ref_map";
pub const ELEM_REF_MAP:&str = "elem_ref_map";
pub const ID:&str = "id";
pub const DOC_ID:&str = "docId";
pub const IS_DOC:&str = "isDoc";

pub const HAPI_FROM_STRING:&str = "fromString";
pub const HAPI_FROM_FILE:&str = "fromFile";
pub const HAPI_FROM_URL:&str = "fromURL";

pub const HAPI_ROOT_ELEM:&str = "rootElement";
pub const HAPI_SELECT:&str = "select";
pub const HAPI_HTML:&str = "html";
pub const HAPI_INNER_HTML:&str = "innerHtml";
pub const HAPI_TEXT:&str = "text";
pub const HAPI_ANCESTORS:&str = "ancestors";
pub const HAPI_CHILDREN:&str = "children";
pub const HAPI_DESCENDANTS:&str = "descendants";
pub const HAPI_FIRST_CHILD:&str = "firstChild";
pub const HAPI_FIRST_CHILDREN:&str = "firstChildren";
pub const HAPI_HAS_CHILDREN:&str = "hasChildren";
pub const HAPI_HAS_SIBLINGS:&str = "hasSiblings";
pub const HAPI_LAST_CHILD:&str = "lastChild";
pub const HAPI_LAST_CHILDREN:&str = "lastChildren";
pub const HAPI_NEXT_SIBLING:&str = "nextSibling";
pub const HAPI_NEXT_SIBLINGS:&str = "nextSiblings";
pub const HAPI_PREV_SIBLING:&str = "prevSibling";
pub const HAPI_PREV_SIBLINGS:&str = "prevSiblings";
pub const HAPI_ATTR:&str = "attr";
pub const HAPI_ATTRS:&str = "attrs";
pub const HAPI_HAS_CLASS:&str = "hasClass";
pub const HAPI_CLASSES:&str = "classes";
pub const HAPI_NAME:&str = "name";


pub fn set_user_data(engine:&mut Ducc){
    let html_ref_map:HashMap<String, Html> = HashMap::new();
    engine.set_user_data(HTML_REF_MAP, RefCell::new(html_ref_map));

    let elem_ref_map:HashMap<String, NodeId> = HashMap::new();
    engine.set_user_data(ELEM_REF_MAP, RefCell::new(elem_ref_map));
}

pub fn load(engine:&Ducc) -> bool{
    let api_res:Result<Value, _> = engine.globals().get(API_KEY);
    let api_obj = api_res.unwrap();                
    let api = api_obj.as_object().unwrap();

    let html_select = engine.create_object();
    html_select.set(HAPI_FROM_STRING, engine.create_function(html_from_string)).unwrap();
    html_select.set(HAPI_FROM_FILE, engine.create_function(html_from_file)).unwrap();
    html_select.set(HAPI_FROM_URL, engine.create_function(html_from_url)).unwrap();

    api.set(HTML_API, html_select).unwrap();

    return true;
}

fn html_from_string(inv: Invocation) -> Result<Value, DuccError>{
    let engine = inv.ducc;
    let args = inv.args;
    if args.len() == 1{
        let html_res = args.get(0);

        if html_res.is_string(){
            let html = html_res.as_string().unwrap().to_string().unwrap();
            let html_str = html.as_str();
            let html_doc = Html::parse_document(html_str);
                        
            let doc_id = Uuid::new_v4().to_string();
            let eref_map:&RefCell<HashMap<String, Html>> = engine.get_user_data(HTML_REF_MAP).unwrap();
            eref_map.replace_with(|map|{
                map.insert(doc_id.clone(), html_doc);
                return map.to_owned();
            });

            let doc_obj = engine.create_object();
            doc_obj.set(DOC_ID, doc_id.clone()).unwrap();
            doc_obj.set(IS_DOC, true).unwrap();
            
            let root_elem_fn = engine.create_function(root_elem_fn);
            doc_obj.set(HAPI_ROOT_ELEM, root_elem_fn).unwrap();
            let select_fn = engine.create_function(select);
            doc_obj.set(HAPI_SELECT, select_fn).unwrap();
            
            return Ok(Value::Object(doc_obj));
            
        }else{
            error!("Invalid argument for html fromString, expected string");
            return Ok(Value::Null);    
        }
    }else{
        error!("Invalid argument for fromString, expected 1 argument");
        return Ok(Value::Null);
    }
}

fn html_from_file(inv: Invocation) -> Result<Value, DuccError>{
    let engine = inv.ducc;
    let args = inv.args;
    if args.len() == 1{
        let robj:Object = engine.globals().get(ROOSTER_KEY).unwrap();
        let data_root:String = robj.get(DATA_ROOT_KEY).unwrap();

        let fpath_res = args.get(0);
        if fpath_res.is_string(){
            let fpath = format!("{}/{}", data_root, fpath_res.as_string().unwrap().to_string().unwrap());
            let file = File::open(&fpath);
            if file.is_ok(){
                let mut html_str = String::new();
                let read_res = file.unwrap().read_to_string(&mut html_str);
                if read_res.is_ok(){
                    let html_doc = Html::parse_document(html_str.as_str());
                    let doc_id = Uuid::new_v4().to_string();
                    let eref_map:&RefCell<HashMap<String, Html>> = engine.get_user_data(HTML_REF_MAP).unwrap();
                    eref_map.replace_with(|map|{
                        map.insert(doc_id.clone(), html_doc);
                        return map.to_owned();
                    });

                    let doc_obj = engine.create_object();
                    doc_obj.set(DOC_ID, doc_id.clone()).unwrap();
                    doc_obj.set(IS_DOC, true).unwrap();
                    
                    let root_elem_fn = engine.create_function(root_elem_fn);
                    doc_obj.set(HAPI_ROOT_ELEM, root_elem_fn).unwrap();
                    let select_fn = engine.create_function(select);
                    doc_obj.set(HAPI_SELECT, select_fn).unwrap();
                    
                    return Ok(Value::Object(doc_obj));
                }else{
                    error!("Could not read file during html fromFile API call");
                    return Ok(Value::Null);
                }
            }else{
                error!("Could not open file during html fromFile API call");
                return Ok(Value::Null);
            }
        }else{
            error!("Invalid argument for html fromFile API call, expected String");
            return Ok(Value::Null);
        }
    }else{
        error!("Invalid argument for html fromFile, expected 1 argument");
        return Ok(Value::Null);
    }
}

fn html_from_url(inv: Invocation) -> Result<Value, DuccError>{
    let engine = inv.ducc;
    let args = inv.args;
    if args.len() == 1{
        let url_res = args.get(0);
        if url_res.is_string(){
            let url = url_res.as_string().unwrap().to_string().unwrap();
            let mut request = ureq::get(url.as_str());
            let url_resp = request.call();
            let resp = url_resp.into_string();

            if resp.is_ok(){
                let html_doc = Html::parse_document(resp.unwrap().clone().as_str());
                let doc_id = Uuid::new_v4().to_string();
                let eref_map:&RefCell<HashMap<String, Html>> = engine.get_user_data(HTML_REF_MAP).unwrap();
                eref_map.replace_with(|map|{
                    map.insert(doc_id.clone(), html_doc);
                    return map.to_owned();
                });

                let doc_obj = engine.create_object();
                doc_obj.set(DOC_ID, doc_id.clone()).unwrap();
                doc_obj.set(IS_DOC, true).unwrap();
                
                let root_elem_fn = engine.create_function(root_elem_fn);
                doc_obj.set(HAPI_ROOT_ELEM, root_elem_fn).unwrap();
                let select_fn = engine.create_function(select);
                doc_obj.set(HAPI_SELECT, select_fn).unwrap();
                
                return Ok(Value::Object(doc_obj));
            }else{
                error!("Could not open file during html fromURL API call");
                return Ok(Value::Null);
            }
        }else{
            error!("Invalid argument for html fromURL API call, expected String");
            return Ok(Value::Null);
        }
    }else{
        error!("Invalid argument for html fromURL, expected 1 argument");
        return Ok(Value::Null);
    }
}

fn root_elem_fn(inv: Invocation) -> Result<Value, DuccError>{
    let engine = inv.ducc;
    let this = inv.this.as_object().unwrap();

    let doc_id:String = this.get(DOC_ID).unwrap();
    let doc_ref_map:&RefCell<HashMap<String, Html>> = engine.get_user_data(HTML_REF_MAP).unwrap();
    let doc_map = doc_ref_map.borrow();
    let doc = doc_map.get(&doc_id).unwrap();

    let elem = doc.root_element();
    
    let eid = Uuid::new_v4().to_string();
    let elem_obj = engine.create_object();
    elem_obj.set(DOC_ID, doc_id.clone()).unwrap();
    elem_obj.set(ID, eid.clone()).unwrap();
    let select_fn = engine.create_function(select);
    elem_obj.set(HAPI_SELECT, select_fn).unwrap();
    init_elem_obj(engine, &elem_obj);
    
    let ref_map:&RefCell<HashMap<String, NodeId>> = engine.get_user_data(ELEM_REF_MAP).unwrap();
    let mut elem_map = ref_map.borrow_mut();
    elem_map.insert(eid.clone(), elem.id());
    
    return Ok(Value::Object(elem_obj));
}

fn select(inv: Invocation) -> Result<Value, DuccError>{
    let engine = inv.ducc;
    let args = inv.args;
    let this = inv.this.as_object().unwrap();

    if args.len() == 1{
        let select_res = args.get(0);
        if select_res.is_string(){
            let select_str = select_res.as_string().unwrap().to_string().unwrap();
            
            if this.contains_key(ID).unwrap(){
                let id:String = this.get(ID).unwrap();
                let doc_id:String = this.get(DOC_ID).unwrap();
                
                let doc_ref_map:&RefCell<HashMap<String, Html>> = engine.get_user_data(HTML_REF_MAP).unwrap();
                let doc_map = doc_ref_map.borrow();
                let doc_opt = doc_map.get(&doc_id);
                if doc_opt.is_none(){
                    return Ok(Value::Null);
                }

                let doc:&Html = doc_opt.unwrap();

                let ref_map:&RefCell<HashMap<String, NodeId>> = engine.get_user_data(ELEM_REF_MAP).unwrap();
                let mut elem_map = ref_map.borrow_mut();
                let node_id_res = elem_map.get(&id);
                
                if node_id_res.is_some(){
                    let node_id = node_id_res.unwrap();
                    let node_ref_opt = doc.tree.get(*node_id);
                    if node_ref_opt.is_none(){
                        return Ok(Value::Null);
                    }
                    let elem_ref = ElementRef::wrap(node_ref_opt.unwrap()).unwrap();
        
                    let sel_res = Selector::parse(select_str.as_str());
                    if sel_res.is_err(){
                        return Ok(Value::Null);
                    }
                    
                    let selection = sel_res.unwrap();
                    let selected = elem_ref.select(&selection);
                    let elems = selected.collect::<Vec<ElementRef>>();

                    let elems_arr = engine.create_array();
                    let mut new_elem_ref_hash:HashMap<String, NodeId> = HashMap::new();
                    init_elems(engine, elems, doc_id, &elems_arr, &mut new_elem_ref_hash);
                    elem_map.extend(new_elem_ref_hash);

                    return Ok(Value::Array(elems_arr));
                }else{
                    return Ok(Value::Null);
                }
            }else if this.contains_key(IS_DOC).unwrap(){
                let doc_id:String = this.get(DOC_ID).unwrap();
                let doc_ref_map:&RefCell<HashMap<String, Html>> = engine.get_user_data(HTML_REF_MAP).unwrap();
                let doc_map = doc_ref_map.borrow();
                let doc = doc_map.get(&doc_id).unwrap();
                
                let sel_res = Selector::parse(select_str.as_str());
                if sel_res.is_err(){
                    return Ok(Value::Null);
                }
                let selection = sel_res.unwrap();
                let selected = doc.select(&selection);
                let elems = selected.collect::<Vec<ElementRef>>();
                
                let elems_arr = engine.create_array();
                let mut new_elem_ref_hash:HashMap<String, NodeId> = HashMap::new();
                init_elems(engine, elems, doc_id, &elems_arr, &mut new_elem_ref_hash);
                
                let eref_map:&RefCell<HashMap<String, NodeId>> = engine.get_user_data(ELEM_REF_MAP).unwrap();
                eref_map.replace_with(|map|{
                    map.extend(new_elem_ref_hash);
                    return map.to_owned();
                });
                return Ok(Value::Array(elems_arr));
                
            }else{
                return Ok(Value::Null);
            }
        }else{
            error!("Invalid argument for html select, expected string");
            return Ok(Value::Null);    
        }
    }else{
        error!("Invalid argument for html select, expected 1 argument");
        return Ok(Value::Null);
    }
}

fn init_elems(engine:&Ducc, elems:Vec<ElementRef>, doc_id:String, elems_arr:&Array, new_elem_ref_hash:&mut HashMap<String, NodeId>){
    for elem in elems{
        let eid = Uuid::new_v4().to_string();
        
        let elem_obj = engine.create_object();
        elem_obj.set(DOC_ID, doc_id.clone()).unwrap();
        elem_obj.set(ID, eid.clone()).unwrap();
        let select_fn = engine.create_function(select);
        elem_obj.set(HAPI_SELECT, select_fn).unwrap();
        init_elem_obj(engine, &elem_obj);
        elems_arr.push(elem_obj).unwrap();

        new_elem_ref_hash.insert(eid.clone(), elem.clone().id());
    }
}

fn init_elem_obj(engine:&Ducc, elem_obj:&Object){
    let elem_html_fn = engine.create_function(elem_html_fn);
    elem_obj.set(HAPI_HTML, elem_html_fn).unwrap();

    let elem_innerhtml_fn = engine.create_function(elem_innerhtml_fn);
    elem_obj.set(HAPI_INNER_HTML, elem_innerhtml_fn).unwrap();

    let elem_text_fn = engine.create_function(elem_text_fn);
    elem_obj.set(HAPI_TEXT, elem_text_fn).unwrap();

    let elem_ancestors_fn = engine.create_function(elem_ancestors_fn);
    elem_obj.set(HAPI_ANCESTORS, elem_ancestors_fn).unwrap();

    let elem_children_fn = engine.create_function(elem_children_fn);
    elem_obj.set(HAPI_CHILDREN, elem_children_fn).unwrap();

    let elem_descendants_fn = engine.create_function(elem_descendants_fn);
    elem_obj.set(HAPI_DESCENDANTS, elem_descendants_fn).unwrap();

    let elem_first_child_fn = engine.create_function(elem_first_child_fn);
    elem_obj.set(HAPI_FIRST_CHILD, elem_first_child_fn).unwrap();

    let elem_first_children_fn = engine.create_function(elem_first_children_fn);
    elem_obj.set(HAPI_FIRST_CHILDREN, elem_first_children_fn).unwrap();

    let elem_has_children_fn = engine.create_function(elem_has_children_fn);
    elem_obj.set(HAPI_HAS_CHILDREN, elem_has_children_fn).unwrap();

    let elem_has_siblings_fn = engine.create_function(elem_has_siblings_fn);
    elem_obj.set(HAPI_HAS_SIBLINGS, elem_has_siblings_fn).unwrap();

    let elem_last_child_fn = engine.create_function(elem_last_child_fn);
    elem_obj.set(HAPI_LAST_CHILD, elem_last_child_fn).unwrap();

    let elem_last_children_fn = engine.create_function(elem_last_children_fn);
    elem_obj.set(HAPI_LAST_CHILDREN, elem_last_children_fn).unwrap();

    let elem_next_sibling_fn = engine.create_function(elem_next_sibling_fn);
    elem_obj.set(HAPI_NEXT_SIBLING, elem_next_sibling_fn).unwrap();

    let elem_next_siblings_fn = engine.create_function(elem_next_siblings_fn);
    elem_obj.set(HAPI_NEXT_SIBLINGS, elem_next_siblings_fn).unwrap();

    let elem_prev_sibling_fn = engine.create_function(elem_prev_sibling_fn);
    elem_obj.set(HAPI_PREV_SIBLING, elem_prev_sibling_fn).unwrap();

    let elem_prev_siblings_fn = engine.create_function(elem_prev_siblings_fn);
    elem_obj.set(HAPI_PREV_SIBLINGS, elem_prev_siblings_fn).unwrap();

    let elem_attr_fn = engine.create_function(elem_attr_fn);
    elem_obj.set(HAPI_ATTR, elem_attr_fn).unwrap();

    let elem_attrs_fn = engine.create_function(elem_attrs_fn);
    elem_obj.set(HAPI_ATTRS, elem_attrs_fn).unwrap();

    let elem_has_class_fn = engine.create_function(elem_has_class_fn);
    elem_obj.set(HAPI_HAS_CLASS, elem_has_class_fn).unwrap();

    let elem_classes_fn = engine.create_function(elem_classes_fn);
    elem_obj.set(HAPI_CLASSES, elem_classes_fn).unwrap();

    let elem_name_fn = engine.create_function(elem_name_fn);
    elem_obj.set(HAPI_NAME, elem_name_fn).unwrap();
}

fn elem_html_fn(inv: Invocation) -> Result<Value, DuccError>{
    return process(inv, HAPI_HTML.to_string());
}

fn elem_innerhtml_fn(inv: Invocation) -> Result<Value, DuccError>{
    return process(inv, HAPI_INNER_HTML.to_string());
}

fn elem_text_fn(inv: Invocation) -> Result<Value, DuccError>{
    return process(inv, HAPI_TEXT.to_string());
}

fn elem_ancestors_fn(inv: Invocation) -> Result<Value, DuccError>{
    return process_multiple(inv, HAPI_ANCESTORS.to_string());
}

fn elem_children_fn(inv: Invocation) -> Result<Value, DuccError>{
    return process_multiple(inv, HAPI_CHILDREN.to_string());
}

fn elem_descendants_fn(inv: Invocation) -> Result<Value, DuccError>{
    return process_multiple(inv, HAPI_DESCENDANTS.to_string());
}

fn elem_first_child_fn(inv: Invocation) -> Result<Value, DuccError>{
    return process_single(inv, HAPI_FIRST_CHILD.to_string());
}

fn elem_first_children_fn(inv: Invocation) -> Result<Value, DuccError>{
    return process_multiple(inv, HAPI_FIRST_CHILDREN.to_string());
}

fn elem_has_children_fn(inv: Invocation) -> Result<Value, DuccError>{
    return process(inv, HAPI_HAS_CHILDREN.to_string());
}

fn elem_has_siblings_fn(inv: Invocation) -> Result<Value, DuccError>{
    return process(inv, HAPI_HAS_SIBLINGS.to_string());
}

fn elem_last_child_fn(inv: Invocation) -> Result<Value, DuccError>{
    return process_single(inv, HAPI_LAST_CHILD.to_string());
}

fn elem_last_children_fn(inv: Invocation) -> Result<Value, DuccError>{
    return process_multiple(inv, HAPI_LAST_CHILDREN.to_string());
}

fn elem_next_sibling_fn(inv: Invocation) -> Result<Value, DuccError>{
    return process_single(inv, HAPI_NEXT_SIBLING.to_string());
}

fn elem_next_siblings_fn(inv: Invocation) -> Result<Value, DuccError>{
    return process_multiple(inv, HAPI_NEXT_SIBLINGS.to_string());
}

fn elem_prev_sibling_fn(inv: Invocation) -> Result<Value, DuccError>{
    return process_single(inv, HAPI_PREV_SIBLING.to_string());
}

fn elem_prev_siblings_fn(inv: Invocation) -> Result<Value, DuccError>{
    return process_multiple(inv, HAPI_PREV_SIBLINGS.to_string());
}

fn elem_attr_fn(inv: Invocation) -> Result<Value, DuccError>{
    return process(inv, HAPI_ATTR.to_string());
}

fn elem_attrs_fn(inv: Invocation) -> Result<Value, DuccError>{
    return process(inv, HAPI_ATTRS.to_string());
}

fn elem_has_class_fn(inv: Invocation) -> Result<Value, DuccError>{
    return process(inv, HAPI_HAS_CLASS.to_string());
}

fn elem_classes_fn(inv: Invocation) -> Result<Value, DuccError>{
    return process(inv, HAPI_CLASSES.to_string());
}

fn elem_name_fn(inv: Invocation) -> Result<Value, DuccError>{
    return process(inv, HAPI_NAME.to_string());
}

fn process(inv:Invocation, method:String) -> Result<Value, DuccError>{
    let engine = inv.ducc;
    let this = inv.this.as_object().unwrap();

    if this.contains_key(ID).unwrap(){
        let id:String = this.get(ID).unwrap();
        let doc_id:String = this.get(DOC_ID).unwrap();

        let doc_ref_map:&RefCell<HashMap<String, Html>> = engine.get_user_data(HTML_REF_MAP).unwrap();
        let doc_map = doc_ref_map.borrow();
        let doc_res = doc_map.get(&doc_id);

        if doc_res.is_none(){
            return Ok(Value::Null);
        }
        let doc:&Html = doc_res.unwrap(); 

        let ref_map:&RefCell<HashMap<String, NodeId>> = engine.get_user_data(ELEM_REF_MAP).unwrap();
        let node_map = ref_map.borrow();
        let node_res = node_map.get(&id);
        
        if node_res.is_some(){
            let node_id = node_res.unwrap();
            let node_opt = doc.tree.get(*node_id);
            if node_opt.is_some(){
                let elem = ElementRef::wrap(node_opt.unwrap());
                if elem.is_some(){
                    let elem = elem.unwrap();
                    match method.as_str(){
                        HAPI_NAME => return Ok(Value::String(engine.create_string(&elem.value().name()).unwrap())),
                        HAPI_CLASSES =>{
                            let classes = elem.value().classes();
                            let classes_arr = engine.create_array();
                            for class in classes{
                                classes_arr.push(engine.create_string(class).unwrap()).unwrap();
                            }
                            
                            return Ok(Value::Array(classes_arr));
                        },
                        HAPI_HAS_CLASS => {
                            let args = inv.args;
                            if args.len() != 1{
                                error!("Invalid argument for hasClass method in html select API, expected 1 argument");
                                return Ok(Value::Null);
                            }
                            let class_name_val = args.get(0);
                            if !class_name_val.is_string(){
                                error!("Invalid argument for hasClass method in html select API, expected String");
                                return Ok(Value::Null);
                            }
                            let class_name = class_name_val.as_string().unwrap();
                            return Ok(Value::Boolean(elem.value().has_class(class_name.to_string().unwrap().as_str(), CaseSensitivity::CaseSensitive)));
                        },
                        HAPI_ATTR => {
                            let args = inv.args;

                            if args.len() != 1{
                                error!("Invalid argument for attr method in html select API");
                                return Ok(Value::Null);
                            }
                            let attr_key_val = args.get(0);
                            if !attr_key_val.is_string(){
                                error!("Invalid argument for attr method in html select API");
                                return Ok(Value::Null);
                            }
                            let attr_key = attr_key_val.as_string().unwrap();

                            let attr_opt = elem.value().attr(attr_key.to_string().unwrap().as_str());
                            if attr_opt.is_none(){
                                return Ok(Value::Null);
                            }
                            let attr_val = attr_opt.unwrap();
                            return Ok(Value::String(engine.create_string(attr_val).unwrap()));
                        },
                        HAPI_ATTRS => {
                            let attrs = elem.value().attrs();
                            let attrs_obj = engine.create_object();
                            for attr in attrs{
                                attrs_obj.set(engine.create_string(attr.0).unwrap(), attr.1).unwrap();
                            }
                            
                            return Ok(Value::Object(attrs_obj));
                        },
                        HAPI_HAS_CHILDREN => {
                            return Ok(Value::Boolean(elem.has_children()));
                        },
                        HAPI_HAS_SIBLINGS => {
                            return Ok(Value::Boolean(elem.has_siblings()));
                        },
                        HAPI_HTML => {
                            return Ok(Value::String(engine.create_string(&elem.html()).unwrap()));
                        },
                        HAPI_INNER_HTML => {
                            return Ok(Value::String(engine.create_string(&elem.inner_html()).unwrap()));
                        },
                        HAPI_TEXT => {
                            return Ok(Value::String(engine.create_string(&elem.text().collect::<Vec<_>>().concat()).unwrap()));
                        },
                        _ => return Ok(Value::Null)
                    }
                    
                }else{
                    return Ok(Value::Null);
                }
            }else{
                return Ok(Value::Null);
            }
        }else{
            return Ok(Value::Null);
        }   
    }else{
        return Ok(Value::Null);
    }
}

fn process_single(inv:Invocation, method:String) -> Result<Value, DuccError>{
    let engine = inv.ducc;
    let this = inv.this.as_object().unwrap();

    if this.contains_key(ID).unwrap(){
        let id:String = this.get(ID).unwrap();
        let doc_id:String = this.get(DOC_ID).unwrap();

        let doc_ref_map:&RefCell<HashMap<String, Html>> = engine.get_user_data(HTML_REF_MAP).unwrap();
        let doc_map = doc_ref_map.borrow();
        let doc_res = doc_map.get(&doc_id);

        if doc_res.is_none(){
            return Ok(Value::Null);
        }
        let doc:&Html = doc_res.unwrap(); 

        let ref_map:&RefCell<HashMap<String, NodeId>> = engine.get_user_data(ELEM_REF_MAP).unwrap();
        let mut node_map = ref_map.borrow_mut();
        let node_res = node_map.get(&id);
        
        if node_res.is_some(){
            let node_id = node_res.unwrap();
            let node_opt = doc.tree.get(*node_id);
            if node_opt.is_some(){
                let elem_opt = ElementRef::wrap(node_opt.unwrap());
                if elem_opt.is_none(){
                    return Ok(Value::Null);
                }
                let elem = elem_opt.unwrap();

                let child_opt = match method.as_str(){
                    HAPI_FIRST_CHILD => elem.first_child(),
                    HAPI_LAST_CHILD => elem.last_child(),
                    HAPI_NEXT_SIBLING => elem.next_sibling(),
                    HAPI_PREV_SIBLING => elem.prev_sibling(),
                    _ => None
                };

                if child_opt.is_none(){
                    return Ok(Value::Null);
                }
                let child = child_opt.unwrap();

                let eid = Uuid::new_v4().to_string();                    
                let elem_obj = engine.create_object();
                elem_obj.set(DOC_ID, doc_id.clone()).unwrap();
                elem_obj.set(ID, eid.clone()).unwrap();
                let select_fn = engine.create_function(select);
                elem_obj.set(HAPI_SELECT, select_fn).unwrap();
                init_elem_obj(engine, &elem_obj);

                node_map.insert(eid.clone(), child.clone().id());

                return Ok(Value::Object(elem_obj));
            }else{
                return Ok(Value::Null);
            }
        }else{
            return Ok(Value::Null);
        }   
    }else{
        return Ok(Value::Null);
    }
}

fn process_multiple(inv:Invocation, method:String) -> Result<Value, DuccError>{
    let engine = inv.ducc;
    let this = inv.this.as_object().unwrap();

    if this.contains_key(ID).unwrap(){
        let id:String = this.get(ID).unwrap();
        let doc_id:String = this.get(DOC_ID).unwrap();

        let doc_ref_map:&RefCell<HashMap<String, Html>> = engine.get_user_data(HTML_REF_MAP).unwrap();
        let doc_map = doc_ref_map.borrow();
        let doc_res = doc_map.get(&doc_id);

        if doc_res.is_none(){
            return Ok(Value::Null);
        }
        let doc:&Html = doc_res.unwrap(); 

        let ref_map:&RefCell<HashMap<String, NodeId>> = engine.get_user_data(ELEM_REF_MAP).unwrap();
        let mut node_map = ref_map.borrow_mut();
        let node_res = node_map.get(&id);
        
        if node_res.is_some(){
            let node_id = node_res.unwrap();
            let node_opt = doc.tree.get(*node_id);
            if node_opt.is_some(){
                let elem_opt = ElementRef::wrap(node_opt.unwrap());
                if elem_opt.is_none(){
                    return Ok(Value::Null);
                }
                let elem = elem_opt.unwrap();
                let multiple_opt:Option<Vec<NodeRef<Node>>> = match method.as_str(){
                    HAPI_ANCESTORS => {
                        let elems = elem.ancestors();
                        let mut nodevec:Vec<NodeRef<Node>> = Vec::new();
                        for node in elems{
                            nodevec.push(node);
                        }
                        Some(nodevec)
                    },HAPI_CHILDREN => {
                        let elems = elem.children();
                        let mut nodevec:Vec<NodeRef<Node>> = Vec::new();
                        for node in elems{
                            nodevec.push(node);
                        }
                        Some(nodevec)
                    },HAPI_DESCENDANTS => {
                        let elems = elem.descendants();
                        let mut nodevec:Vec<NodeRef<Node>> = Vec::new();
                        for node in elems{
                            nodevec.push(node);
                        }
                        Some(nodevec)
                    },HAPI_FIRST_CHILDREN => {
                        let elems = elem.first_children();
                        let mut nodevec:Vec<NodeRef<Node>> = Vec::new();
                        for node in elems{
                            nodevec.push(node);
                        }
                        Some(nodevec)
                    },
                    HAPI_LAST_CHILDREN => {
                        let elems = elem.last_children();
                        let mut nodevec:Vec<NodeRef<Node>> = Vec::new();
                        for node in elems{
                            nodevec.push(node);
                        }
                        Some(nodevec)
                    },
                    HAPI_NEXT_SIBLINGS => {
                        let elems = elem.next_siblings();
                        let mut nodevec:Vec<NodeRef<Node>> = Vec::new();
                        for node in elems{
                            nodevec.push(node);
                        }
                        Some(nodevec)
                    },
                    HAPI_PREV_SIBLINGS => {
                        let elems = elem.prev_siblings();
                        let mut nodevec:Vec<NodeRef<Node>> = Vec::new();
                        for node in elems{
                            nodevec.push(node);
                        }
                        Some(nodevec)
                    },
                    _ => None
                };

                if multiple_opt.is_none(){
                    return Ok(Value::Null);
                }
                let multiple = multiple_opt.unwrap();
                let elems_arr = engine.create_array();

                let mut new_elem_ref_hash:HashMap<String, NodeId> = HashMap::new();
                for elem in multiple{
                    let eid = Uuid::new_v4().to_string();                    
                    let elem_obj = engine.create_object();
                    elem_obj.set(DOC_ID, doc_id.clone()).unwrap();
                    elem_obj.set(ID, eid.clone()).unwrap();
                    let select_fn = engine.create_function(select);
                    elem_obj.set(HAPI_SELECT, select_fn).unwrap();
                    init_elem_obj(engine, &elem_obj);
                    elems_arr.push(elem_obj).unwrap();

                    new_elem_ref_hash.insert(eid.clone(), elem.clone().id());
                }
                
                node_map.extend(new_elem_ref_hash);
                
                return Ok(Value::Array(elems_arr));
            }else{
                return Ok(Value::Null);
            }
        }else{
            return Ok(Value::Null);
        }   
    }else{
        return Ok(Value::Null);
    }
}
