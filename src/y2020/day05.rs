use regex::Regex;
use std::collections::HashSet;
use std::iter::Iterator;
use std::vec::Vec;

#[aoc_generator(day5)]
pub fn parser(input: &str) -> Vec<String> {
    input.lines().map(|x| x.to_string()).collect::<Vec<_>>()
}

fn get_seat_id(id: String) -> usize {
    lazy_static! {
        static ref RE1: Regex = Regex::new(r"[BR]").unwrap();
        static ref RE2: Regex = Regex::new(r"[FL]").unwrap();
    }
    let x = RE1.replace_all(&id, "1");
    let y = RE2.replace_all(&x, "0");
    usize::from_str_radix(&y, 2).unwrap()
}

fn lowest_seat_id(data: &Vec<String>) -> usize {
    data.iter()
        .map(|l| get_seat_id(l.to_string()))
        .min()
        .unwrap()
}

fn highest_seat_id(data: &Vec<String>) -> usize {
    data.iter()
        .map(|l| get_seat_id(l.to_string()))
        .max()
        .unwrap()
}

#[aoc(day5, part1)]
pub fn day5_part1(data: &Vec<String>) -> usize {
    highest_seat_id(data)
}

#[aoc(day5, part2)]
pub fn day5_part2(data: &Vec<String>) -> usize {
    let lowest = lowest_seat_id(data);
    let highest = highest_seat_id(data);

    let mut set = HashSet::<usize>::new();
    for s in lowest..highest {
        set.insert(s);
    }

    for seat_id in data.iter().map(|l| get_seat_id(l.to_string())) {
        set.remove(&seat_id);
    }

    *set.iter().next().unwrap()
}

#[test]
fn test_cases() {
    assert_eq!(get_seat_id("BFFFBBFRRR".to_string()), 567);
    assert_eq!(get_seat_id("FFFBBBFRRR".to_string()), 119);
    assert_eq!(get_seat_id("BBFFBBFRLL".to_string()), 820);
}
