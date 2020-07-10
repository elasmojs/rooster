pub trait StringUtils{
    fn index_of(&self, match_str:&str) -> i32;
    fn last_index_of(&self, match_str:&str) -> i32;
    fn last_char(&self) -> String;
}

impl StringUtils for String{
    fn index_of(&self, match_str:&str) -> i32{
        let str_idx = match self.find(match_str){
            Some(idx) => idx as i32,
            None => i32::from(-1)
        };
        return str_idx;
    }

    fn last_index_of(&self, match_str:&str) -> i32{
        let str_idx = match self.rfind(match_str){
            Some(idx) => idx as i32,
            None => i32::from(-1)
        };
        return str_idx;
    }

    fn last_char(&self) -> String{
        let last_char = match self.get(self.chars().count()..){
            Some(char) => char,
            None => ""
        };
        return last_char.to_string();
    }
}
