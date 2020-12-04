use crate::y2019::intcode::{Action, InstructionByte, IntCodeInterpreter};
use std::collections::HashMap;
use std::iter::Iterator;
use std::vec::Vec;

#[aoc_generator(day11)]
pub fn parser(input: &str) -> Vec<InstructionByte> {
    input
        .trim()
        .split(",")
        .map(|line| line.parse::<InstructionByte>().unwrap())
        .collect::<Vec<InstructionByte>>()
}

#[derive(PartialEq, Eq, Hash)]
struct Pos {
    row: isize,
    col: isize,
}

fn run(instructions: &Vec<InstructionByte>, paint: &mut HashMap<Pos, isize>) {
    let memory = instructions.clone();
    let mut interp = IntCodeInterpreter::new(&memory);

    let mut row = 0;
    let mut col = 0;

    let mut rowdelta = -1;
    let mut coldelta = 0;

    loop {
        let current_pos = Pos { row, col };

        let paint_color = *paint.get(&current_pos).unwrap_or(&0);

        // Provide current paint color to the robot
        interp.write_input(paint_color);
        let action1 = interp.run();

        match action1 {
            Action::Halt => break,
            Action::Output(o) => {
                paint.insert(current_pos, o);
            }
        }

        let action2 = interp.run();
        match action2 {
            Action::Output(o) => {
                match o {
                    0 => {
                        // CCW
                        let temp = coldelta;
                        coldelta = rowdelta;
                        rowdelta = -temp;
                    }
                    1 => {
                        // CW
                        let temp = rowdelta;
                        rowdelta = coldelta;
                        coldelta = -temp;
                    }
                    _ => unimplemented!("unexpected direction"),
                }
            }
            _ => unimplemented!("unexpected action1"),
        }

        row += rowdelta;
        col += coldelta;
    }
}

#[aoc(day11, part1)]
pub fn day11_part1(instructions: &Vec<InstructionByte>) -> usize {
    let mut paint = HashMap::<Pos, isize>::new();

    run(instructions, &mut paint);

    paint.len()
}

fn find_min_max<'a, I>(vals: I) -> (isize, isize)
where
    I: Iterator<Item = &'a isize>,
    I: Clone,
{
    let min = *vals.clone().min().unwrap();
    let max = *vals.max().unwrap();
    (min, max)
}

#[aoc(day11, part2)]
pub fn day11_part2(instructions: &Vec<InstructionByte>) -> String {
    let mut paint = HashMap::<Pos, isize>::new();
    paint.insert(Pos { col: 0, row: 0 }, 1);
    run(instructions, &mut paint);

    let positions = paint.iter().map(|(pos, _)| pos);
    let columns = positions.clone().map(|Pos { row: _, col }| col);
    let rows = positions.map(|Pos { row, col: _ }| row);
    let (mincol, maxcol) = find_min_max(columns);
    let (minrow, maxrow) = find_min_max(rows);

    let mut lines = Vec::<String>::new();
    lines.push(String::new());

    for row in minrow..=maxrow {
        let mut chars = Vec::<&str>::new();

        for col in mincol..=maxcol {
            let pcol = *paint.get(&Pos { row, col }).unwrap_or(&0);
            chars.push(if pcol == 0 { " " } else { "*" });
        }
        let line = chars.join("");
        lines.push(line);
    }

    lines.join("\r\n")
}

#[test]
pub fn pos_hash() {
    let mut hash = HashMap::<Pos, &str>::new();

    let k1 = Pos { row: 1, col: 2 };
    let k2 = Pos { row: 2, col: 1 };

    hash.insert(k1, "k1");
    hash.insert(k2, "k2");

    let new_k1 = Pos { row: 1, col: 2 };
    let v1 = *hash.get(&new_k1).unwrap();
    assert_eq!(v1, "k1");
}
