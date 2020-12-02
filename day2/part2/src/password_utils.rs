use crate::file_utils::conditions;

pub fn check_password(data: &conditions) -> bool {
    let pos1 = data.range[0] - 1;
    let pos2 = data.range[1] - 1;
    let letter = data.required_letter;
    let chars: Vec<char> = data.password.chars().collect();

    if (chars[pos1] == letter && chars[pos2] != letter) ||
        (chars[pos1] != letter && chars[pos2] == letter) {
        true
    } else {
        false
    }
}