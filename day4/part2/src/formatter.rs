use std::collections::HashMap;


pub fn get_passport_array(content: String) ->  Vec<HashMap<String, String>> {
    let lines = content.split("\r\n").collect::<Vec<&str>>();
    let mut cache: Vec<&str> = vec![];
    let mut arr: Vec<HashMap<String, String>> = vec![];
    for line in lines {
        if line != "" {
            cache.push(line);
        } else {
            arr.push(get_passport_struct(&cache));
            cache = vec![];
        }
    }
    arr
}

fn get_passport_struct(lines: &Vec<&str>) -> HashMap<String, String> {
    let mut res_line = String::new();
    if lines.len() < 2 {
        res_line = lines[0].to_string();
    } else {
        res_line = lines[0].to_string();
        for i in 1..lines.len() {
            res_line = format!("{} {}", res_line, lines[i]);
        }
    }
    let split = res_line.split(" ").collect::<Vec<&str>>();
    let mut hashmap: HashMap<String, String> = HashMap::new();
    for value_pair in split {
        let k_v = value_pair.split(":").collect::<Vec<&str>>();
        hashmap.insert(k_v[0].to_string(), k_v[1].to_string());
    }
    hashmap
}