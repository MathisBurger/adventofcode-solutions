use std::fs;

pub struct conditions {
    pub(crate) range: Vec<usize>,
    pub(crate) required_letter: char,
    pub(crate) password: String
}

pub fn read_content() -> String {
    let filename = "./input.txt";
    fs::read_to_string(filename).expect("Cannot read file content")
}

pub fn parse_to_conditions(content: String) -> Vec<conditions> {
    let lines = content.split("\n").collect::<Vec<&str>>();
    let mut result_array: Vec<conditions> = vec![];
    for line in lines {
        let split = line.split(": ").collect::<Vec<&str>>();
        let condition = split[0].split(" ").collect::<Vec<&str>>();
        let positions = condition[0].split("-").collect::<Vec<&str>>();
        result_array.push(conditions {
            range: vec![positions[0].parse().unwrap(), positions[1].parse().unwrap()],
            required_letter: condition[1].parse().unwrap(),
            password: split[1].to_string()
        });
    }
    result_array
}
