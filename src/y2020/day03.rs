use std::iter::Iterator;
use std::vec::Vec;

#[aoc_generator(day3)]
pub fn parser(input: &str) -> Vec<String> {
    input.lines().map(|x| x.to_string()).collect::<Vec<_>>()
}

fn at(data: &Vec<String>, row: usize, col: usize) -> char {
    let row_base = &data[row];
    let p = col % row_base.len();
    row_base.chars().nth(p).unwrap()
}

fn compute_tree_count(data: &Vec<String>, row_step: usize, col_step: usize) -> usize {
    let end_row = data.len();

    let mut row = 0;
    let mut col = 0;

    let mut trees = 0;

    loop {
        if row >= end_row {
            break;
        }
        if at(data, row, col) == '#' {
            trees += 1;
        }
        row += row_step;
        col += col_step;
    }

    trees
}

#[aoc(day3, part1)]
pub fn day3_part1(data: &Vec<String>) -> usize {
    compute_tree_count(data, 1, 3)
}

#[aoc(day3, part2)]
pub fn day3_part2(data: &Vec<String>) -> usize {
    compute_tree_count(data, 1, 1)
        * compute_tree_count(data, 1, 3)
        * compute_tree_count(data, 1, 5)
        * compute_tree_count(data, 1, 7)
        * compute_tree_count(data, 2, 1)
}
