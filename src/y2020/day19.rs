use regex::Regex;
use std::collections::HashMap;
use std::vec::Vec;

#[derive(Clone)]
pub enum Rule {
    Char(char),
    RuleSequences(Vec<RuleSequence>),
}

pub type RuleSequence = Vec<usize>;

pub struct PuzzleInput {
    rules: HashMap<usize, Rule>,
    messages: Vec<String>,
}

#[aoc_generator(day19)]
pub fn parser(input: &str) -> PuzzleInput {
    lazy_static! {
        static ref RULE_PATTERN: Regex =
            Regex::new(r"^(?P<number>\d+): (?P<rule_content>.*)$").unwrap();
    }

    let mut rules = HashMap::<usize, Rule>::new();
    let mut messages = Vec::<String>::new();

    for line in input.lines() {
        if let Some(m) = RULE_PATTERN.captures(line) {
            let number = m.name("number").unwrap().as_str().parse::<usize>().unwrap();
            let rule_content = m.name("rule_content").unwrap().as_str();
            rules.insert(number, parse_rule_content(rule_content));
        } else {
            messages.push(line.to_string());
        }
    }

    PuzzleInput { rules, messages }
}

fn parse_rule_content(s: &str) -> Rule {
    lazy_static! {
        static ref TERMINAL_RULE: Regex = Regex::new(r#"^"(?P<char>.)""#).unwrap();
    }

    if let Some(captures) = TERMINAL_RULE.captures(s) {
        Rule::Char(captures.name("char").unwrap().as_str().chars().next().unwrap())
    } else {
        Rule::RuleSequences(
            s.split(" | ")
                .map(|ored| {
                    ored.split(" ")
                        .map(|num| num.parse::<usize>().unwrap())
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>(),
        )
    }
}

fn is_valid(message: &str, rules: &HashMap<usize, Rule>, rule_sequence: &[usize]) -> bool {
    if rule_sequence.len() == 0 {
        return message.len() == 0;
    }

    let mut chars = message.chars();
    let first_char_of_message = chars.next();
    let message_rest = chars.as_str();

    match first_char_of_message {
        None => false,
        Some(first_char) => {
            // We process our rules strictly from left-to-right, otherwise
            // madness ensues. Happily, the way the input rules are written,
            // that isn't a problem.
            let first_rule_number = rule_sequence[0];
            let first_rule = rules.get(&first_rule_number).unwrap();
            let rules_rest = &rule_sequence[1..];
        
            match first_rule {
                Rule::Char(rule_char) => {
                    // Terminal rule; it's a match if the
                    // terminal characters match and the rest of the string matches
                    // the rest of our rules
                    *rule_char == first_char && is_valid(message_rest, rules, rules_rest)
                }
                Rule::RuleSequences(sequences) => sequences.iter().any(|seq| {
                    // Non-terminal rule; we basically decompose the first rule
                    // down into its child options. If any match, it matches.
                    
                    // It matches if the entire message matches the decomposed
                    // first rule followed by the rest of the rules
                    let mut combined_rules = seq.clone();
                    combined_rules.extend(rules_rest);
                    is_valid(message, rules, &combined_rules)
                })
            }        
        }
    }
}

#[aoc(day19, part1)]
pub fn day19_part1(data: &PuzzleInput) -> usize {
    data.messages.iter().filter(|m| is_valid(m, &data.rules, &[0])).count()
}

#[aoc(day19, part2)]
pub fn day19_part2(data: &PuzzleInput) -> usize {
    let mut modified_rules = data.rules.clone();
    modified_rules.insert(8, Rule::RuleSequences(vec![vec![42], vec![42, 8]]));
    modified_rules.insert(11, Rule::RuleSequences(vec![vec![42, 31], vec![42, 11, 31]]));

    data.messages.iter().filter(|m| is_valid(m, &modified_rules, &[0])).count()
}
