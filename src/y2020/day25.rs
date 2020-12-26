use std::vec::Vec;

pub type PuzzleInput = (usize, usize);

#[aoc_generator(day25)]
pub fn parser(input: &str) -> PuzzleInput {
    let nums = input.lines().map(|l| l.parse().unwrap()).collect::<Vec<_>>();

    (nums[0], nums[1])
}

#[aoc(day25, part1)]
pub fn day25_part1(data: &PuzzleInput) -> usize {
    let (key1, key2) = data;
    let loop_size1 = find_loop_size(7, *key1);
    let loop_size2 = find_loop_size(7, *key2);

    let encryption_key1 = transform_subject(*key1, loop_size2);
    let encryption_key2 = transform_subject(*key2, loop_size1);

    assert_eq!(encryption_key1, encryption_key2);

    encryption_key1
}

fn find_loop_size(subject_number: usize, target: usize) -> usize {
    let mut value = 1;
    let mut loop_size = 1;
    loop {
        value = value * subject_number;
        value = value % 20201227;
        if value == target { return loop_size; }
        
        loop_size += 1;
    }
}

fn transform_subject(subject_number: usize, loop_size: usize) -> usize {
    let mut value = 1;
    for _loop in 0..loop_size {
        value = value * subject_number;
        value = value % 20201227;        
    }
    value
}
