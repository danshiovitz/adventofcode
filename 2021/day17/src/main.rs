use std::ops::Range;

use itertools::Itertools;
use lazy_regex::regex;

extern crate common;

use common::framework::{parse_lines, run_day, BaseDay, InputReader};
use common::grid::{add_direction, Coord, Direction};

#[derive(PartialEq, Eq, Copy, Clone, Debug, Hash)]
struct Area {
    min: Coord,
    max: Coord,
}

struct Day17 {
    vals: Vec<Area>,
}

fn fire_probe(start: Coord, dir: Direction, area: Area, verbose: bool) -> Option<i32> {
    let mut cur = start;
    let mut cur_dir = dir;
    let mut max_y = 0;
    if verbose {
        println!(
            "Firing probe at ({},{}) trying to hit ({},{}..{},{})",
            dir.dx, dir.dy, area.min.x, area.min.y, area.max.x, area.max.y
        );
    }
    while cur.y + cur_dir.dy >= area.min.y {
        cur = add_direction(&cur, &cur_dir);
        if cur_dir.dx > 0 {
            cur_dir.dx -= 1;
        } else if cur_dir.dx < 0 {
            cur_dir.dx += 1;
        }
        cur_dir.dy -= 1;
        if verbose {
            println!(
                "Probe is at ({},{}), dir is ({},{})",
                cur.x, cur.y, cur_dir.dx, cur_dir.dy
            );
        }

        if cur.y > max_y {
            max_y = cur.y;
        }
        if cur.x >= area.min.x && cur.x <= area.max.x && cur.y >= area.min.y && cur.y <= area.max.y
        {
            return Some(max_y);
        }
    }
    return None;
}

impl BaseDay for Day17 {
    fn parse(&mut self, input: &mut InputReader) {
        fn parse_line(line: String) -> Area {
            let rex = regex!(r#"target area: x=(-?\d+)\.\.(-?\d+),\s*y=(-?\d+)\.\.(-?\d+)"#);
            match rex.captures(&line) {
                Some(c) => {
                    return Area {
                        min: Coord {
                            x: c[1].parse::<i32>().unwrap(),
                            y: c[3].parse::<i32>().unwrap(),
                        },
                        max: Coord {
                            x: c[2].parse::<i32>().unwrap(),
                            y: c[4].parse::<i32>().unwrap(),
                        },
                    };
                }
                None => panic!("Bad line: {}", &line),
            };
        }
        self.vals = parse_lines(input, &mut parse_line);
    }

    fn pt1(&mut self) -> String {
        let area = self.vals[0];
        let dxs: Range<i32> = 0..area.max.x + 1; // assumes target area x > 0
        let dys: Range<i32> = area.min.y - 1..area.max.x + 1;
        let start = Coord { x: 0, y: 0 };
        let ret = dxs
            .cartesian_product(dys)
            .filter_map(|(dx, dy)| fire_probe(start, Direction { dx: dx, dy: dy }, area, false))
            .max()
            .unwrap();
        return ret.to_string();
    }

    fn pt2(&mut self) -> String {
        let area = self.vals[0];
        let dxs: Range<i32> = 0..area.max.x + 1; // assumes target area x > 0
        let dys: Range<i32> = area.min.y - 1..area.max.x + 1;
        let start = Coord { x: 0, y: 0 };
        let ret = dxs
            .cartesian_product(dys)
            .filter_map(|(dx, dy)| fire_probe(start, Direction { dx: dx, dy: dy }, area, false))
            .count();
        return ret.to_string();
    }
}

fn main() {
    let mut day = Day17 { vals: Vec::new() };
    run_day(&mut day);
}
