mod file_utils;
mod password_utils;

fn main() {
    let raw = file_utils::read_content();
    let data = file_utils::parse_to_conditions(raw);
    let mut correct_passwords = 0;
    for el in data {
        if password_utils::check_password(&el) {
            correct_passwords = correct_passwords + 1;
        }
    }
    println!("Correct passwords: {}", correct_passwords);
}
