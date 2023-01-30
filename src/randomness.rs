extern crate rand;
use rand::Rng;

pub fn generate_random_number() -> u32 {
    rand::thread_rng().gen_range(1..101) //
}

pub fn generate_random_bool() -> bool {
    rand::thread_rng().gen_bool(0.8) //gen_bool takes a float between 0 and 1, where 0 is always false and 1 is always true
}