use itertools::Itertools;
use std::collections::HashSet;
use std::vec::Vec;

#[aoc_generator(day11)]
pub fn parser(input: &str) -> Vec<Vec<char>> {
    input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>()
}

fn occupied_count(data: &Vec<Vec<char>>, row: usize, col: usize, depth: usize) -> usize {
    let mut count: usize = 0;

    let mut depth_so_far = 1 as isize;

    let mut remaining_deltas = (-1..=1)
        .cartesian_product(-1..=1)
        .filter(|(rd, cd)| *rd != 0 || *cd != 0)
        .collect::<HashSet<_>>();

    loop {
        for entry in remaining_deltas.clone().iter() {
            let (row_delta_sgn, col_delta_sgn) = entry;
            let row_delta = row_delta_sgn * depth_so_far;
            let col_delta = col_delta_sgn * depth_so_far;
            let r = row as isize + row_delta;
            let c = col as isize + col_delta;

            if r < 0 || c < 0 || r > data.len() as isize - 1 || c > data[0].len() as isize - 1 {
                remaining_deltas.remove(entry);
                continue;
            }

            let ru = r as usize;
            let cu = c as usize;

            if data[ru][cu] == '#' {
                count += 1;
                remaining_deltas.remove(entry);
            }
            if data[ru][cu] == 'L' {
                remaining_deltas.remove(entry);
            }
        }

        if remaining_deltas.len() == 0 {
            return count;
        }

        if depth_so_far == depth as isize {
            return count;
        }

        depth_so_far += 1;
    }
}

fn run_seat_layout(data: &mut Vec<Vec<char>>, depth: usize, seat_count_for_empty: usize) -> bool {
    let mut changed = false;

    let read_only = data.clone();

    for row_index in 0..data.len() {
        let row = data.get_mut(row_index).unwrap();

        for col_index in 0..row.len() {
            let cell = row[col_index];
            match cell {
                'L' => {
                    if occupied_count(&read_only, row_index, col_index, depth) == 0 {
                        row[col_index] = '#';
                        changed = true;
                    }
                }
                '#' => {
                    if occupied_count(&read_only, row_index, col_index, depth)
                        >= seat_count_for_empty
                    {
                        row[col_index] = 'L';
                        changed = true;
                    }
                }
                _ => {}
            }
        }
    }

    changed
}

#[aoc(day11, part1)]
pub fn day11_part1(data: &Vec<Vec<char>>) -> usize {
    let mut state = data.to_owned();

    loop {
        let changed = run_seat_layout(&mut state, 1, 4);
        if !changed {
            break;
        }
    }

    let occupied_count = state.iter().fold(0, |acc, row| {
        acc + row
            .iter()
            .fold(0, |acc, cell| acc + if *cell == '#' { 1 } else { 0 })
    });

    occupied_count
}

#[aoc(day11, part2)]
pub fn day11_part2(data: &Vec<Vec<char>>) -> usize {
    let mut state = data.to_owned();


    loop {
        let changed = run_seat_layout(&mut state, usize::MAX, 5);
        if !changed {
            break;
        }
    }

    let occupied_count = state.iter().fold(0, |acc, row| {
        acc + row
            .iter()
            .fold(0, |acc, cell| acc + if *cell == '#' { 1 } else { 0 })
    });

    occupied_count
}
