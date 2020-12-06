use std::collections::HashSet;
use std::iter::Iterator;
use std::vec::Vec;

#[aoc_generator(day6)]
pub fn parser(input: &str) -> Vec<String> {
    input.lines().map(|l| l.to_string()).collect::<Vec<_>>()
}

#[aoc(day6, part1)]
pub fn day6_part1(data: &Vec<String>) -> usize {
    let mut count = 0;
    let mut group = HashSet::<char>::new();
    fn process_group(g: &mut HashSet<char>, c: &mut usize) {
        if g.len() > 0 {
            *c += g.len();
            g.clear()
        }
    }
    for line in data {
        if line.len() == 0 {
            process_group(&mut group, &mut count);
        }
        for ch in line.chars() {
            group.insert(ch);
        }
    }

    process_group(&mut group, &mut count);

    count
}

#[aoc(day6, part2)]
pub fn day6_part2(data: &Vec<String>) -> usize {
    let mut count = 0;
    let mut person_answers_for_group = Vec::<HashSet<char>>::new();

    fn process_group(g: &Vec<HashSet<char>>, c: &mut usize) {
        let set1 = &g[0];
        let chars_in_all_sets = set1.iter().filter(|ch| g.iter().all(|s| s.contains(ch)));

        *c += chars_in_all_sets.count();
    }

    for line in data {
        if line.len() == 0 {
            // End of group
            process_group(&person_answers_for_group, &mut count);
            person_answers_for_group.clear();
            continue;
        }

        let mut answers_for_person = HashSet::<char>::new();
        for ch in line.chars() {
            answers_for_person.insert(ch);
        }

        person_answers_for_group.push(answers_for_person);
    }

    process_group(&person_answers_for_group, &mut count);

    count
}
