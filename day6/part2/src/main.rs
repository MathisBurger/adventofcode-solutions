use std::fs;

mod utils;

fn main() {
    let content = fs::read_to_string("./input.txt").expect("Cannot read file");
    let group_vectors = utils::get_group_vectors(content);
    let mut count = 0;
    for group_vector in group_vectors {
        count += utils::calculate_questions(&group_vector);
    }
    println!("Count: {}", count);
}
