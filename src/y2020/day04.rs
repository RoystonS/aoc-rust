use regex::Regex;
use std::collections::HashMap;
use std::iter::Iterator;
use std::vec::Vec;

#[aoc_generator(day4)]
pub fn parser(input: &str) -> Vec<HashMap<String, String>> {
    let mut res = Vec::<HashMap<String, String>>::new();

    let mut hash = HashMap::<String, String>::new();

    for mut line in input.lines() {
        line = line.trim();

        if line.len() == 0 {
            res.push(hash);
            hash = HashMap::<String, String>::new();
            continue;
        }

        for part in line.split(' ') {
            let bits: Vec<&str> = part.split(':').collect();
            hash.insert(bits[0].to_string(), bits[1].to_string());
        }
    }

    if hash.len() > 0 {
        res.push(hash);
    }

    res
}

fn validate_byr(value: &str) -> bool {
    match value.parse::<u16>() {
        Ok(value) => value >= 1920 && value <= 2020,
        Err(_) => false,
    }
}

fn validate_iyr(value: &str) -> bool {
    match value.parse::<u16>() {
        Ok(value) => value >= 2010 && value <= 2020,
        Err(_) => false,
    }
}

fn validate_eyr(value: &str) -> bool {
    match value.parse::<u16>() {
        Ok(value) => value >= 2020 && value <= 2030,
        Err(_) => false,
    }
}

fn validate_hgt(value: &str) -> bool {
    lazy_static! {
        static ref PATTERN: Regex = Regex::new(r"^(\d+)(cm|in)$").unwrap();
    }

    match PATTERN.captures(value) {
        Some(caps) => {
            let num = caps.get(1).unwrap().as_str().parse::<u16>();
            let typ = caps.get(2).unwrap().as_str();

            match (num, typ) {
                (Ok(val), "cm") => val >= 150 && val <= 193,
                (Ok(val), "in") => val >= 59 && val <= 76,
                _ => false,
            }
        }
        None => false,
    }
}

fn validate_hcl(value: &str) -> bool {
    lazy_static! {
        static ref PATTERN: Regex = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
    }

    PATTERN.is_match(value)
}

fn validate_ecl(value: &str) -> bool {
    lazy_static! {
        static ref PATTERN: Regex = Regex::new(r"^amb|blu|brn|gry|grn|hzl|oth$").unwrap();
    }

    PATTERN.is_match(value)
}

fn validate_pid(value: &str) -> bool {
    lazy_static! {
        static ref PATTERN: Regex = Regex::new(r"^\d{9}$").unwrap();
    }

    PATTERN.is_match(value)
}

fn anything(_value: &str) -> bool {
    true
}

type Validator = fn(value: &str) -> bool;

#[aoc(day4, part1)]
pub fn day4_part1(data: &Vec<HashMap<String, String>>) -> usize {
    check(data, false)
}

#[aoc(day4, part2)]
pub fn day4_part2(data: &Vec<HashMap<String, String>>) -> usize {
    check(data, true)
}

fn check(data: &Vec<HashMap<String, String>>, validate_values: bool) -> usize {
    let mut required_fields = HashMap::<String, Validator>::new();
    required_fields.insert("byr".to_string(), validate_byr);
    required_fields.insert("iyr".to_string(), validate_iyr);
    required_fields.insert("eyr".to_string(), validate_eyr);
    required_fields.insert("hgt".to_string(), validate_hgt);
    required_fields.insert("hcl".to_string(), validate_hcl);
    required_fields.insert("ecl".to_string(), validate_ecl);
    required_fields.insert("pid".to_string(), validate_pid);
    let mut optional_fields = HashMap::<String, Validator>::new();
    optional_fields.insert("cid".to_string(), anything);

    let mut valid = 0;

    for passport in data {
        let has_illegal_fields = passport.keys().any(|f| {
            let validator = required_fields.get(f).or(optional_fields.get(f));

            match validator {
                Some(func) => validate_values && !func(passport.get(f).unwrap()),
                None => true,
            }
        });
        if has_illegal_fields {
            continue;
        }

        let missing_required_field = required_fields.keys().any(|f| !passport.contains_key(f));
        if missing_required_field {
            continue;
        }

        valid += 1;
    }

    valid
}
