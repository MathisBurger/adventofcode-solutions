use std::collections::HashMap;
use crate::validation;

pub fn check_validation(map: &HashMap<String, String>) -> bool {
    if validation::check_byr(map.get("byr").unwrap()) {
        if validation::check_iyr(map.get("iyr").unwrap()) {
            if validation::check_eyr(map.get("eyr").unwrap()) {
                if validation::check_hgt(map.get("hgt").unwrap()) {
                    if validation::check_hcl(map.get("hcl").unwrap()) {
                        if validation::check_ecl(map.get("ecl").unwrap()) {
                            if validation::check_pid(map.get("pid").unwrap()) {
                                true
                            } else {
                                false
                            }
                        } else {
                            false
                        }
                    } else {
                        false
                    }
                } else {
                    false
                }
            } else {
                false
            }
        } else {
            false
        }
    } else {
        false
    }
}



pub fn check_completeness(map: &HashMap<String, String>) -> bool {
    match map.get("byr") {
        None => {false}
        Some(_) => {
            match map.get("iyr") {
                None => {false}
                Some(_) => {
                    match map.get("eyr") {
                        None => {false}
                        Some(_) => {
                            match map.get("hgt") {
                                None => {false}
                                Some(_) => {
                                    match map.get("hcl") {
                                        None => {false}
                                        Some(_) => {
                                            match map.get("ecl") {
                                                None => {false}
                                                Some(_) => {
                                                    match map.get("pid") {
                                                        None => {false}
                                                        Some(_) => {true}
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

