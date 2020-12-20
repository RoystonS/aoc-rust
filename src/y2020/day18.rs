use regex::Regex;

pub enum PrecedenceMode {
    Equal,
    PlusOverMinus,
}

#[derive(Debug)]
pub enum ExpressionNode {
    Operator(char),
    Number(i64),
}

#[derive(Copy, Clone, Debug)]
pub enum Symbol {
    Number(i64),
    Operator(char),
    OpenParen,
    CloseParen,
}

pub fn lex(line: &str) -> Vec<Symbol> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"((\d+)|([\+\*\(\)]))").unwrap();
    }
    RE.find_iter(line)
        .map(|m| {
            let text = m.as_str();
            match text {
                "*" => Symbol::Operator('*'),
                "+" => Symbol::Operator('+'),
                "(" => Symbol::OpenParen,
                ")" => Symbol::CloseParen,
                _ => Symbol::Number(text.parse::<i64>().unwrap()),
            }
        })
        .collect::<Vec<_>>()
}

pub fn to_postfix(symbols: &Vec<Symbol>, mode: PrecedenceMode) -> Vec<ExpressionNode> {
    let mut result = Vec::<ExpressionNode>::new();
    let mut stack = Vec::<Symbol>::new();

    for symbol in symbols {
        match symbol {
            Symbol::OpenParen => {
                stack.push(*symbol);
            }
            Symbol::Number(n) => {
                result.push(ExpressionNode::Number(*n));
            }
            Symbol::CloseParen => loop {
                if stack.len() == 0 {
                    break;
                }
                let last_stack = stack.pop().unwrap();
                match last_stack {
                    Symbol::OpenParen => break,
                    Symbol::Operator(op_char) => {
                        result.push(ExpressionNode::Operator(op_char));
                    }
                    _ => {
                        panic!(
                            "Unexpected value on stack when closing paren: {:?}",
                            last_stack
                        );
                    }
                }
            },
            Symbol::Operator(_) => {
                // Clear out some of the stack
                loop {
                    let peek = stack.last();
                    match peek {
                        None => break,
                        Some(Symbol::OpenParen) => break,
                        Some(Symbol::Operator(top_stack_op_char)) => {
                            match mode {
                                PrecedenceMode::PlusOverMinus => {
                                    if *top_stack_op_char == '*' {
                                        break;
                                    }
                                }
                                _ => {}
                            }
                            result.push(ExpressionNode::Operator(*top_stack_op_char));
                            stack.pop();
                        }
                        _ => {
                            panic!(
                                "Unexpected value on stack when processing operator: {:?}",
                                peek
                            );
                        }
                    }
                }

                stack.push(*symbol);
            }
        }
    }

    loop {
        match stack.pop() {
            None => break,
            Some(Symbol::Operator(op_char)) => {
                result.push(ExpressionNode::Operator(op_char));
            }
            _ => unimplemented!("Unexpected at end of traversal"),
        }
    }
    result
}

pub fn execute_postfix(expression: &Vec<ExpressionNode>) -> i64 {
    let mut stack = Vec::<i64>::new();

    for node in expression {
        match node {
            ExpressionNode::Number(n) => stack.push(*n),
            ExpressionNode::Operator('+') => {
                let op1 = stack.pop().unwrap();
                let op2 = stack.pop().unwrap();
                stack.push(op1 + op2);
            }
            ExpressionNode::Operator('*') => {
                let op1 = stack.pop().unwrap();
                let op2 = stack.pop().unwrap();
                stack.push(op1 * op2);
            }
            _ => unimplemented!("Unexpected postfix value"),
        }
    }

    stack[0]
}

#[aoc_generator(day18)]
pub fn parser(input: &str) -> Vec<Vec<Symbol>> {
    input.lines().map(lex).collect::<Vec<_>>()
}

#[aoc(day18, part1)]
pub fn day18_part1(data: &Vec<Vec<Symbol>>) -> i64 {
    data.iter().fold(0, |acc, symbols| {
        acc + execute_postfix(&to_postfix(symbols, PrecedenceMode::Equal))
    })
}

#[aoc(day18, part2)]
pub fn day18_part2(data: &Vec<Vec<Symbol>>) -> i64 {
    data.iter().fold(0, |acc, symbols| {
        acc + execute_postfix(&to_postfix(symbols, PrecedenceMode::PlusOverMinus))
    })
}

#[test]
pub fn test_cases() {
    fn run(s: &str) -> (i64, i64) {
        (
            execute_postfix(&to_postfix(&lex(s), PrecedenceMode::Equal)),
            execute_postfix(&to_postfix(&lex(s), PrecedenceMode::PlusOverMinus)),
        )
    }

    assert_eq!(run("2 * 3 + (4 * 5)"), (26, 46));
    assert_eq!(run("5 + (8 * 3 + 9 + 3 * 4 * 3)"), (437, 1445));
    assert_eq!(
        run("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))"),
        (12240, 669060)
    );
    assert_eq!(
        run("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"),
        (13632, 23340)
    );
}
