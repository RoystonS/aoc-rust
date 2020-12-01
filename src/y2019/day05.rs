use crate::y2019::intcode::{Action, InstructionByte, IntCodeInterpreter};
use std::iter::Iterator;
use std::vec::Vec;

#[aoc_generator(day5)]
pub fn parser(input: &str) -> Vec<InstructionByte> {
    input
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

#[aoc(day5, part1)]
pub fn day5_part1(instructions: &Vec<InstructionByte>) -> String {
    run(instructions, 1)
}

#[aoc(day5, part2)]
pub fn day5_part2(instructions: &Vec<InstructionByte>) -> String {
    run(instructions, 5)
}

#[test]
pub fn tests() {
    let inst: [InstructionByte; 5] = [1002, 4, 3, 4, 33];
    let mut prog = IntCodeInterpreter::new(&inst.to_vec());
    prog.run();
    assert_eq!(99, prog.memory[4]);
}

#[test]
pub fn test2() {
    let mut prog = IntCodeInterpreter::new(&[1101, 100, -1, 4, 0].to_vec());
    prog.run();
    assert_eq!(99, prog.memory[4]);
}

#[test]
pub fn echo_test() {
    let mut prog = IntCodeInterpreter::new(&[3, 0, 4, 0, 99].to_vec());
    prog.write_input(42);
    let out = prog.run();
    match out {
        Action::Output(o) => {
            assert_eq!(42, o);
        }
        _ => unimplemented!("Unexpected action result"),
    }
}
