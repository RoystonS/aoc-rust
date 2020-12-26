use std::collections::HashSet;
use std::vec::Vec;

#[derive(Debug)]
pub enum Direction {
    East,
    SouthEast,
    SouthWest,
    West,
    NorthWest,
    NorthEast,
}
pub type PuzzleLine = Vec<Direction>;
pub type PuzzleInput = Vec<PuzzleLine>;

#[derive(Debug)]
enum TileColour {
    White,
    Black,
}

struct Bounds {
    min_col: isize,
    max_col: isize,
    min_row: isize,
    max_row: isize,
}

#[derive(Debug)]
struct Tiles {
    set: HashSet<(isize, isize)>,
}

impl Tiles {
    pub fn new() -> Tiles {
        Tiles {
            set: HashSet::new(),
        }
    }

    pub fn get_color(&self, pos: (isize, isize)) -> TileColour {
        if self.set.contains(&pos) {
            TileColour::Black
        } else {
            TileColour::White
        }
    }

    pub fn set_color(&mut self, pos: (isize, isize), color: TileColour) {
        match color {
            TileColour::White => {
                self.set.remove(&pos);
            }
            TileColour::Black => {
                self.set.insert(pos);
            }
        }
    }

    pub fn swap(&mut self, pos: (isize, isize)) {
        let color = self.get_color(pos);
        let opposite = match color {
            TileColour::White => TileColour::Black,
            TileColour::Black => TileColour::White,
        };
        self.set_color(pos, opposite);
    }

    pub fn bounds(&self) -> Bounds {
        let tiles = &self.set;

        Bounds {
            min_row: *tiles.iter().map(|(row, _col)| row).min().unwrap(),
            max_row: *tiles.iter().map(|(row, _col)| row).max().unwrap(),
            min_col: *tiles.iter().map(|(_row, col)| col).min().unwrap(),
            max_col: *tiles.iter().map(|(_row, col)| col).max().unwrap(),
        }
    }

    pub fn black_count(&self) -> usize {
        self.set.len()
    }

    pub fn surrounding_black_count(&self, pos: (isize, isize)) -> usize {
        let surroundings = vec![
            compute_move(pos, &Direction::East),
            compute_move(pos, &Direction::West),
            compute_move(pos, &Direction::NorthEast),
            compute_move(pos, &Direction::NorthWest),
            compute_move(pos, &Direction::SouthEast),
            compute_move(pos, &Direction::SouthWest),
        ];

        surroundings
            .iter()
            .filter(|pos| self.set.contains(pos))
            .count()
    }
}

#[aoc_generator(day24)]
pub fn parser(input: &str) -> PuzzleInput {
    input
        .lines()
        .map(|l| {
            let mut out = Vec::<Direction>::new();
            let mut chars = l.chars();

            while let Some(ch) = chars.next() {
                let dir = match ch {
                    'e' => Direction::East,
                    'w' => Direction::West,
                    's' => {
                        if let Some('w') = chars.next() {
                            Direction::SouthWest
                        } else {
                            Direction::SouthEast
                        }
                    }
                    'n' => {
                        if let Some('w') = chars.next() {
                            Direction::NorthWest
                        } else {
                            Direction::NorthEast
                        }
                    }
                    _ => panic!("Unexpected char {}", ch),
                };

                out.push(dir);
            }

            out
        })
        .collect()
}

fn run(data: &PuzzleInput) -> Tiles {
    let mut row;
    let mut col;
    let mut tiles = Tiles::new();

    for line in data {
        row = 0;
        col = 0;

        for dir in line {
            let (new_row, new_col) = compute_move((row, col), dir);
            row = new_row;
            col = new_col;
        }
        let pos = (row, col);
        tiles.swap(pos);
    }
    tiles
}

fn compute_move(pos: (isize, isize), direction: &Direction) -> (isize, isize) {
    let (row, col) = pos;

    match direction {
        Direction::East => (row, col + 1),
        Direction::West => (row, col - 1),
        Direction::NorthEast => (row - 1, col + 1),
        Direction::SouthWest => (row + 1, col - 1),
        Direction::NorthWest => (row - 1, col),
        Direction::SouthEast => (row + 1, col),
    }
    // match direction {
    //     Direction::East => (row, col + 2),
    //     Direction::West => (row, col - 2),
    //     Direction::NorthEast => (row - 1, col + 1),
    //     Direction::SouthWest => (row + 1, col - 1),
    //     Direction::NorthWest => (row - 1, col - 1),
    //     Direction::SouthEast => (row + 1, col + 1),
    // }
}

#[aoc(day24, part1)]
pub fn day24_part1(data: &PuzzleInput) -> usize {
    let tiles = run(data);
    tiles.black_count()
}

#[aoc(day24, part2)]
pub fn day24_part2(data: &PuzzleInput) -> usize {
    let mut tiles = run(data);
    // let mut tiles = Tiles::new();

    loop {

        for _day in 0..100 {
            let bounds = tiles.bounds();
            let mut new_tiles = Tiles::new();

            for row in (bounds.min_row - 2)..=(bounds.max_row + 2) {
                for col in (bounds.min_col - 2)..=(bounds.max_col + 2) {
                    let pos = (row, col);
                    let black_count = tiles.surrounding_black_count(pos);

                    let color = tiles.get_color(pos);
                    match color {
                        TileColour::Black => {
                            if black_count == 0 || black_count > 2 {
                                new_tiles.set_color(pos, TileColour::White);
                            } else {
                                new_tiles.set_color(pos, TileColour::Black);
                            }
                        }
                        TileColour::White => {
                            if black_count == 2 {
                                new_tiles.set_color(pos, TileColour::Black);
                            } else {
                                new_tiles.set_color(pos, TileColour::White);
                            }
                        }
                    }
                }
            }

            tiles = new_tiles;
        }
        break;
    }

    tiles.black_count()
}
