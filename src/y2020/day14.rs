use regex::Regex;
use std::collections::HashMap;
use std::vec::Vec;

pub enum Instruction {
    // 'keep mask' and 'value'
    // result = (initial_value & keep_mask) | value
    SetMask(String),
    Write(u64, u64),
}

#[aoc_generator(day14)]
pub fn parser(input: &str) -> Vec<Instruction> {
    lazy_static! {
        static ref MASK_PATTERN: Regex = Regex::new(r"mask = (?P<mask>.*)$").unwrap();
        static ref ASSIGNMENT_PATTERN: Regex =
            Regex::new(r"mem\[(?P<target>\d+)\] = (?P<value>\d+)").unwrap();
    }
    input
        .lines()
        .map(|l| {
            if let Some(c) = MASK_PATTERN.captures(l) {
                let mask = c.name("mask").unwrap().as_str();
                return Instruction::SetMask(mask.to_string());
            } else if let Some(c) = ASSIGNMENT_PATTERN.captures(l) {
                let target = c.name("target").unwrap().as_str().parse::<u64>().unwrap();
                let value = c.name("value").unwrap().as_str().parse::<u64>().unwrap();
                return Instruction::Write(target, value);
            } else {
                println!("LINE {}", l);
                unimplemented!("line did not match");
            }
        })
        .collect::<Vec<_>>()
}

#[aoc(day14, part1)]
pub fn day14_part1(data: &Vec<Instruction>) -> u64 {
    let mut memory = HashMap::<u64, u64>::new();
    let mut latest_keep_mask = 0;
    let mut latest_mask_value = 0;

    for instruction in data {
        match instruction {
            Instruction::SetMask(mask) => {
                latest_keep_mask = u64::from_str_radix(
                    &mask.replace("0", "0").replace("1", "0").replace("X", "1"),
                    2,
                )
                .unwrap();
                latest_mask_value = u64::from_str_radix(&mask.replace("X", "0"), 2).unwrap();
            }
            Instruction::Write(target, value) => {
                let masked_value = (*value & latest_keep_mask) | latest_mask_value;
                memory.insert(*target, masked_value);
            }
        }
    }

    memory.iter().fold(0, |acc, (_key, value)| acc + value)
}

#[aoc(day14, part2)]
pub fn day14_part2(data: &Vec<Instruction>) -> u64 {
    let mut memory = HashMap::<u64, u64>::new();
    let mut latest_mask = "".to_string();

    for instruction in data {
        match instruction {
            Instruction::SetMask(mask) => {
                latest_mask = mask.to_string();
            }
            Instruction::Write(target, value) => {
                let target_binary = value_to_binary(*target);
                for possible_binary in permute_rest(latest_mask.as_str(), target_binary) {
                    memory.insert(u64::from_str_radix(&possible_binary, 2).unwrap(), *value);
                }
            }
        }
    }

    memory.iter().fold(0, |acc, (_key, value)| acc + value)
}

fn value_to_binary(value: u64) -> String {
    let binary = format!("{:b}", value);
    format!("{:0>36}", binary)
}

fn permute_rest(mask: &str, value: String) -> Vec<String> {
    if mask.len() == 0 {
        let mut res = Vec::<String>::new();
        res.push("".to_string());
        return res;
    }

    let mask_last = mask.chars().last().unwrap();
    let mut mask_rest = mask.to_string();
    mask_rest.pop();

    let value_last = value.chars().last().unwrap();
    let mut value_rest = value.to_string();
    value_rest.pop();

    let permuted_rest = permute_rest(&mask_rest.to_string(), value_rest.to_string());

    match mask_last {
        '0' => permuted_rest
            .iter()
            .map(|s| {
                let mut str = String::from(s);
                str.push(value_last);
                str
            })
            .collect::<Vec<_>>(),
        '1' => permuted_rest
            .iter()
            .map(|s| {
                let mut str = String::from(s);
                str.push('1');
                str
            })
            .collect::<Vec<_>>(),
        'X' => {
            let mut output = Vec::<String>::new();
            for j in permuted_rest {
                let mut str = j.to_string();
                str.push('0');
                output.push(str);
                let mut str = j.to_string();
                str.push('1');
                output.push(str);
            }
            output
        }
        _ => unimplemented!("unexpected char"),
    }
}
