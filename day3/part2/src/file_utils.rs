use std::fs;


pub fn load_content() -> String {
    let filename = "./input.txt";
    fs::read_to_string(filename).expect("Cannot read content from file")
}