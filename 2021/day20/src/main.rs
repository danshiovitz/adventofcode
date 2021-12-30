use std::collections::{HashMap, HashSet};

extern crate common;

use common::framework::{parse_grid, parse_lines, run_day, BaseDay, InputReader};
use common::grid::{Coord, Grid};

struct Day20 {
    algo: HashSet<i32>,
    image: HashMap<Coord, bool>,
}

fn enhance(
    image: &HashMap<Coord, bool>,
    algo: &HashSet<i32>,
    rounds: i32,
    verbose: bool,
) -> HashMap<Coord, bool> {
    let mut cur = image.clone();

    fn flip(bit: u8, coord: Coord, image: &HashMap<Coord, bool>, default: bool) -> i32 {
        if *image.get(&coord).unwrap_or(&default) {
            return 1 << bit;
        } else {
            return 0;
        }
    }

    let get_default = |round: i32| -> bool {
        return algo.contains(&0) && round % 2 == 1;
    };

    for round in 0..rounds {
        let min_x = cur.keys().map(|c| c.x).min().unwrap();
        let min_y = cur.keys().map(|c| c.y).min().unwrap();
        let max_x = cur.keys().map(|c| c.x).max().unwrap();
        let max_y = cur.keys().map(|c| c.y).max().unwrap();

        let mut nxt = HashMap::new();
        for x in min_x - 1..max_x + 2 {
            for y in min_y - 1..max_y + 2 {
                let mut key = 0;
                key |= flip(8, Coord { x: x - 1, y: y - 1 }, &cur, get_default(round));
                key |= flip(7, Coord { x: x + 0, y: y - 1 }, &cur, get_default(round));
                key |= flip(6, Coord { x: x + 1, y: y - 1 }, &cur, get_default(round));
                key |= flip(5, Coord { x: x - 1, y: y + 0 }, &cur, get_default(round));
                key |= flip(4, Coord { x: x + 0, y: y + 0 }, &cur, get_default(round));
                key |= flip(3, Coord { x: x + 1, y: y + 0 }, &cur, get_default(round));
                key |= flip(2, Coord { x: x - 1, y: y + 1 }, &cur, get_default(round));
                key |= flip(1, Coord { x: x + 0, y: y + 1 }, &cur, get_default(round));
                key |= flip(0, Coord { x: x + 1, y: y + 1 }, &cur, get_default(round));
                if key < 0 || key >= 512 {
                    panic!("Bad key: {}", key);
                }
                if algo.contains(&key) {
                    nxt.insert(Coord { x: x, y: y }, true);
                } else {
                    nxt.insert(Coord { x: x, y: y }, false);
                }
            }
        }
        cur = nxt;

        if verbose {
            let min_x = cur.keys().map(|c| c.x).min().unwrap();
            let min_y = cur.keys().map(|c| c.y).min().unwrap();
            let max_x = cur.keys().map(|c| c.x).max().unwrap();
            let max_y = cur.keys().map(|c| c.y).max().unwrap();

            for y in min_y - 1..max_y + 2 {
                for x in min_x - 1..max_x + 2 {
                    if *cur
                        .get(&Coord { x: x, y: y })
                        .unwrap_or(&(get_default(round)))
                    {
                        print!("#");
                    } else {
                        print!(".");
                    }
                }
                println!();
            }
            println!();
            println!();
        }
    }

    return cur;
}

impl BaseDay for Day20 {
    fn parse(&mut self, input: &mut InputReader) {
        let mut parse_line = |line: String| -> bool {
            if line.len() != 512 {
                panic!("Ack");
            }
            self.algo = line
                .chars()
                .enumerate()
                .filter_map(|(idx, v)| if v == '#' { Some(idx as i32) } else { None })
                .collect();
            return true;
        };
        parse_lines(input, &mut parse_line);

        fn parse_coord(c: char, _coord: &Coord) -> Option<bool> {
            if c == '#' {
                Some(true)
            } else {
                Some(false)
            }
        }
        self.image = parse_grid(input, &mut parse_coord).coords;
    }

    fn pt1(&mut self) -> String {
        let enhanced = enhance(&self.image, &self.algo, 2, false);
        return enhanced.values().filter(|v| **v).count().to_string();
    }

    fn pt2(&mut self) -> String {
        let enhanced = enhance(&self.image, &self.algo, 50, false);
        return enhanced.values().filter(|v| **v).count().to_string();
    }
}

fn main() {
    let mut day = Day20 { algo: HashSet::new(), image: HashMap::new() };
    run_day(&mut day);
}
