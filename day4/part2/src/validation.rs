use regex::Regex;

pub fn check_byr(value: &String) -> bool {
    let chars: Vec<char> = value.chars().collect();
    if chars.len() != 4 {
        false
    } else {
       if 1920 <= value.parse::<i32>().unwrap() && value.parse::<i32>().unwrap() <= 2002 {
           true
       } else {
           false
       }
    }
}

pub fn check_iyr(value: &String) -> bool {
    let chars: Vec<char> = value.chars().collect();
    if chars.len() != 4 {
        false
    } else {
        if 2010 <= value.parse::<i32>().unwrap() && value.parse::<i32>().unwrap() <= 2020 {
            true
        } else {
            false
        }
    }
}

pub fn check_eyr(value: &String) -> bool {
    let chars: Vec<char> = value.chars().collect();
    if chars.len() != 4 {
        false
    } else {
        if 2020 <= value.parse::<i32>().unwrap() && value.parse::<i32>().unwrap() <= 2030 {
            true
        } else {
            false
        }
    }
}

pub fn check_hgt(value: &String) -> bool {
    let mut char_indexes: Vec<i32> = vec![];
    let mut char_values: Vec<char> = vec![];
    let chars: Vec<char> = value.chars().collect();
    for i in 0..chars.len() {
        if chars[i].is_alphabetic() {
            char_indexes.push(i as i32);
            char_values.push(chars[i]);
        }
    }
    let char_string: String = char_values.into_iter().collect();
    if char_string == "cm" || char_string == "in" {
        if char_indexes[0] == ((chars.len() as i32) - 2) || char_indexes[1] == ((chars.len() as i32) - 1) {
            if char_string == "cm" {
                let mut number = String::new();
                for i in 0..char_indexes[0] {
                    number = format!("{}{}", number, chars[i as usize]);
                }
                if 150 <= number.parse::<i32>().unwrap() && number.parse::<i32>().unwrap() <= 193 {
                    true
                } else {
                    false
                }
            } else {
                let mut number = String::new();
                for i in 0..char_indexes[0] {
                    number = format!("{}{}", number, chars[i as usize]);
                }
                if 59 <= number.parse::<i32>().unwrap() && number.parse::<i32>().unwrap() <= 76 {
                    true
                } else {
                    false
                }
            }
        } else {
            false
        }
    } else {
        false
    }
}

pub fn check_hcl(value: &String) -> bool {
    let re = Regex::new(r"#([0-9]|[A-F]|[a-f]){6}").unwrap();
    re.is_match(value)
}

pub fn check_ecl(value: &String) -> bool {
    let arr = vec!["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
    arr.contains(&&**value)
}

pub fn check_pid(value: &String) -> bool {
    let chars: Vec<char> = value.chars().collect();
    if chars.len() == 9 {
        let mut counter = 0;
        for char in chars {
            if vec![0,1,2,3,4,5,6,7,8,9].contains(&(char.to_string()).parse::<i32>().unwrap()) {
                counter = counter + 1;
            }
        }
        if counter == 9 {
            true
        } else {
            false
        }
    } else {
        false
    }
}

