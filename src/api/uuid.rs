use ducc::{Ducc, Invocation, Value, Error as DuccError};

mod uuid;

use self::uuid::UUID;

pub const API_KEY:&str = "api";
pub const UUID_API:&str = "uuid";

pub fn load(engine:&Ducc) -> bool{
    let api_res:Result<Value, _> = engine.globals().get(API_KEY);
    let api_obj = api_res.unwrap();                
    let api = api_obj.as_object().unwrap();

    let uuid = engine.create_object();
    uuid.set("get", engine.create_function(uuid_get)).unwrap();

    api.set(UUID_API, uuid).unwrap();

    return true;
}

pub fn uuid_get(inv: Invocation) -> Result<Value, DuccError>{
    let engine = inv.ducc;
    return Ok(Value::String(engine.create_string(UUID::get().as_str()).unwrap()));
}
