
pub fn split_content_into_groups(content: String) -> Vec<String> {
    let lines = content.split("\r\n").collect::<Vec<&str>>();
    let mut res: Vec<String> = vec![];
    let mut cache: Vec<&str> = vec![];
    for line in lines {
        if line == "" {
            let mut group_string: String = String::new();
            for el in cache {
                group_string = format!("{}{}", group_string, el);
            }
            res.push(group_string);
            cache = vec![];
        } else {
            cache.push(line);
        }
    }
    res
}

pub fn calculate_questions(value: &String) -> i32 {
    let chars: Vec<char> = value.chars().collect();
    let mut containments: Vec<char> = vec![];
    for char in chars {
        if !containments.contains(&char) {
            containments.push(char);
        }
    }
    containments.len() as i32
}