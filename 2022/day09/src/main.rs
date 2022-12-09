use lazy_regex::regex;
use std::collections::HashSet;

extern crate common;

use common::framework::{parse_lines, run_day, BaseDay, InputReader};
use common::grid::{add_direction, get_unit_direction, Coord, Direction};

struct Day09 {
    vals: Vec<Direction>,
}

fn unstretch(rh: Coord, rt: Coord) -> Coord {
    let ax = rh.x - rt.x;
    let ay = rh.y - rt.y;
    if ax.abs() >= 2 || ay.abs() >= 2 {
        return add_direction(&rt, &get_unit_direction(&rt, &rh));
    } else {
        return rt;
    }
}

fn stretch(num_knots: i32, vals: &Vec<Direction>) -> usize {
    let mut knots = Vec::new();
    for _ in 0..num_knots {
        knots.push(Coord { x: 0, y: 0 });
    }
    let mut visited = HashSet::new();
    visited.insert(*knots.last().unwrap());

    for dir in vals {
        knots[0] = add_direction(&knots[0], dir);
        for i in 1..knots.len() {
            knots[i] = unstretch(knots[i - 1], knots[i]);
            // println!("head: {:?}, tail: {:?}", rh, rt);
        }
        visited.insert(*knots.last().unwrap());
    }
    return visited.len();
}

impl BaseDay for Day09 {
    fn parse(&mut self, input: &mut InputReader) {
        let parsed = parse_lines(input, &mut |line: String| {
            let rex = regex!(r#"([LRUD])\s+(\d+)"#);
            match rex.captures(&line) {
                Some(c) => {
                    let amt = c[2].parse::<i32>().unwrap();
                    if &c[1] == "L" {
                        return (Direction { dx: -1, dy: 0 }, amt);
                    } else if &c[1] == "R" {
                        return (Direction { dx: 1, dy: 0 }, amt);
                    } else if &c[1] == "U" {
                        return (Direction { dx: 0, dy: 1 }, amt);
                    } else {
                        return (Direction { dx: 0, dy: -1 }, amt);
                    }
                }
                None => {
                    panic!("Bad line: {}", line);
                }
            }
        });
        for (dir, amt) in parsed {
            for _ in 0..amt {
                self.vals.push(dir);
            }
        }
    }

    fn pt1(&mut self) -> String {
        return stretch(2, &self.vals).to_string();
    }

    fn pt2(&mut self) -> String {
        return stretch(10, &self.vals).to_string();
    }
}

fn main() {
    let mut day = Day09 { vals: Vec::new() };
    run_day(&mut day);
}
