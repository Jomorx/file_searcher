use std::{env, fs, process};

fn main() {
    let args = env::args().collect::<Vec<String>>();
    let config = Config::new(&args).unwrap_or_else(|err|{
        println!("Problem parsing argument {}",err);
        process::exit(1);
    });
    let res = fs::read_to_string(config.filename).expect("something wrong!!");
    println!("{res}")
}

struct Config {
    query: String,
    filename: String,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("argument is not enough!!");
        }
        let query = args[1].clone();
        let filename = args[2].clone();
        Ok(Config { query, filename })
    }
}
