use lazy_regex::regex;
use std::collections::HashMap;

extern crate common;

use common::framework::{parse_lines, run_day, BaseDay, InputReader};

#[derive(Debug)]
struct Line {
    start: (i32, i32),
    end: (i32, i32),
}

struct Day05 {
    vals: Vec<Line>,
}

fn count_points(vals: &Vec<Line>, diag: bool) -> String {
    let mut point_counts: HashMap<(i32, i32), i32> = HashMap::new();
    for val in vals {
        let mx = if val.start.0 < val.end.0 {
            1
        } else if val.start.0 > val.end.0 {
            -1
        } else {
            0
        };
        let my = if val.start.1 < val.end.1 {
            1
        } else if val.start.1 > val.end.1 {
            -1
        } else {
            0
        };

        if !diag && mx != 0 && my != 0 {
            println!("Skipping: {:?}", val);
            continue;
        }

        let mut cur = val.start;
        loop {
            if let Some(v) = point_counts.get_mut(&cur) {
                *v += 1;
            } else {
                point_counts.insert(cur, 1);
            }
            if cur == val.end {
                break;
            } else {
                cur = (cur.0 + mx, cur.1 + my);
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
                        start: (c[1].parse::<i32>().unwrap(), c[2].parse::<i32>().unwrap()),
                        end: (c[3].parse::<i32>().unwrap(), c[4].parse::<i32>().unwrap()),
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
