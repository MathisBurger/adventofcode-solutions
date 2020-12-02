use crate::file_utils::conditions;

pub fn check_password(data: &conditions) -> bool {
    let chars: Vec<char> = data.password.chars().collect();
    let mut counter = 0;
    for char in chars {
        if char == data.required_letter {
            counter = counter + 1;
        }
    }
    if counter > data.max_frequency || counter < data.min_frequency {
        false
    } else {
        true
    }
}