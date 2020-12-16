use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Debug)]
struct RuleRange {
    min: usize,
    max: usize,
}
#[derive(Debug)]
struct Rule {
    key_name: String,
    ranges: Vec<RuleRange>,
}

type Ticket = Vec<usize>;

pub struct PuzzleInput {
    rules: Vec<Rule>,
    your_ticket: Ticket,
    nearby_tickets: Vec<Ticket>,
}

#[derive(Debug)]
enum ParserMode {
    Rule,
    YourTicket,
    NearbyTickets,
}

#[aoc_generator(day16)]
pub fn parser(input: &str) -> PuzzleInput {
    let mut mode = ParserMode::Rule;
    lazy_static! {
        static ref RULE_PATTERN: Regex = Regex::new(
            r"^(?P<key>.*): (?P<r1min>\d+)-(?P<r1max>\d+) or (?P<r2min>\d+)-(?P<r2max>\d+)$"
        )
        .unwrap();
    }

    let mut rules = Vec::<Rule>::new();
    let mut your_ticket: Option<Ticket> = None;
    let mut nearby_tickets = Vec::<Ticket>::new();

    input.lines().for_each(|l| {
        match l {
            "your ticket:" => {
                mode = ParserMode::YourTicket;
                return;
            }
            "nearby tickets:" => {
                mode = ParserMode::NearbyTickets;
                return;
            }
            "" => {
                return;
            }
            _ => {
                match mode {
                    // Can't quite be bothered parsing N range rules; there are always 2
                    ParserMode::Rule => {
                        let caps = RULE_PATTERN.captures(l).unwrap();
                        let mut ranges = Vec::<RuleRange>::new();
                        ranges.push(RuleRange {
                            min: caps
                                .name("r1min")
                                .unwrap()
                                .as_str()
                                .parse::<usize>()
                                .unwrap(),
                            max: caps
                                .name("r1max")
                                .unwrap()
                                .as_str()
                                .parse::<usize>()
                                .unwrap(),
                        });
                        ranges.push(RuleRange {
                            min: caps
                                .name("r2min")
                                .unwrap()
                                .as_str()
                                .parse::<usize>()
                                .unwrap(),
                            max: caps
                                .name("r2max")
                                .unwrap()
                                .as_str()
                                .parse::<usize>()
                                .unwrap(),
                        });

                        rules.push(Rule {
                            key_name: caps.name("key").unwrap().as_str().to_string(),
                            ranges,
                        });
                    }
                    ParserMode::YourTicket => {
                        your_ticket = Some(
                            l.split(',')
                                .map(|n| n.parse::<usize>().unwrap())
                                .collect::<Vec<_>>(),
                        );
                    }
                    ParserMode::NearbyTickets => {
                        nearby_tickets.push(
                            l.split(',')
                                .map(|n| n.parse::<usize>().unwrap())
                                .collect::<Vec<_>>(),
                        );
                    }
                }
            }
        }
    });

    PuzzleInput {
        rules,
        your_ticket: your_ticket.unwrap(),
        nearby_tickets,
    }
}

#[aoc(day16, part1)]
pub fn day16_part1(data: &PuzzleInput) -> usize {
    data.nearby_tickets.iter().fold(0, |acc, ticket| {
        acc + ticket.iter().fold(0, |acc2, ticket_field| {
            if data.rules.iter().any(|rule| {
                rule.ranges
                    .iter()
                    .any(|range| *ticket_field >= range.min && *ticket_field <= range.max)
            }) {
                acc2 + 0
            } else {
                acc2 + ticket_field
            }
        })
    })
}

#[aoc(day16, part2)]
pub fn day16_part(data: &PuzzleInput) -> usize {
    let valid_nearby_tickets = data
        .nearby_tickets
        .iter()
        .filter(|ticket| {
            ticket.iter().all(|ticket_field| {
                data.rules.iter().any(|rule| {
                    rule.ranges
                        .iter()
                        .any(|range| *ticket_field >= range.min && *ticket_field <= range.max)
                })
            })
        })
        .collect::<Vec<_>>();

    let value_indexes = (0..data.your_ticket.len()).collect::<Vec<usize>>();

    // A map from rule field name to a set of field indexes that are all valid for that rule.
    // It may be, for instance, that rule A is valid for fields 2 and 3, rule B is valid for fields 1, 2 and 3
    // and rule C is only valid for rule 3.  Armed with that we can eventually calculate that rule B must be for field 1.
    let mut field_validity = HashMap::<String, HashSet<&usize>>::new();
    for rule in &data.rules {
        let matching_value_indexes = value_indexes
            .iter()
            .filter(|value_index| {
                let satisfies_all_tickets = valid_nearby_tickets.iter().all(|t| {
                    let t2 = *t;
                    let ticket_field = t2[**value_index];
                    let satisfies_rule = rule
                        .ranges
                        .iter()
                        .any(|range| ticket_field >= range.min && ticket_field <= range.max);

                    satisfies_rule
                });
                satisfies_all_tickets
            })
            .collect::<HashSet<_>>();

        field_validity.insert(rule.key_name.to_string(), matching_value_indexes);
    }

    // Compute the actual indexes for each field
    let output = process(&mut field_validity);

    output.iter().filter_map(|(field_name, field_index)| {
        if field_name.starts_with("departure") {
            Some(data.your_ticket[*field_index])
        } else {
            None
        }
    }).fold(1, |value, acc| acc * value)
}

fn process(data: &mut HashMap<String, HashSet<&usize>>) -> HashMap<String, usize> {
    let mut result = HashMap::<String, usize>::new();

    loop {
        if data.len() == 0 {
            break;
        }

        let (rule_key_name, rule_field) = get_first_field_with_only_one_possible_index(data);
        result.insert(rule_key_name.to_string(), rule_field);
        data.remove(&rule_key_name);

        // Remove that index from all the other fields
        for k in data.values_mut() {
            k.remove(&rule_field);
        }
    }

    result
}

fn get_first_field_with_only_one_possible_index(data: &HashMap<String, HashSet<&usize>>) -> (String, usize) {
    let (rule_key_name, rule_fields) = data
        .iter()
        .filter(|(_key_name, fields)| fields.len() == 1)
        .next()
        .unwrap();

    (rule_key_name.to_string(), **rule_fields.iter().next().unwrap())
}
