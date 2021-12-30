use std::cmp::{max, min};
use std::collections::HashSet;

use lazy_regex::regex;

extern crate common;

use common::framework::{parse_lines, run_day, BaseDay, InputReader};
use common::grid3d::{Coord3d};

#[derive(PartialEq, Eq, Copy, Clone, Debug, Hash)]
struct Cube {
    min: Coord3d,
    max: Coord3d,
    enable: bool,
}

struct Day22 {
    vals: Vec<Cube>,
}

#[allow(dead_code)]
fn init_reactor_dumb(cubes: &Vec<Cube>) -> i64 {
    let mut active = HashSet::new();
    for cube in cubes {
        for x in cube.min.x..=cube.max.x {
            for y in cube.min.y..=cube.max.y {
                for z in cube.min.z..=cube.max.z {
                    if cube.enable {
                        active.insert((x, y, z));
                    } else {
                        active.remove(&(x, y, z));
                    }
                }
            }
        }
    }
    return active.len() as i64;
}

fn filter_cubes(cubes: &Vec<Cube>, filter_val: i32) -> Vec<Cube> {
    let overall = Cube {
        min: Coord3d { x: -filter_val, y: -filter_val, z: -filter_val },
        max: Coord3d { x: filter_val, y: filter_val, z: filter_val },
        enable: false,
    };
    return cubes.iter().filter_map(|c| get_overlap(&overall, c)).collect();
}

fn init_reactor(cubes: &Vec<Cube>, verbose: bool) -> i64 {
    let mut actives = HashSet::new(); // non-overlapping cubes of turned-on stuff
    let mut working = cubes.clone();

    while !working.is_empty() {
        if verbose {
            println!("Working size={}, active size={}", working.len(), actives.len());
        }
        let cur = working.remove(0);
        let overlap = actives.iter().filter_map(|a| get_overlap(a, &cur).map(|o| (*a, o))).take(1).next();
        if overlap.is_none() {
            if cur.enable {
                actives.insert(cur);
            }
            continue;
        }
        let (active, overlap) = overlap.unwrap();

        let shatter_cur = shatter(&cur, &overlap);
        let shatter_active = shatter(&active, &overlap);

        if verbose {
            println!("Found overlap between {:?} (shattered into {}) and {:?} (shattered into {})", &cur,
            shatter_cur.len(), active, shatter_active.len());
        }

        actives.remove(&active);

        // Decide about the overlap area
        if overlap.enable {
            actives.insert(overlap);
        }

        // Keep remaining bits of active, if any
        actives.extend(shatter_active);

        // Stick remaining bits of cur at the start of the working list
        // to be processed next - they shouldn't overlap with this item
        // so we shouldn't get into a loop
        for s in shatter_cur {
            working.insert(0, s);
        }
    }

    if verbose {
        println!("Final actives:");
        for active in &actives {
            println!("{} - {:?}", cube_volume(active), active);
        }
    }
    return actives.iter().map(|c| cube_volume(c)).sum();
}

fn cube_volume(cube: &Cube) -> i64 {
    return (cube.max.x - cube.min.x + 1) as i64 * (cube.max.y - cube.min.y + 1) as i64 * (cube.max.z - cube.min.z + 1) as i64;
}

fn get_overlap(first: &Cube, second: &Cube) -> Option<Cube> {
    let omin = Coord3d {
        x: max(first.min.x, second.min.x),
        y: max(first.min.y, second.min.y),
        z: max(first.min.z, second.min.z),
    };
    let omax = Coord3d {
        x: min(first.max.x, second.max.x),
        y: min(first.max.y, second.max.y),
        z: min(first.max.z, second.max.z),
    };
    if omin.x > omax.x || omin.y > omax.y || omin.z > omax.z {
        return None;
    }
    return Some(Cube { min: omin, max: omax, enable: second.enable });
}

// break up volume around overlap, not including the overlap
// area itself (we can assume overlap is entirely within volume)
fn shatter(volume: &Cube, overlap: &Cube) -> Vec<Cube> {
    let mut ret = Vec::new();
    // top slice
    if volume.max.z > overlap.max.z {
        ret.push(Cube {
            min: Coord3d { x: volume.min.x, y: volume.min.y, z: overlap.max.z + 1 },
            max: Coord3d { x: volume.max.x, y: volume.max.y, z: volume.max.z },
            enable: volume.enable,
        });
    }
    // bottom slice
    if volume.min.z < overlap.min.z {
        ret.push(Cube {
            min: Coord3d { x: volume.min.x, y: volume.min.y, z: volume.min.z },
            max: Coord3d { x: volume.max.x, y: volume.max.y, z: overlap.min.z - 1 },
            enable: volume.enable,
        });
    }

    // up to four slices around overlap (some may not be present if overlap is at edge)
    //   .......-y.......
    //   -x0y  olap  +x0y
    //   .......+y.......

    // -y
    if volume.min.y < overlap.min.y {
        ret.push(Cube {
            min: Coord3d { x: volume.min.x, y: volume.min.y, z: overlap.min.z },
            max: Coord3d { x: volume.max.x, y: overlap.min.y - 1, z: overlap.max.z },
            enable: volume.enable,
        });
    }

    // +x, 0y
    if volume.max.x > overlap.max.x {
        ret.push(Cube {
            min: Coord3d { x: overlap.max.x + 1, y: overlap.min.y, z: overlap.min.z },
            max: Coord3d { x: volume.max.x, y: overlap.max.y, z: overlap.max.z },
            enable: volume.enable,
        });
    }

    // -x, 0y
    if volume.min.x < overlap.min.x {
        ret.push(Cube {
            min: Coord3d { x: volume.min.x, y: overlap.min.y, z: overlap.min.z },
            max: Coord3d { x: overlap.min.x - 1, y: overlap.max.y, z: overlap.max.z },
            enable: volume.enable,
        });
    }

    // +y
    if volume.max.y > overlap.max.y {
        ret.push(Cube {
            min: Coord3d { x: volume.min.x, y: overlap.max.y + 1, z: overlap.min.z },
            max: Coord3d { x: volume.max.x, y: volume.max.y, z: overlap.max.z },
            enable: volume.enable,
        });
    }

    return ret;
}

impl BaseDay for Day22 {
    fn parse(&mut self, input: &mut InputReader) {
        fn parse_line(line: String) -> Cube {
            let rex = regex!(r#"(on|off) x=(-?\d+)\.\.(-?\d+),\s*y=(-?\d+)\.\.(-?\d+),\s*z=(-?\d+)\.\.(-?\d+)"#);
            match rex.captures(&line) {
                Some(c) => {
                    return Cube {
                        enable: &c[1] == "on",
                        min: Coord3d {
                            x: c[2].parse::<i32>().unwrap(),
                            y: c[4].parse::<i32>().unwrap(),
                            z: c[6].parse::<i32>().unwrap(),
                        },
                        max: Coord3d {
                            x: c[3].parse::<i32>().unwrap(),
                            y: c[5].parse::<i32>().unwrap(),
                            z: c[7].parse::<i32>().unwrap(),
                        },
                    };
                }
                None => panic!("Bad line: {}", &line),
            };
        }
        self.vals = parse_lines(input, &mut parse_line);
    }

    fn pt1(&mut self) -> String {
        // let correct = init_reactor_dumb(&filter_cubes(&self.vals, 50)).to_string();
        let ret = init_reactor(&filter_cubes(&self.vals, 50), false).to_string();
        return ret.to_string();
    }

    fn pt2(&mut self) -> String {
        let ret = init_reactor(&self.vals, false).to_string();
        return ret.to_string();
    }
}

fn main() {
    let mut day = Day22 { vals: Vec::new() };
    run_day(&mut day);
}
