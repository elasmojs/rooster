use rand::prelude::{random, thread_rng};
use rand::Rng;
pub struct Rand{}

impl Rand{
    pub fn get() -> u8{
        return random();
    }

    pub fn float() -> f64{
        let mut rng = thread_rng();
        return rng.gen();
    }

    pub fn range(min:u8, max:u8) -> u8{
        let mut rng = thread_rng();
        return rng.gen_range(min, max);
    }
}
