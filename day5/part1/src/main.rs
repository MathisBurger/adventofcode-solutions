use std::fs;
use crate::calculation::seat;

mod formatter;
mod calculation;

fn main() {
    let content = fs::read_to_string("./input.txt").unwrap();
    let binarys = formatter::get_binary_array(content);
    let mut seats: Vec<seat> = vec![];
    for binary in binarys {
        seats.push(calculation::get_seat(binary));
    }
    let highest_id = calculation::get_highest_id(seats);
    println!("{}", highest_id);
}
