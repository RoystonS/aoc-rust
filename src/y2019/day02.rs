use crate::y2019::intcode::{InstructionByte, IntCodeInterpreter};
use std::iter::Iterator;
use std::vec::Vec;

#[aoc_generator(day2)]
pub fn parser(input: &str) -> Vec<InstructionByte> {
    input
        .split(",")
        .map(|line| line.parse::<InstructionByte>().unwrap())
        .collect::<Vec<InstructionByte>>()
}

#[aoc(day2, part1)]
pub fn day2_part1(instructions: &Vec<isize>) -> isize {
    let mut memory = instructions.clone();
    memory[1] = 12;
    memory[2] = 2;

    let mut interp = IntCodeInterpreter::new(&memory);
    interp.execute();
    interp.memory[0]
}

#[aoc(day2, part2)]
pub fn day1_part2(instructions: &Vec<InstructionByte>) -> Option<InstructionByte> {
    for verb in 0..99 {
        for noun in 0..99 {
            let mut memory = instructions.clone();
            memory[1] = noun;
            memory[2] = verb;
            let mut interp = IntCodeInterpreter::new(&memory);
            interp.run();
            if interp.memory[0] == 19690720 {
                return Some(100 * noun + verb);
            }
        }
    }
    None
}

#[test]
pub fn tests() {
    let inst: [InstructionByte; 12] = [1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50];
    let mut prog = IntCodeInterpreter::new(&inst.to_vec());
    prog.run();
    assert_eq!(3500, prog.memory[0]);
}

#[test]
pub fn test2() {
    pub fn run(input: &[InstructionByte]) -> Vec<isize> {
        let mem = input.to_vec();
        let mut prog = IntCodeInterpreter::new(&mem);
        prog.run();
        prog.memory
    }

    assert_eq!(run(&[1, 0, 0, 0, 99]), [2, 0, 0, 0, 99]);
    assert_eq!(run(&[2, 3, 0, 3, 99]), [2, 3, 0, 6, 99]);
    assert_eq!(run(&[2, 4, 4, 5, 99, 0]), [2, 4, 4, 5, 99, 9801]);
    assert_eq!(
        run(&[1, 1, 1, 4, 99, 5, 6, 0, 99]),
        [30, 1, 1, 4, 2, 5, 6, 0, 99]
    );
}
