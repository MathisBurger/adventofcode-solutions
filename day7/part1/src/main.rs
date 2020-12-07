use std::fs;

mod utils;
mod parser;
#[macro_use(lazy_static)]
extern crate lazy_static;

fn main() {
    let content = fs::read_to_string("./input.txt").expect("Cannot read file");
    let lines = content.split("\r\n").collect::<Vec<&str>>();
    println!("{}", utils::get_num(lines.iter()));
}
