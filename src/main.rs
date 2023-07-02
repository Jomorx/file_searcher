use std::{env, process};
use file_searcher::{Config, run};

fn main() {
    let args = env::args().collect::<Vec<String>>();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing argument {}", err);
        process::exit(1);
    });
    if let Err(e) = run(config) {
        println!("App error {}",e);
        process::exit(1);
    }
}