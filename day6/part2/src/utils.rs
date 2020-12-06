
pub fn get_group_vectors(value: String) -> Vec<Vec<String>> {
    let lines = value.split("\r\n").collect::<Vec<&str>>();
    let mut res: Vec<Vec<String>> = vec![];
    let mut cache: Vec<&str> = vec![];
    for line in lines {
        if line == "" {
            let mut group_vec: Vec<String> = vec![];
            for el in cache {
                group_vec.push(el.to_string());
            }
            res.push(group_vec);
            cache = vec![];
        } else {
            cache.push(line);
        }
    }
    res
}

pub fn calculate_questions(values: &Vec<String>) -> i32 {
    let mut containments: Vec<char> = vec![];
    for value in values {
        let chars: Vec<char> = value.chars().collect();
        for char in chars {
            let mut counter = 0;
            for val in values {
                if val.contains(char) {
                    counter += 1;
                }
            }
            if counter == values.len() {
                if !containments.contains(&char) {
                    containments.push(char);
                }
            }
        }
    }
    containments.len() as i32
}

fn adv_contains(value: &String, selector: &char) -> bool {
    let chars: Vec<char> = value.chars().collect();
    if chars.contains(selector) {
        true
    } else {
        false
    }
}