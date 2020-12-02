use std::iter::Iterator;
use std::vec::Vec;

pub struct Row {
    range1: usize,
    range2: usize,
    ch: char,
    text: String,
}

#[aoc_generator(day2)]
pub fn parser(input: &str) -> Vec<Row> {
    input
        .lines()
        .map(|line| {
            let bits: Vec<&str> = line.split(' ').collect();
            let range_bits: Vec<&str> = bits[0].split('-').collect();

            return Row {
                range1: range_bits[0].parse::<usize>().unwrap(),
                range2: range_bits[1].parse::<usize>().unwrap(),
                ch: bits[1].chars().next().unwrap(),
                text: bits[2].to_string(),
            };
        })
        .collect::<Vec<_>>()
}

#[aoc(day2, part1)]
pub fn day2_part1(data: &Vec<Row>) -> usize {
    let valid_rows = data.iter().filter(|row| {
        let matching_chars = row.text.chars().filter(|c| *c == row.ch);
        let match_count = matching_chars.count();
        return match_count >= row.range1 && match_count <= row.range2;
    });

    valid_rows.count()
}

#[aoc(day2, part2)]
pub fn day2_part2(data: &Vec<Row>) -> usize {
    let valid_rows = data.iter().filter(|row| {
        let v1 = row.text.chars().nth(row.range1 - 1).unwrap() == row.ch;
        let v2 = row.text.chars().nth(row.range2 - 1).unwrap() == row.ch;

        // Only one must be true
        return v1 != v2;
    });

    valid_rows.count()
}
