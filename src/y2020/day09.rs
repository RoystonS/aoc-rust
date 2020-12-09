use std::iter::Iterator;
use std::vec::Vec;

#[aoc_generator(day9)]
pub fn parser(input: &str) -> Vec<usize> {
    input
        .lines()
        .map(|line| line.parse::<usize>().unwrap())
        .collect::<Vec<_>>()
}

fn contains_sum(data: &[usize], num: usize) -> bool {
    for i in 0..data.len() {
        for j in 0..data.len() {
            if i != j {
                if num == data[i] + data[j] {
                    return true;
                }
            }
        }
    }

    return false;
}

#[aoc(day9, part1)]
pub fn day9_part1(data: &Vec<usize>) -> usize {
    let back = 25;

    let mut pos = back;

    loop {
        if pos > data.len() {
            unimplemented!("Ran off the end. No matching number found");
        }

        let num = data[pos];
        let slice = &data[pos - back..pos];
        if !contains_sum(slice, num) {
            return num;
        }
        pos += 1;
    }
}

#[aoc(day9, part2)]
pub fn day9_part2(data: &Vec<usize>) -> usize {
    let search_number = day9_part1(data);

    let search_index = data.iter().position(|x| *x == search_number).unwrap();

    let mut smallest_index = 0;

    // Try summing values from [x..y], increasing x, looking for a sum matching the search_number.
    loop {
        let mut sum = 0;
        let mut smallest = usize::MAX;
        let mut largest = usize::MIN;

        for i in smallest_index..search_index {
            let this_value = data[i];

            smallest = smallest.min(this_value);
            largest = largest.max(this_value);

            sum += this_value;

            if sum == search_number {
                return smallest + largest;
            }
        }

        smallest_index += 1;
    }
}
