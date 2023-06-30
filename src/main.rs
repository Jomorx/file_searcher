use std::{env, fs};

fn main() {
    let args = env::args().collect::<Vec<String>>();
    let config = Config::new(&args);
    let res = fs::read_to_string(config.filename).expect("something wrong!!");
    println!("{res}")
}

struct Config {
    query: String,
    filename: String,
}

impl Config {
    fn new(args: &[String]) -> Config {
        let query = args[1].clone();
        let filename = args[2].clone();
        Config { query, filename }
    }
}
