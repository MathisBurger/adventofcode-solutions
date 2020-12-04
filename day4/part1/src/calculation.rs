use std::collections::HashMap;

pub fn check_completeness(map: &HashMap<String, String>) -> bool{
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