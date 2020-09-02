use ducc::{Ducc, Invocation, Value, Error as DuccError};
use log::*;

mod rnd;

use self::rnd::Rand;

pub const API_KEY:&str = "api";
pub const RND_API:&str = "rnd";

pub fn load(engine:&Ducc) -> bool{
    let api_res:Result<Value, _> = engine.globals().get(API_KEY);
    let api_obj = api_res.unwrap();                
    let api = api_obj.as_object().unwrap();

    let rnd = engine.create_object();
    rnd.set("get", engine.create_function(rnd_get)).unwrap();
    rnd.set("float", engine.create_function(rnd_float)).unwrap();
    rnd.set("range", engine.create_function(rnd_range)).unwrap();

    api.set(RND_API, rnd).unwrap();

    return true;
}

pub fn rnd_get(_inv: Invocation) -> Result<Value, DuccError>{
    return Ok(Value::Number(Rand::get() as f64));
}

pub fn rnd_float(_inv: Invocation) -> Result<Value, DuccError>{
    return Ok(Value::Number(Rand::float()));
}

pub fn rnd_range(inv: Invocation) -> Result<Value, DuccError>{
    let args = inv.args;
    if args.len() == 2{
        let mn = args.get(0);
        let mx = args.get(1);

        if mn.is_number() && mx.is_number(){
            let min = mn.as_number().unwrap() as u8;
            let max = mx.as_number().unwrap() as u8;
            return Ok(Value::Number(Rand::range(min, max) as f64));
        }else{
            error!("Invalid argument for range, expected number");
            return Ok(Value::Null);
        }
    }else{
        error!("Invalid argument for range, expected 2 arguments");
        return Ok(Value::Null);
    }
}
