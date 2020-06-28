pub fn _index_of(str:String, match_str:&str) -> i32{
    let str_idx = match str.find(match_str){
        Some(idx) => idx as i32,
        None => i32::from(-1)
    };
    return str_idx;
}

pub fn _last_index_of(str:String, match_str:&str) -> i32{
    let str_idx = match str.rfind(match_str){
        Some(idx) => idx as i32,
        None => i32::from(-1)
    };
    return str_idx;
}

pub fn _last_char(str:String) -> String{
    let last_char = match str.get(str.chars().count()..){
        Some(char) => char,
        None => ""
    };
    return last_char.to_string();
}