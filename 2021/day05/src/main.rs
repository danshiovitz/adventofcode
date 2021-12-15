use lazy_regex::regex;
use std::collections::HashMap;

extern crate common;

use common::framework::{parse_lines, run_day, BaseDay, InputReader};
use common::grid::{Coord, Line, get_unit_direction, add_direction};
use common::utils::{inc_counter};

struct Day05 {
    vals: Vec<Line>,
}

fn count_points(vals: &Vec<Line>, diag: bool) -> String {
    let mut point_counts: HashMap<Coord, i32> = HashMap::new();
    for val in vals {
        let dir = get_unit_direction(&val.start, &val.end);

        if !diag && dir.dx != 0 && dir.dy != 0 {
            println!("Skipping: {:?}", val);
            continue;
        }

        let mut cur = val.start;
        loop {
            inc_counter(&mut point_counts, cur, 1);
            if cur == val.end {
                break;
            } else {
                cur = add_direction(&cur, &dir);
            }
        }
    }
    let cnts = point_counts.values().filter(|v| **v > 1).count();
    return cnts.to_string();
}

impl BaseDay for Day05 {
    fn parse(&mut self, input: &mut InputReader) {
        fn parse_line(line: String) -> Line {
            let rex = regex!(r#"(-?\d+),\s*(-?\d+)\s*->\s*(-?\d+),\s*(-?\d+)"#);
            match rex.captures(&line) {
                Some(c) => {
                    return Line {
                        start: Coord { x: c[1].parse::<i32>().unwrap(), y: c[2].parse::<i32>().unwrap() },
                        end: Coord { x: c[3].parse::<i32>().unwrap(), y: c[4].parse::<i32>().unwrap() },
                    };
                }
                None => panic!("Bad line: {}", &line),
            };
        }
        self.vals = parse_lines(input, &mut parse_line);
    }

    fn pt1(&mut self) -> String {
        return count_points(&self.vals, false);
    }

    fn pt2(&mut self) -> String {
        return count_points(&self.vals, true);
    }
}

fn main() {
    let mut day = Day05 { vals: Vec::new() };
    run_day(&mut day);
}
