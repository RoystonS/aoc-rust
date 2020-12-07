use regex::Regex;
use std::collections::HashSet;
use std::iter::Iterator;
use std::vec::Vec;

pub struct TypeAndCount {
    typ: String,
    count: usize,
}
pub struct Rule {
    bag: String,
    contains: Vec<TypeAndCount>,
}

// light red bags contain 1 bright white bag, 2 muted yellow bags

#[aoc_generator(day7)]
pub fn parser(input: &str) -> Vec<Rule> {
    lazy_static! {
        static ref OUTER_PATTERN: Regex =
            Regex::new(r"^(?P<bag>.*?) bags? contain (?P<contains>.*).$").unwrap();
        static ref INNER_PATTERN: Regex = Regex::new(r"(?P<count>\d+) (?P<bag>.*?) bags?").unwrap();
    }

    input
        .lines()
        .map(|l| {
            let x = OUTER_PATTERN.captures(l).unwrap();
            let container_bag = x.name("bag").unwrap().as_str().to_string();
            let contains = x.name("contains").unwrap().as_str();

            let contained = INNER_PATTERN
                .captures_iter(contains)
                .map(|c| {
                    let count = c.name("count").unwrap().as_str().parse::<usize>().unwrap();
                    let contained_bag = c.name("bag").unwrap().as_str().to_string();
                    TypeAndCount {
                        typ: contained_bag,
                        count,
                    }
                })
                .collect::<Vec<_>>();

            Rule {
                bag: container_bag,
                contains: contained,
            }
        })
        .collect::<Vec<_>>()
}

#[aoc(day7, part1)]
pub fn day7_part1(data: &Vec<Rule>) -> usize {
    let mut set = HashSet::<String>::new();
    fn add_containers(data: &Vec<Rule>, bag: String, set: &mut HashSet<String>) {
        for rule in data {
            for x in &rule.contains {
                if x.typ == bag {
                    // We've found a bag that can contain ours
                    if set.insert(rule.bag.to_string()) {
                        add_containers(data, rule.bag.to_string(), set);
                    }
                }
            }
        }
    }

    add_containers(data, "shiny gold".to_string(), &mut set);

    set.len()
}

#[aoc(day7, part2)]
pub fn day7_part2(data: &Vec<Rule>) -> usize {
    let mut count = 0;

    fn accumulate(count: usize, bag: String, data: &Vec<Rule>, acc: &mut usize) {
        let rule = data.iter().find(|rule| rule.bag == bag).unwrap();
        
        *acc += count;

        for contents in &rule.contains {
            accumulate(count * contents.count, contents.typ.to_string(), data, acc);
        }
    }

    accumulate(1, "shiny gold".to_string(), data, &mut count);

    // count contains the total number of bags INCLUDING the outermost one.
    // We want to know what's inside, so subtract 1
    count - 1
}
