use std::iter::Iterator;
use std::vec::Vec;

#[aoc_generator(day1)]
pub fn mass_parser(input: &str) -> Vec<i64> {
    input
        .lines()
        .map(|line| line.parse::<i64>().unwrap())
        .collect::<Vec<_>>()
}

#[aoc(day1, part1)]
pub fn day1_part1(masses: &Vec<i64>) -> i64 {
    masses
        .iter()
        .fold(0, |acc, m| acc + ((*m as f64 / 3.).floor() - 2.) as i64)
}

fn fuel_for_mass(mass: i64) -> i64 {
    let fuel = (((mass as f64) / 3.0).floor() - 2.0) as i64;
    if fuel < 0 {
        return 0;
    }
    fuel + fuel_for_mass(fuel)
}

#[aoc(day1, part2)]
pub fn day1_part2(masses: &Vec<i64>) -> i64 {
    masses
        .iter()
        .fold(0, |acc, mass| acc + fuel_for_mass(*mass))
}
