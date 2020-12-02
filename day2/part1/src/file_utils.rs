use std::fs;

pub struct conditions {
    pub(crate) min_frequency: i16,
    pub(crate) max_frequency: i16,
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
        let max_min_frequency = condition[0].split("-").collect::<Vec<&str>>();
        result_array.push(conditions {
            min_frequency: max_min_frequency[0].parse().unwrap(),
            max_frequency: max_min_frequency[1].parse().unwrap(),
            required_letter: condition[1].to_string().parse().unwrap(),
            password: split[1].to_string()
        });
    }
    result_array
}

