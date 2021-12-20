use std::collections::{HashMap, HashSet};
use std::io::BufRead;

use itertools::Itertools;
use lazy_regex::regex;

extern crate common;

use common::framework::{run_day, BaseDay, InputReader};
use common::grid::{Coord3d, Direction3d, add_direction3d, find_direction3d, sub_direction3d, manhattan3d};

struct Scanner {
    name: String,
    beacons: Vec<Coord3d>,
}

const MAX_ROTATIONS: u8 = 24;

#[derive(PartialEq, Eq, Copy, Clone, Debug, Hash)]
struct Orientation {
    translation: Direction3d,
    rotation: u8, // valid range here is 0-23, seems like too many for an enum
}

struct Day19 {
    vals: Vec<Scanner>,
    orientations: Vec<Orientation>,
    all_beacons: HashSet<Coord3d>,
}

const MIN_REQUIRED_OVERLAPS: usize = 12;

fn find_possible_overlap(absolute: &Vec<Coord3d>, relative: &Vec<Coord3d>) -> Option<(Orientation, HashSet<Coord3d>)> {
    let abs_set: HashSet<Coord3d> = HashSet::from_iter(absolute.iter().map(|c| *c));

    let abs_diff_pairs = find_abs_diff_pairs(&absolute, &relative);
    for pair in &abs_diff_pairs {
        for orient in compute_orientations(&pair.0.0, &pair.1.0) {
            // see if this works for the other half of the diff line
            if to_absolute(&pair.1.1, &orient) != pair.0.1 {
                continue;
            }
            // if so it's worth trying everywhere
            let corrected: HashSet<Coord3d> = relative.iter().map(|c| to_absolute(c, &orient)).collect();
            let num_overlaps = abs_set.intersection(&corrected).count();
            if num_overlaps >= MIN_REQUIRED_OVERLAPS {
                return Some((orient, corrected));
            }
        }
    }
    return None;
}

fn find_abs_diff_pairs(absolute: &Vec<Coord3d>, relative: &Vec<Coord3d>) -> Vec<((Coord3d, Coord3d), (Coord3d, Coord3d))> {
    fn do_single(coords: &Vec<Coord3d>) -> HashMap<i32, Vec<(Coord3d, Coord3d)>> {
        let mut ret: HashMap<i32, Vec<(Coord3d, Coord3d)>> = HashMap::new();
        // For simplicity later, put in X-Y and Y-X as two separate entries
        for i in 0..coords.len() {
            for j in 0..coords.len() {
                if i == j {
                    continue;
                }
                let diff = (coords[i].x - coords[j].x).abs() + (coords[i].y - coords[j].y).abs() + (coords[i].z - coords[j].z).abs();
                if let Some(cur) = ret.get_mut(&diff) {
                    cur.push((coords[i], coords[j]));
                } else {
                    ret.insert(diff, vec![(coords[i], coords[j])]);
                }
            }
        }
        return ret;
    }

    let pdiffs = do_single(absolute);
    let sdiffs = do_single(relative);

    let mut both = Vec::new();
    for (diff, pair) in &pdiffs {
        if let Some(other) = sdiffs.get(&diff) {
            for p in pair {
                for o in other {
                    both.push((*p, *o));
                }
            }
        }
    }
    return both;
}

fn compute_orientations(absolute: &Coord3d, relative: &Coord3d) -> Vec<Orientation> {
    let mut ret = Vec::new();
    for rot in 0..MAX_ROTATIONS {
        let rotated = rotate(relative, rot);
        let dir = find_direction3d(&rotated, absolute);
        ret.push(Orientation { translation: dir, rotation: rot });
    }
    return ret;
}

fn to_absolute(coord: &Coord3d, orientation: &Orientation) -> Coord3d {
    let mut cur = *coord;
    cur = rotate(&cur, orientation.rotation);
    return add_direction3d(&cur, &orientation.translation);
}

#[allow(dead_code)]
fn to_relative(coord: &Coord3d, orientation: &Orientation) -> Coord3d {
    let mut cur = *coord;
    cur = sub_direction3d(&cur, &orientation.translation);
    return unrotate(&cur, orientation.rotation);
}

fn rotate(coord: &Coord3d, rotation: u8) -> Coord3d {
    return match rotation {
        0 => Coord3d { x: coord.x, y: coord.y, z: coord.z },
        1 => Coord3d { x: coord.x, y: coord.z, z: -coord.y },
        2 => Coord3d { x: coord.x, y: -coord.y, z: -coord.z },
        3 => Coord3d { x: coord.x, y: -coord.z, z: coord.y },

        4 => Coord3d { x: -coord.x, y: coord.y, z: -coord.z },
        5 => Coord3d { x: -coord.x, y: coord.z, z: coord.y },
        6 => Coord3d { x: -coord.x, y: -coord.y, z: coord.z },
        7 => Coord3d { x: -coord.x, y: -coord.z, z: -coord.y },

        8 => Coord3d { x: coord.y, y: coord.x, z: -coord.z },
        9 => Coord3d { x: coord.y, y: coord.z, z: coord.x },
        10 => Coord3d { x: coord.y, y: -coord.x, z: coord.z },
        11 => Coord3d { x: coord.y, y: -coord.z, z: -coord.x },

        12 => Coord3d { x: -coord.y, y: coord.x, z: coord.z },
        13 => Coord3d { x: -coord.y, y: coord.z, z: -coord.x },
        14 => Coord3d { x: -coord.y, y: -coord.x, z: -coord.z },
        15 => Coord3d { x: -coord.y, y: -coord.z, z: coord.x },

        16 => Coord3d { x: coord.z, y: coord.x, z: coord.y },
        17 => Coord3d { x: coord.z, y: coord.y, z: -coord.x },
        18 => Coord3d { x: coord.z, y: -coord.x, z: -coord.y },
        19 => Coord3d { x: coord.z, y: -coord.y, z: coord.x },

        20 => Coord3d { x: -coord.z, y: coord.x, z: -coord.y },
        21 => Coord3d { x: -coord.z, y: coord.y, z: coord.x },
        22 => Coord3d { x: -coord.z, y: -coord.x, z: coord.y },
        23 => Coord3d { x: -coord.z, y: -coord.y, z: -coord.x },

        r => panic!("bad rotation: {}", r),
    };
}

#[allow(dead_code)]
fn unrotate(coord: &Coord3d, rotation: u8) -> Coord3d {
    let map = HashMap::from([
        (0, 0),
        (1, 3),
        (2, 2),
        (3, 1),
        (4, 4),
        (5, 5),
        (6, 6),
        (7, 7),
        (8, 8),
        (9, 16),
        (10, 12),
        (11, 20),
        (12, 10),
        (13, 22),
        (14, 14),
        (15, 18),
        (16, 9),
        (17, 21),
        (18, 15),
        (19, 19),
        (20, 11),
        (21, 17),
        (22, 13),
        (23, 23),
    ]);
    return rotate(coord, *map.get(&rotation).unwrap());
}

impl BaseDay for Day19 {
    fn parse(&mut self, input: &mut InputReader) {
        let header_rex = regex!(r#"--- (.*?) ---"#);
        let coord_rex = regex!(r#"(-?\d+)\s*,\s*(-?\d+)\s*,\s*(-?\d+)"#);

        let mut cur: Option<Scanner> = None;
        for line in input.lines() {
            let line = line.unwrap();
            if line.is_empty() {
                if cur.is_some() {
                    self.vals.push(cur.unwrap());
                    cur = None;
                }
            } else if let Some(c) = header_rex.captures(&line) {
                cur = Some(Scanner { name: c[1].to_owned(), beacons: Vec::new() });
            } else if let Some(c) = coord_rex.captures(&line) {
                let coord = Coord3d { x: c[1].parse::<i32>().unwrap(), y: c[2].parse::<i32>().unwrap(), z: c[3].parse::<i32>().unwrap() };
                if let Some(ref mut scanner) = cur {
                    scanner.beacons.push(coord);
                } else {
                    panic!("No active scanner for coord: {:?}", coord);
                }
            } else {
                panic!("Bad line: {}", line);
            }
        }
        if cur.is_some() {
            self.vals.push(cur.unwrap());
        }
    }

    fn setup(&mut self) {
        self.all_beacons.extend(self.vals[0].beacons.iter().map(|c| *c));
        let origin = Orientation { translation: Direction3d {dx: 0, dy: 0, dz: 0}, rotation: 0 };
        self.orientations = vec![origin; self.vals.len()];
        let mut done = HashSet::from([0]);
        while done.len() < self.vals.len() {
            let mut did_any = false;
            for (idx, scanner) in self.vals.iter().enumerate() {
                if done.contains(&idx) {
                    continue;
                }
                if let Some((orientation, corrected)) = find_possible_overlap(&self.all_beacons.iter().map(|c| *c).collect(), &scanner.beacons) {
                    println!("Found overlap for {}", scanner.name);
                    self.all_beacons.extend(corrected);
                    self.orientations[idx] = orientation;
                    done.insert(idx);
                    did_any = true;
                }
            }
            if !did_any {
                panic!("Some matches remaining? {}", self.vals.len() - done.len());
            }
        }
    }

    fn pt1(&mut self) -> String {
        return self.all_beacons.len().to_string();
    }

    fn pt2(&mut self) -> String {
        let as_coord = |dir: Direction3d| Coord3d { x: dir.dx, y: dir.dy, z: dir.dz };
        let max_md = self.orientations.iter().cartesian_product(self.orientations.iter()).map(|(d1, d2)| manhattan3d(&as_coord(d1.translation), &as_coord(d2.translation))).max().unwrap();
        return max_md.to_string();
    }
}

fn main() {
    let mut day = Day19 { vals: Vec::new(), orientations: Vec::new(), all_beacons: HashSet::new() };
    run_day(&mut day);
}
