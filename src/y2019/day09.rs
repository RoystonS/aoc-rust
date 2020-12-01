use crate::y2019::intcode::{Action, InstructionByte, IntCodeInterpreter};
use std::iter::Iterator;
use std::vec::Vec;

#[aoc_generator(day9)]
pub fn parser(input: &str) -> Vec<InstructionByte> {
    input
        .trim()
        .split(",")
        .map(|line| line.parse::<InstructionByte>().unwrap())
        .collect::<Vec<InstructionByte>>()
}

fn run(instructions: &Vec<InstructionByte>, input: isize) -> String {
    let memory = instructions.clone();
    let mut interp = IntCodeInterpreter::new(&memory);

    interp.write_input(input);

    let mut outputs = Vec::<isize>::new();

    loop {
        let action = interp.run();
        match action {
            Action::Halt => break,
            Action::Output(o) => {
                outputs.push(o);
            }
        }
    }

    format!("{:?}", outputs)
}

#[aoc(day9, part1)]
pub fn part1(instructions: &Vec<InstructionByte>) -> String {
    run(instructions, 1)
}

#[aoc(day9, part2)]
pub fn part2(instructions: &Vec<InstructionByte>) -> String {
    run(instructions, 2)
}

#[test]
pub fn quine_test() {
    let original_program = vec![
        109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99,
    ];

    let mut prog = IntCodeInterpreter::new(&original_program);
    let mut output = Vec::<isize>::new();

    loop {
        let out = prog.run();
        match out {
            Action::Output(o) => {
                output.push(o);
            }
            Action::Halt => {
                break;
            }
            _ => unimplemented!("Unexpected action result"),
        }
    }

    assert_eq!(output, original_program);
}
