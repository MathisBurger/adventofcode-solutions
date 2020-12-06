use std::fs;

mod utils;

fn main() {
    let content = fs::read_to_string("./input.txt").expect("Cannot read file");
    let group_strings = utils::split_content_into_groups(content);
    let mut count = 0;
    for group_string in group_strings {
        count += utils::calculate_questions(&group_string);
    }
    println!("Count: {}", count);
}
