use multimap::MultiMap;
use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;
use std::hash::Hash;
use std::hash::Hasher;
use std::vec::Vec;

pub type PuzzleInput = Vec<Tile>;

#[derive(Debug, PartialEq, Clone, Copy, Eq)]
pub enum EdgeType {
    North,
    East,
    South,
    West,
}

impl Hash for EdgeType {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            EdgeType::North => 1.hash(state),
            EdgeType::East => 2.hash(state),
            EdgeType::South => 3.hash(state),
            EdgeType::West => 4.hash(state),
        }
    }
}

type EdgeInfo = (EdgeType, String);

#[derive(Clone, Debug)]
pub struct Tile {
    id: usize,
    original_contents: Vec<String>,
    possible_contents: Vec<Vec<String>>,
}

fn generate_all_possible_variants(contents: &Vec<String>) -> Vec<Vec<String>> {
    let mut out = Vec::<Vec<String>>::new();

    let c1 = contents.clone();
    let c2 = rotate_ccw(&c1);
    let c3 = rotate_ccw(&c2);
    let c4 = rotate_ccw(&c3);
    let c5 = flip(&c1);
    let c6 = rotate_ccw(&c5);
    let c7 = rotate_ccw(&c6);
    let c8 = rotate_ccw(&c7);

    out.push(c1);
    out.push(c2);
    out.push(c3);
    out.push(c4);
    out.push(c5);
    out.push(c6);
    out.push(c7);
    out.push(c8);

    out
}

impl Tile {
    pub fn new(id: usize, contents: Vec<String>) -> Tile {
        Tile {
            id,
            possible_contents: generate_all_possible_variants(&contents),
            original_contents: contents,
        }
    }

    pub fn is_already_pinned(&self) -> bool {
        self.possible_contents.len() == 1
    }

    pub fn get_line(&self, index: usize) -> String {
        if !self.is_already_pinned() {
            panic!("cannot retrieve line from unpinned tile");
        }

        self.possible_contents[0][index].clone()
    }

    pub fn get_all_possible_edges(&self) -> Vec<EdgeInfo> {
        let mut out = Vec::<EdgeInfo>::new();

        for contents in &self.possible_contents {
            self.accumulate_edges_for_orientation(&contents, &mut out);
        }

        out
    }

    fn accumulate_edges_for_orientation(&self, contents: &Vec<String>, target: &mut Vec<EdgeInfo>) {
        let size = self.original_contents.len();

        let mut west = String::new();
        let mut east = String::new();

        for i in 0..size {
            let mut chars = contents[i].chars();
            west.push(chars.next().unwrap());
            east.push(chars.last().unwrap());
        }

        target.push((EdgeType::North, contents[0].clone()));
        target.push((EdgeType::East, east));
        target.push((EdgeType::South, contents[size - 1].clone()));
        target.push((EdgeType::West, west));
    }

    pub fn pin_orientation(&mut self, edge: &EdgeInfo) {
        let all_possible = self.get_all_possible_edges();

        let mut edge_index = None;
        for (i, an_edge_info) in all_possible.iter().enumerate() {
            match edge {
                (dir, contents) => {
                    let (an_dir, an_contents) = an_edge_info;
                    if dir == an_dir && contents == an_contents {
                        if edge_index != None {
                            panic!("multiple possible edges for orientation")
                        }
                        edge_index = Some(i);
                    }
                }
            }
        }

        if let Some(e) = edge_index {
            // 4 directions for each tile rotation
            let contents_index = e / 4;
            let good_contents = &self.possible_contents[contents_index];
            self.possible_contents = vec![good_contents.clone()];
        } else {
            panic!("Did not find an orientation with the specified edge");
        }
    }
}

#[aoc_generator(day20)]
pub fn parser(input: &str) -> PuzzleInput {
    lazy_static! {
        static ref TILE_ID_PATTERN: Regex = Regex::new(r"^Tile (?P<number>\d+):").unwrap();
    }

    let mut tile_lines = Vec::<String>::new();
    let mut tile_id = 0;
    let mut tiles = Vec::<Tile>::new();

    for line in input.lines() {
        if line.len() == 0 {
            continue;
        }

        if let Some(m) = TILE_ID_PATTERN.captures(line) {
            // New tile start
            tile_id = m.name("number").unwrap().as_str().parse::<usize>().unwrap();
            continue;
        }

        tile_lines.push(line.to_string());

        if tile_lines.len() == 10 {
            tiles.push(Tile::new(tile_id, tile_lines));
            tile_lines = Vec::<String>::new();
        }
    }

    tiles
}

fn run(all_tiles: &mut HashMap<usize, Tile>) -> MultiMap<usize, (EdgeType, usize)> {
    let mut connection_records = MultiMap::<usize, (EdgeType, usize)>::new();
    let mut fixed_tile_ids_to_process = Vec::<usize>::new();

    // Grab a tile and fix it in position
    let first_tile_id = *all_tiles.iter().nth(0).unwrap().0;
    let first_tile = all_tiles.get_mut(&first_tile_id).unwrap();
    let first_tile_edges = &first_tile.get_all_possible_edges();
    first_tile.pin_orientation(&first_tile_edges[2]);

    fixed_tile_ids_to_process.push(first_tile_id);

    while let Some(fixed_tile_id) = fixed_tile_ids_to_process.pop() {
        let fixed_tile = all_tiles.get(&fixed_tile_id).unwrap();
        let all_possible_edges = fixed_tile.get_all_possible_edges();

        for (edge_type, edge_pattern) in all_possible_edges {
            let looking_for_edge_type = opposite_edge(&edge_type);

            let possible_tiles_ids_for_this_direction = &all_tiles
                .values()
                .filter(|t| {
                    t.get_all_possible_edges().iter().any(
                        |(possible_edge_type, possible_edge_pattern)| {
                            *possible_edge_type == looking_for_edge_type
                                && *possible_edge_pattern == edge_pattern
                        },
                    )
                })
                .map(|tile| tile.id)
                .collect::<Vec<_>>();

            if possible_tiles_ids_for_this_direction.len() == 1 {
                // We can pin that tile
                let tile_id_to_pin = possible_tiles_ids_for_this_direction[0];

                connection_records.insert(fixed_tile_id, (edge_type, tile_id_to_pin).clone());

                let pinned_tile = all_tiles.get_mut(&tile_id_to_pin).unwrap();
                if !pinned_tile.is_already_pinned() {
                    pinned_tile.pin_orientation(&(looking_for_edge_type, edge_pattern));

                    fixed_tile_ids_to_process.push(pinned_tile.id);
                }
            }
        }
    }

    connection_records
}

#[aoc(day20, part1)]
pub fn day20_part1(data: &PuzzleInput) -> usize {
    let mut all_tiles: HashMap<usize, Tile> = data.iter().map(|t| (t.id, t.clone())).collect();
    let connection_records = run(&mut all_tiles);

    let corner_tile_ids = connection_records
        .keys()
        .filter(|tile_id| {
            let neighbours = connection_records.get_vec(tile_id).unwrap();
            neighbours.len() == 2
        })
        .collect::<Vec<_>>();

    corner_tile_ids
        .iter()
        .fold(1, |acc, tile_id| acc * *tile_id)
}

#[aoc(day20, part2)]
pub fn day20_part2(data: &PuzzleInput) -> usize {
    let mut all_tiles: HashMap<usize, Tile> = data.iter().map(|t| (t.id, t.clone())).collect();
    let connection_records = run(&mut all_tiles);

    let picture = build_picture(&connection_records, &all_tiles);

    let mut sea_monster_parts = HashSet::<(usize, usize)>::new();

    let monster = vec![
        "                  # ".to_string(),
        "#    ##    ##    ###".to_string(),
        " #  #  #  #  #  #   ".to_string()];
    let possible_monsters = generate_all_possible_variants(&monster);

    let picture_height = picture.len();
    let picture_width = picture[0].len();

    for possible_monster in possible_monsters {
        let monster_height = possible_monster.len();
        let monster_width = possible_monster[0].len();

        for start_row in 0..picture_height - monster_height {
            for start_col in 0..picture_width - monster_width {
                if is_monster_at_position(&picture, &possible_monster, start_row, start_col) {
                    println!("found monster at {} {}", start_row, start_col);

                    // Accumulate monster position
                    for row_offset in 0..monster_height {
                        let picture_row_index = start_row + row_offset;
                
                        for col_offset in 0..monster_width {
                            let picture_col_index = start_col + col_offset;
                            let monster_char = possible_monster[row_offset].chars().nth(col_offset).unwrap();
                            if monster_char == '#' {
                                sea_monster_parts.insert((picture_row_index, picture_col_index));
                            }
                        }
                    }
                }
            }
        }
    }

    let mut roughness = 0;
    for row in 0..picture_height {
        for col in 0..picture_width {
            if picture[row].chars().nth(col).unwrap() == '#' && !sea_monster_parts.contains(&(row, col)) {
                roughness += 1;
            }
        }
    }

    roughness
}

fn is_monster_at_position(picture: &Vec<String>, possible_monster: &Vec<String>, start_row: usize, start_col: usize) -> bool {
    let monster_height = possible_monster.len();
    let monster_width = possible_monster[0].len();

    for row_offset in 0..monster_height {
        let picture_row_index = start_row + row_offset;

        for col_offset in 0..monster_width {
            let picture_col_index = start_col + col_offset;
            let monster_char = possible_monster[row_offset].chars().nth(col_offset).unwrap();
            if monster_char == '#' {
                let picture_char = picture[picture_row_index].chars().nth(picture_col_index).unwrap();
                if picture_char != '#' {
                    return false;
                }
            }
        }
    }

    return true;
}

fn get_tile_in_direction(
    connections: &MultiMap<usize, (EdgeType, usize)>,
    tile_id: usize,
    direction: EdgeType,
) -> Option<usize> {
    let links = connections.get_vec(&tile_id).unwrap();
    for (edge_type, target_tile_id) in links {
        if *edge_type == direction {
            return Some(*target_tile_id);
        }
    }
    None
}

fn build_picture(
    connections: &MultiMap<usize, (EdgeType, usize)>,
    all_tiles: &HashMap<usize, Tile>,
) -> Vec<String> {
    let mut output = Vec::<String>::new();

    let northwest_tile_id = find_northwest_tile_id(&connections);
    let mut first_tile_id = northwest_tile_id;

    // loop around rows of tiles
    loop {
        // loop around rows IN tiles
        // skip line 0 and 9: they're the borders of the tiles, which are to be ignored
        for tile_row_index in 1..9 {
            let mut line = String::new();
            let mut tile_id = first_tile_id;

            // loop around columns in tile rows
            loop {
                let tile = all_tiles.get(&tile_id).unwrap();
                let contents = tile.get_line(tile_row_index);
                let contents_without_border = contents[1..9].to_string();
                line.push_str(&contents_without_border);

                if let Some(next_tile_id) = get_tile_in_direction(&connections, tile_id, EdgeType::East) {
                    tile_id = next_tile_id;
                } else {
                    break;
                }
            }

            output.push(line);
        }

        if let Some(next_tile_row_first_id) = get_tile_in_direction(&connections, first_tile_id, EdgeType::South) {
            first_tile_id = next_tile_row_first_id;
        } else {
            break;
        }
    }
    output
}

fn find_northwest_tile_id(connections: &MultiMap<usize, (EdgeType, usize)>) -> usize {
    // Pick any tile to start
    let mut tile_id = *connections.iter().nth(0).unwrap().0;

    while let Some(next_tile_id) = get_tile_in_direction(&connections, tile_id, EdgeType::North) {
        tile_id = next_tile_id;
    }
    while let Some(next_tile_id) = get_tile_in_direction(&connections, tile_id, EdgeType::West) {
        tile_id = next_tile_id;
    }
    tile_id
}

fn opposite_edge(edge_type: &EdgeType) -> EdgeType {
    match edge_type {
        EdgeType::North => EdgeType::South,
        EdgeType::South => EdgeType::North,
        EdgeType::East => EdgeType::West,
        EdgeType::West => EdgeType::East,
    }
}

// This copes with a non-square
fn rotate_ccw(contents: &Vec<String>) -> Vec<String> {
    let input_row_count = contents.len();
    let input_col_count = contents[0].len();

    let mut output = Vec::<String>::new();

    for output_row in 0..input_col_count {
        let mut row = String::new();

        for output_col in 0..input_row_count {
            let anti_j = (input_col_count - 1) - output_row;

            row.push(contents[output_col].chars().nth(anti_j).unwrap());
        }
        output.push(row);
    }
    output
}

fn flip(contents: &Vec<String>) -> Vec<String> {
    contents.iter().map(|l| l.chars().rev().collect()).collect()
}
