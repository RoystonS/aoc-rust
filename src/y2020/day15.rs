use std::collections::HashMap;

#[aoc_generator(day15)]
pub fn parser(input: &str) -> Vec<usize> {
    input
        .split(',')
        .map(|n| n.parse().unwrap())
        .collect::<Vec<_>>()
}

#[aoc(day15, part1)]
pub fn day15_part1(data: &Vec<usize>) -> usize {
    compute(data, 2020)
}

#[aoc(day15, part2)]
pub fn day15_part2(data: &Vec<usize>) -> usize {
    compute(data, 30000000)
}

fn compute(data: &Vec<usize>, target: usize) -> usize {
    let mut count = 1;

    // The last number spoken
    let mut last_number: usize = 0;

    // The difference between the last two times each number was spoken
    let mut last_diff = HashMap::<usize, usize>::new();

    // The last turn that each number was actually spoken in
    let mut last_turn_spoken = HashMap::<usize, usize>::new();
    loop {
        if count <= data.len() {
            let starting_number = data[count - 1];
            last_diff.insert(starting_number, 0);
            last_turn_spoken.insert(starting_number, count);
            last_number = starting_number;
        } else {
            let new_number = *last_diff.get(&last_number).unwrap();
            if let Some(previous_turn) = last_turn_spoken.get(&new_number) {
                last_diff.insert(new_number, count - *previous_turn);
            } else {
                last_diff.insert(new_number, 0);
            }

            last_turn_spoken.insert(new_number, count);

            last_number = new_number;
        }

        if count == target {
            break;
        }

        count += 1;
    }

    last_number
}

#[test]
pub fn cases() {
    assert_eq!(compute(&vec![1 as usize, 3, 2], 2020), 1);
    assert_eq!(compute(&vec![2 as usize, 1, 3], 2020), 10);
    assert_eq!(compute(&vec![1 as usize, 2, 3], 2020), 27);
    assert_eq!(compute(&vec![2 as usize, 3, 1], 2020), 78);
    assert_eq!(compute(&vec![3 as usize, 2, 1], 2020), 438);
    assert_eq!(compute(&vec![3 as usize, 1, 2], 2020), 1836);
}
