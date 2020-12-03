use std::collections::HashMap;


pub fn get_value_map(content: String) -> HashMap<i32, Vec<char>> {
    let mut result: HashMap<i32, Vec<char>> = HashMap::new();
    let lines = content.split("\r\n").collect::<Vec<&str>>();
    //multiplicator must be greater than 3
    let multiplicator = lines.len() * 3;
    for line in lines {
        let mut str = String::new();
        for i in 0..multiplicator {
            str = format!("{}{}", str, line);
        }
        let chars: Vec<char> = str.chars().collect();
        result.insert((result.len() as i32), chars);
    }
    result
}

pub fn iterate_for_trees(map: &HashMap<i32, Vec<char>>, right: i32, down: i32) -> i32 {
    let mut tree_counter = 0;
    let mut vertical_counter = 0;
    let mut horizontal_counter = 0;
    let line_iterator = map.len() / (down as usize);
    for i in 0..line_iterator {
        let char: &char = map.get(&vertical_counter).unwrap().get(horizontal_counter).unwrap();
        let const_char: Vec<char> = "#".chars().collect();
        if char == &const_char[0] {
            tree_counter = tree_counter + 1;
        }
        horizontal_counter = horizontal_counter + (right as usize);
        vertical_counter = vertical_counter + down;
    }
    tree_counter
}