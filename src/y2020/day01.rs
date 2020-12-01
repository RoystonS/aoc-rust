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
pub fn day1_part1(data: &Vec<i64>) -> i64 {
    0
}
