use std::collections::HashMap;

type Passport = HashMap<String, String>;

fn parse_passport_lines(lines: &[String]) -> Passport {
    let mut ret = Passport::new();

    for line in lines {
        for pair in line.split(' ') {
            let key_val = pair.split(':').collect::<Vec<_>>();
            ret.insert(String::from(key_val[0]), String::from(key_val[1]));
        }
    }

    ret
}

fn passport_has_required_fields(passport: &Passport) -> bool {
    passport.contains_key("byr")
        && passport.contains_key("iyr")
        && passport.contains_key("eyr")
        && passport.contains_key("hgt")
        && passport.contains_key("hcl")
        && passport.contains_key("ecl")
        && passport.contains_key("pid")
}

fn validate_year(min: u32, max: u32, val: &str) -> bool {
    match val.parse::<u32>() {
        Ok(year) => year >= min && year <= max,
        Err(_) => false,
    }
}

fn validate_height(val: &str) -> bool {
    if val.ends_with("in") {
        match val.replace("in", "").parse::<u32>() {
            Ok(inches) => inches >= 59 && inches <= 76,
            Err(_) => false,
        }
    } else if val.ends_with("cm") {
        match val.replace("cm", "").parse::<u32>() {
            Ok(cms) => cms >= 150 && cms <= 193,
            Err(_) => false,
        }
    } else {
        false
    }
}

fn validate_hex_color(val: &str) -> bool {
    val.len() == 7 && val.starts_with("#") && u32::from_str_radix(&val[1..], 16).is_ok()
}

fn validate_named_color(val: &str) -> bool {
    val == "amb"
        || val == "blu"
        || val == "brn"
        || val == "gry"
        || val == "grn"
        || val == "hzl"
        || val == "oth"
}

fn validate_passport_id(val: &str) -> bool {
    val.len() == 9 && val.parse::<u32>().is_ok()
}

fn passport_fields_are_valid(passport: &Passport) -> bool {
    validate_year(1920, 2002, passport["byr"].as_str())
        && validate_year(2010, 2020, passport["iyr"].as_str())
        && validate_year(2020, 2030, passport["eyr"].as_str())
        && validate_height(passport["hgt"].as_str())
        && validate_hex_color(passport["hcl"].as_str())
        && validate_named_color(passport["ecl"].as_str())
        && validate_passport_id(passport["pid"].as_str())
}

pub fn main() {
    let lines = std::fs::read_to_string("data/day4.txt")
        .unwrap()
        .lines()
        .map(|x| String::from(x.trim()))
        .collect::<Vec<_>>();

    let passports = lines
        .split(|x| x.trim().len() < 1)
        .map(parse_passport_lines)
        .collect::<Vec<_>>();

    let part1 = passports
        .iter()
        .filter(|x| passport_has_required_fields(x))
        .count();

    let part2 = passports
        .iter()
        .filter(|x| passport_has_required_fields(x))
        .filter(|x| passport_fields_are_valid(x))
        .count();

    println!("{} {}", part1, part2);
}
