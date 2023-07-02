use std::env;
use std::error::Error;
use std::fs;
pub fn run(
    Config {
        filename,
        query,
        case_sensitive,
    }: Config,
) -> Result<(), Box<dyn Error>> {
    let res = fs::read_to_string(filename)?;
    match case_sensitive {
        true => search(&res, &query),
        false => search_sensitive(&res, &query),
    }.iter().for_each(|line| println!("{}", line));
    Ok(())
}
pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("argument is not enough!!");
        }
        let filename = args[1].clone();
        let query = args[2].clone();
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
        Ok(Config { query, filename ,case_sensitive})
    }
}

pub fn search<'a>(content: &'a str, target: &str) -> Vec<&'a str> {
    let mut res = vec![];
    for line in content.lines() {
        if line.contains(target) {
            res.push(line);
        }
    }
    res
}
pub fn search_sensitive<'a>(content: &'a str, target: &str) -> Vec<&'a str> {
    println!("1111");
    let mut res = vec![];
    for line in content.lines() {
        if line.to_lowercase().contains(& target.to_lowercase()) {
            res.push(line);
        }
    }
    res
}
