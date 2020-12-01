use std::iter::Iterator;
use std::vec::Vec;

#[aoc_generator(day1)]
pub fn parser(input: &str) -> Vec<i64> {
    input
        .lines()
        .map(|line| line.parse::<i64>().unwrap())
        .collect::<Vec<_>>()
}

#[aoc(day1, part1)]
pub fn day1_part1(data: &Vec<i64>) -> Option<i64> {
    for x in data {
        for y in data {
            if x + y == 2020 {
                return Some(x * y);
            }
        }
    }
    None
}

#[aoc(day1, part2)]
pub fn day1_part2(data: &Vec<i64>) -> Option<i64> {
    for x in data {
        for y in data {
            for z in data {
                if x + y + z == 2020 {
                    return Some(x * y * z);
                }
            }
        }
    }
    None
}
