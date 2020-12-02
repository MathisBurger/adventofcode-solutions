mod file_utils;
mod password_utils;


fn main() {
    let content = file_utils::read_content();
    let data = file_utils::parse_to_conditions(content);
    let mut counter = 0;
    for el in data {
        if  password_utils::check_password(&el) {
            counter = counter + 1;
        }
    }
    println!("Valid passwords: {}", counter);
}
