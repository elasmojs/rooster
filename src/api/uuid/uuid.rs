use uuid::Uuid;

pub struct UUID{}

impl UUID{
    pub fn get() -> String{
        return Uuid::new_v4().to_hyphenated().to_string();
    }
}


