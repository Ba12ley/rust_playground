use std::env;

pub fn get_args() -> Vec<String> {
    let args: Vec<String> = env::args().collect();
    for arg in args.iter() {
        println!("{}", arg);
    }
    args

}