use std::{collections::HashMap, iter::Iterator};
use std::vec::Vec;

#[aoc_generator(day10)]
pub fn parser(input: &str) -> Vec<usize> {
    input
        .lines()
        .map(|line| line.parse::<usize>().unwrap())
        .collect::<Vec<_>>()
}

#[aoc(day10, part1)]
pub fn day10_part1(data: &Vec<usize>) -> usize {
    let mut sorted = data.clone();
    sorted.sort();
    
    let mut current = 0;
    let mut diff1 = 0;
    let mut diff3 = 1; // always one 3-diff at the end

    for adapter in sorted {
        let diff = adapter - current;
        match diff {
            1 => { diff1 += 1 },
            3 => { diff3 += 1 },
            _ => {}
        };
        current = adapter;
    }

    diff1 * diff3
}

#[aoc(day10, part2)]
pub fn day10_part2(data: &Vec<usize>) -> usize {
    let maximum = data.iter().fold(0 as usize, |acc,x| acc.max(*x)) + 3;

    ways_to_get_to(data, maximum, &mut HashMap::new())
}

fn ways_to_get_to(data: &Vec<usize>, n: usize, cache: &mut HashMap<usize, usize>) -> usize {
    if n == 0 { return 1; }

    // memoization is necessary other the complexity is similar to
    // a naive Fibonacci but with 3 precursors instead of 2
    match cache.get(&n).map(|entry| entry.clone()) {
        Some(result) => result,
        None => {
            // There are 3 adapters we could have come from.
            // If they exist, each provides some number of ways to get to n
            let mut ways = 0;

            let m1 = n - 1;
            if m1 == 0 || data.contains(&m1) {
                ways += ways_to_get_to(data, m1, cache);
            }

            let m2 = n - 2;
            if m2 == 0 || data.contains(&m2) {
                ways += ways_to_get_to(data, m2, cache);
            }

            let m3 = n - 3;
            if m3 == 0 || data.contains(&m3) {
                ways += ways_to_get_to(data, m3, cache);
            }

            cache.insert(n, ways);
            ways
        }
    }
}