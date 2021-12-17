use std::collections::HashSet;

use lazy_regex::regex;

extern crate common;

use common::framework::{parse_lines, run_day, BaseDay, InputReader};
use common::grid::{print_grid, Coord, Grid};

#[derive(Copy, Clone)]
enum Fold {
    X(i32),
    Y(i32),
}

struct Day13 {
    points: HashSet<Coord>,
    folds: Vec<Fold>,
}

fn print_one(_c: &Coord, maybe_val: Option<&i32>) -> String {
    if maybe_val.is_some() {
        return "#".to_owned();
    } else {
        return ".".to_owned();
    }
}

fn print_points(points: &HashSet<Coord>) {
    let max_x = points.iter().map(|p| p.x).max().unwrap();
    let max_y = points.iter().map(|p| p.y).max().unwrap();
    let grid = Grid {
        min: Coord { x: 0, y: 0 },
        max: Coord { x: max_x, y: max_y },
        coords: points.iter().map(|p| (*p, 1)).collect(),
    };
    print_grid(&grid, &mut print_one);
}

fn fold_points(points: &HashSet<Coord>, fold: Fold) -> HashSet<Coord> {
    let mut ret = HashSet::new();
    for point in points {
        match fold {
            Fold::X(x) => {
                if point.x > x {
                    let new_x = 2 * x - point.x;
                    ret.insert(Coord {
                        x: new_x,
                        y: point.y,
                    });
                } else {
                    ret.insert(Coord {
                        x: point.x,
                        y: point.y,
                    });
                }
            }
            Fold::Y(y) => {
                if point.y > y {
                    let new_y = 2 * y - point.y;
                    ret.insert(Coord {
                        x: point.x,
                        y: new_y,
                    });
                } else {
                    ret.insert(Coord {
                        x: point.x,
                        y: point.y,
                    });
                }
            }
        }
    }
    return ret;
}

impl BaseDay for Day13 {
    fn parse(&mut self, input: &mut InputReader) {
        let mut parse_coord = |line: String| -> Coord {
            let sep = regex!(r#"\s*,\s*"#);
            let pieces: Vec<&str> = sep.split(&line).collect();
            return Coord {
                x: pieces[0].parse::<i32>().unwrap(),
                y: pieces[1].parse::<i32>().unwrap(),
            };
        };

        let mut parse_fold = |line: String| -> Fold {
            let rex = regex!(r#"fold along (x|y)=(\d+)"#);
            match rex.captures(&line) {
                Some(c) => {
                    let val = c[2].parse::<i32>().unwrap();
                    match &c[1] {
                        "x" => return Fold::X(val),
                        "y" => return Fold::Y(val),
                        _ => panic!("Bad fold: {}", &line),
                    };
                }
                None => panic!("Bad line: {}", &line),
            }
        };

        self.points.extend(parse_lines(input, &mut parse_coord));
        self.folds = parse_lines(input, &mut parse_fold);
    }

    fn pt1(&mut self) -> String {
        let folded = fold_points(&self.points, self.folds[0]);
        // print_points(&folded);
        return folded.len().to_string();
    }

    fn pt2(&mut self) -> String {
        let mut folded = self.points.clone();
        for fold in &self.folds {
            folded = fold_points(&folded, *fold);
        }
        print_points(&folded);
        return "".to_string();
    }
}

fn main() {
    let mut day = Day13 {
        points: HashSet::new(),
        folds: Vec::new(),
    };
    run_day(&mut day);
}
