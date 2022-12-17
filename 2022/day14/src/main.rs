use lazy_regex::regex;
use std::collections::HashMap;

extern crate common;

use common::framework::{parse_lines, run_day, BaseDay, InputReader};
use common::grid::{add_direction, get_unit_direction, Coord, Direction, Grid};

struct Day14 {
    vals: Grid<char>,
}

fn flow_one(
    grid: &mut HashMap<Coord, char>,
    source: Coord,
    max_depth: i32,
    has_floor: bool,
) -> Option<Coord> {
    let downs = vec![
        Direction { dx: 0, dy: 1 },
        Direction { dx: -1, dy: 1 },
        Direction { dx: 1, dy: 1 },
    ];

    let mut cur = source;

    'outer: while cur.y <= max_depth {
        //println!("Flowing {:?} ...", cur);
        for down in &downs {
            let new_cur = add_direction(&cur, down);
            if !grid.contains_key(&new_cur) {
                cur = new_cur;
                continue 'outer;
            }
        }

        //println!("Settled!");
        return Some(cur);
    }

    if has_floor {
        //println!("Fell on floor {}!", max_depth);
        return Some(cur);
    } else {
        //println!("Fell off {}!", max_depth);
        return None;
    }
}

fn flow_sand(vals: &Grid<char>, source: Coord, has_floor: bool) -> i32 {
    let mut grid = vals.coords.clone();
    let mut rounds = 0;
    loop {
        match flow_one(&mut grid, source, vals.max.y, has_floor) {
            Some(c) => {
                grid.insert(c, 'o');
                rounds += 1;
                if c == source {
                    return rounds;
                }
            }
            None => {
                return rounds;
            }
        }
    }
}

impl BaseDay for Day14 {
    fn parse(&mut self, input: &mut InputReader) {
        fn parse_line(line: &String) -> Vec<Coord> {
            let sep = regex!(r#"\s*->\s*"#);
            let ws = sep.split(line.trim());
            let joints = ws.map(|n| Coord::parse(n)).collect::<Vec<Coord>>();

            let mut ret = Vec::new();
            for i in 1..joints.len() {
                let mut cur = joints[i - 1];
                while cur != joints[i] {
                    ret.push(cur);
                    cur = add_direction(&cur, &get_unit_direction(&joints[i - 1], &joints[i]));
                }
            }
            ret.push(joints[joints.len() - 1]);
            return ret;
        }

        parse_lines(input, &mut |line: String| {
            self.vals
                .coords
                .extend(parse_line(&line).into_iter().map(|c| (c, '#')))
        });

        self.vals.recompute_minmax();
    }

    fn pt1(&mut self) -> String {
        let grains = flow_sand(&self.vals, Coord { x: 500, y: 0 }, false);
        return grains.to_string();
    }

    fn pt2(&mut self) -> String {
        let grains = flow_sand(&self.vals, Coord { x: 500, y: 0 }, true);
        return grains.to_string();
    }
}

fn main() {
    let mut day = Day14 { vals: Grid::new() };
    run_day(&mut day);
}
