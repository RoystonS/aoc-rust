use std::collections::HashMap;
use std::fmt;
use std::ops::RangeInclusive;

#[derive(Clone)]
pub struct ThreeDMap {
    min_x: isize,
    min_y: isize,
    min_z: isize,
    min_w: isize,
    max_x: isize,
    max_y: isize,
    max_z: isize,
    max_w: isize,

    map: HashMap<String, bool>,
}

impl fmt::Display for ThreeDMap {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut out = "".to_owned();

        for w in self.wrange() {
            for z in self.zrange() {
                out.push_str(format!("z={}, w={}\r\n", z, w).as_str());
                for y in self.yrange() {
                    for x in self.xrange() {
                        out.push_str(if self.get(x, y, z, w) { "#" } else { "." });
                    }
                    out.push_str("\r\n");
                }
            }
        }

        write!(f, "{}", out)
    }
}

impl ThreeDMap {
    pub fn new() -> ThreeDMap {
        ThreeDMap {
            min_x: isize::MAX,
            min_y: isize::MAX,
            min_z: isize::MAX,
            min_w: isize::MAX,
            max_x: isize::MIN,
            max_y: isize::MIN,
            max_z: isize::MIN,
            max_w: isize::MIN,

            map: HashMap::new(),
        }
    }

    pub fn xrange(&self) -> RangeInclusive<isize> {
        (self.min_x - 1)..=(self.max_x + 1)
    }
    pub fn yrange(&self) -> RangeInclusive<isize> {
        (self.min_y - 1)..=(self.max_y + 1)
    }
    pub fn zrange(&self) -> RangeInclusive<isize> {
        (self.min_z - 1)..=(self.max_z + 1)
    }
    pub fn wrange(&self) -> RangeInclusive<isize> {
        (self.min_w - 1)..=(self.max_w + 1)
    }

    pub fn get(&self, x: isize, y: isize, z: isize, w: isize) -> bool {
        let key = format!("{}:{}:{}:{}", z, y, x, w);
        match self.map.get(&key) {
            Some(value) => *value,
            None => false,
        }
    }

    pub fn set(&mut self, x: isize, y: isize, z: isize, w: isize, value: bool) {
        let key = format!("{}:{}:{}:{}", z, y, x, w);
        if value {
            self.map.insert(key, value);
        } else {
            self.map.remove(&key);
        }

        self.min_x = self.min_x.min(x);
        self.min_y = self.min_y.min(y);
        self.min_z = self.min_z.min(z);
        self.min_w = self.min_w.min(w);
        self.max_x = self.max_x.max(x);
        self.max_y = self.max_y.max(y);
        self.max_z = self.max_z.max(z);
        self.max_w = self.max_w.max(w);
    }

    pub fn surrounding(&self, x: isize, y: isize, z: isize, w: isize) -> usize {
        let mut count = 0;

        for px in x - 1..=x + 1 {
            for py in y - 1..=y + 1 {
                for pz in z - 1..=z + 1 {
                    for pw in w - 1..=w + 1 {
                        if px == x && py == y && pz == z && pw == w {
                            continue;
                        }
                        count += if self.get(px, py, pz, pw) { 1 } else { 0 }
                    }
                }
            }
        }

        count
    }

    pub fn active_count(&self) -> usize {
        self.map.len()
    }
}

#[aoc_generator(day17)]
pub fn parser(input: &str) -> ThreeDMap {
    let mut map = ThreeDMap::new();

    for (y, l) in input.lines().enumerate() {
        for (x, c) in l.chars().enumerate() {
            if c == '#' {
                map.set(x as isize, y as isize, 0, 0, true);
            }
        }
    }

    map
}

#[aoc(day17, part1)]
pub fn day17_part1(data: &ThreeDMap) -> usize {
    // Run 4D game of life with w dimension always at 0
    run(data, |_map| 0..=0)
}

#[aoc(day17, part2)]
pub fn day17_part2(data: &ThreeDMap) -> usize {
    // Run 4D game of life
    run(data, |map| map.wrange())
}

fn run<F>(data: &ThreeDMap, wranger: F) -> usize
where
    F: Fn(&ThreeDMap) -> RangeInclusive<isize>,
{
    let mut previous = data.clone();
    let mut c = 0;

    loop {
        let mut next = previous.clone();

        for w in wranger(&previous) {
            for z in previous.zrange() {
                for y in previous.yrange() {
                    for x in previous.xrange() {
                        let active = previous.get(x, y, z, w);
                        let neighbours = previous.surrounding(x, y, z, w);

                        if active && (neighbours < 2 || neighbours > 3) {
                            next.set(x, y, z, w, false);
                        }
                        if !active && neighbours == 3 {
                            next.set(x, y, z, w, true);
                        }
                    }
                }
            }
        }

        previous = next;

        c += 1;

        if c >= 6 {
            break;
        }
    }
    previous.active_count()
}
