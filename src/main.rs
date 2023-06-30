use std::{env, fs};

fn main() {
    let res = env::args().collect::<Vec<String>>();
    let query = &res[1];
    let filename = &res[2];
    let res = fs::read_to_string(filename)
    .expect("something wrong!!");
    println!("{res}")
}
