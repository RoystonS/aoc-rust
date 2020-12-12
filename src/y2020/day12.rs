use std::vec::Vec;

#[derive(Debug)]
pub enum Instruction {
    North(usize),
    South(usize),
    East(usize),
    West(usize),
    Left(usize),
    Right(usize),
    Forward(usize),
}

enum ForwardMoveType {
    Ship,
    Waypoint,
}

#[aoc_generator(day12)]
pub fn parser(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .map(|line| {
            let chars = line.chars().collect::<Vec<_>>();
            let instr_char = chars[0];
            let rest = chars[1..]
                .iter()
                .collect::<String>()
                .parse::<usize>()
                .unwrap();

            match instr_char {
                'N' => Instruction::North(rest),
                'E' => Instruction::East(rest),
                'S' => Instruction::South(rest),
                'W' => Instruction::West(rest),
                'L' => Instruction::Left(rest),
                'R' => Instruction::Right(rest),
                'F' => Instruction::Forward(rest),
                _ => unimplemented!(),
            }
        })
        .collect::<Vec<_>>()
}

fn run(data: &Vec<Instruction>, move_type: ForwardMoveType) -> (isize, isize) {
    let mut y: isize = 0;
    let mut x: isize = 0;
    let mut dx: isize = 1;
    let mut dy: isize = 0;

    for instruction in data {
        match instruction {
            Instruction::Forward(len) => {
                y += dy * *len as isize;
                x += dx * *len as isize;
            }
            Instruction::Left(amount) => match rotate(dx, dy, -(*amount as isize)) {
                (new_dx, new_dy) => {
                    dx = new_dx;
                    dy = new_dy;
                }
            },
            Instruction::Right(amount) => match rotate(dx, dy, *amount as isize) {
                (new_dx, new_dy) => {
                    dx = new_dx;
                    dy = new_dy;
                }
            },
            Instruction::North(len) => match move_type {
                ForwardMoveType::Ship => y -= *len as isize,
                ForwardMoveType::Waypoint => dy -= *len as isize,
            },
            Instruction::South(len) => match move_type {
                ForwardMoveType::Ship => y += *len as isize,
                ForwardMoveType::Waypoint => dy += *len as isize,
            },
            Instruction::West(len) => match move_type {
                ForwardMoveType::Ship => x -= *len as isize,
                ForwardMoveType::Waypoint => dx -= *len as isize,
            },
            Instruction::East(len) => match move_type {
                ForwardMoveType::Ship => x += *len as isize,
                ForwardMoveType::Waypoint => dx += *len as isize,
            },
        }
    }

    (x, y)
}

#[aoc(day12, part1)]
pub fn day12_part1(data: &Vec<Instruction>) -> isize {
    match run(data, ForwardMoveType::Ship) {
        (x, y) => x.abs() + y.abs(),
    }
}

#[aoc(day12, part2)]
pub fn day12_part2(data: &Vec<Instruction>) -> isize {
    match run(data, ForwardMoveType::Waypoint) {
        (x, y) => x.abs() + y.abs(),
    }
}

fn rotate(x: isize, y: isize, angle: isize) -> (isize, isize) {
    let yf = y as f32;
    let xf = x as f32;
    let anglef = (angle as f32).to_radians();

    let sin_angle = anglef.sin();
    let cos_angle = anglef.cos();

    return (
        (xf * cos_angle - yf * sin_angle).round() as isize,
        (xf * sin_angle + yf * cos_angle).round() as isize,
    );
}
