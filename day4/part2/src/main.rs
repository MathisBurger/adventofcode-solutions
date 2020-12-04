use std::fs;

mod formatter;
mod calculation;
mod validation;

fn main() {
    let content = fs::read_to_string("./input.txt").expect("Cannot read file");
    let passport_array = formatter::get_passport_array(content);
    let mut valid_passports = 0;
    for passport in passport_array {
        if calculation::check_completeness(&passport) {
            if calculation::check_validation(&passport) {
                valid_passports = valid_passports + 1;
            }
        }
    }
    println!("Valid passports: {}", valid_passports);

}
