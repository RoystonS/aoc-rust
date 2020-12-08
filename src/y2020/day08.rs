use std::collections::HashSet;
use std::iter::Iterator;
use std::vec::Vec;

#[derive(Debug, Clone)]
pub enum Instruction {
    Nop(Parameter),
    Acc(Parameter),
    Jmp(Parameter),
}

#[derive(Debug, Clone, Copy)]
pub enum Parameter {
    Relative(isize),
}

pub enum Response {
    InfiniteLoop(isize),
    Terminates(isize)
}

#[aoc_generator(day8)]
pub fn parser(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .map(|line| {
            let bits = line.split(' ').collect::<Vec<_>>();
            let delta = bits[1].parse::<isize>().unwrap();
            let inst = bits[0];
            match inst {
                "acc" => Instruction::Acc(Parameter::Relative(delta)),
                "nop" => Instruction::Nop(Parameter::Relative(delta)),
                "jmp" => Instruction::Jmp(Parameter::Relative(delta)),
                _ => unimplemented!(),
            }
        })
        .collect::<Vec<_>>()
}

fn run_instructions(instructions: &Vec<Instruction>) -> Response {
    let mut acc: isize = 0;
    let mut pc: usize = 0;

    let mut pcs_seen = HashSet::<usize>::new();

    loop {
        if pcs_seen.contains(&pc) {
            // Loop
            return Response::InfiniteLoop(acc);
        }

        let inst = &instructions[pc];
        pcs_seen.insert(pc);
        match inst {
            Instruction::Nop(_) => {
                pc += 1;
            }
            Instruction::Acc(Parameter::Relative(delta)) => {
                acc += delta;
                pc += 1;
            }
            Instruction::Jmp(Parameter::Relative(delta)) => {
                pc = ((pc as isize) + delta) as usize;
            }
        };

        if pc >= instructions.len() {
            return Response::Terminates(acc);
        }
    }
}

#[aoc(day8, part1)]
pub fn day8_part1(data: &Vec<Instruction>) -> isize {
    match run_instructions(&data) {
        Response::InfiniteLoop(acc) => acc,
        Response::Terminates(_) => unimplemented!()
    }
}

#[aoc(day8, part2)]
pub fn day8_part2(data: &Vec<Instruction>) -> isize {
    for i in 0..data.len() {
        let mut instructions_clone= data.clone();

        // Try tweaking the program
        let muti = data.get(i);
        match muti {
            Some(Instruction::Nop(p1)) => {
                instructions_clone[i] = Instruction::Jmp(*p1);
            },
            Some(Instruction::Jmp(param)) => {
                instructions_clone[i] = Instruction::Nop(*param);
            },
            _ =>{}
        }


        let result = run_instructions(&instructions_clone);
        match result {
            Response::Terminates(value) => {
                // Hooray - it terminated!
                return value;
            },
            _ => {}
        }
    }

    unimplemented!("No terminating program found");
}
