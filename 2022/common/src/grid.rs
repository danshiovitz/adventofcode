use lazy_regex::regex;
use std::collections::{HashMap, HashSet};

use crate::solver::SolverState;

#[derive(PartialEq, Eq, PartialOrd, Ord, Copy, Clone, Debug, Hash)]
pub struct Coord {
    pub x: i32,
    pub y: i32,
}

impl Coord {
    pub fn parse(val: &str) -> Coord {
        match regex!(r#"(\d+)\s*,\s*(\d+)"#).captures(val) {
            Some(c) => {
                return Coord {
                    x: c[1].parse::<i32>().unwrap(),
                    y: c[2].parse::<i32>().unwrap(),
                }
            }
            None => {
                panic!("Bad value for coord: {}", val);
            }
        }
    }
}

impl SolverState for Coord {}

#[derive(PartialEq, Eq, PartialOrd, Ord, Copy, Clone, Debug, Hash)]
pub struct Direction {
    pub dx: i32,
    pub dy: i32,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Copy, Clone, Debug, Hash)]
pub struct Line {
    pub start: Coord,
    pub end: Coord,
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Grid<T> {
    pub coords: HashMap<Coord, T>,
    pub min: Coord,
    pub max: Coord,
}

impl<T> Grid<T> {
    pub fn new() -> Self {
        Grid {
            coords: HashMap::new(),
            min: Coord { x: 0, y: 0 },
            max: Coord { x: 0, y: 0 },
        }
    }

    pub fn recompute_minmax(&mut self) {
        let mut min_x = i32::MAX;
        let mut min_y = i32::MAX;
        let mut max_x = i32::MIN;
        let mut max_y = i32::MIN;
        for coord in self.coords.keys() {
            if coord.x < min_x {
                min_x = coord.x;
            }
            if coord.y < min_y {
                min_y = coord.y;
            }
            if coord.x > max_x {
                max_x = coord.x;
            }
            if coord.y > max_y {
                max_y = coord.y;
            }
        }
        self.min = Coord { x: min_x, y: min_y };
        self.max = Coord { x: max_x, y: max_y };
    }
}

pub fn four_neighbors(coord: &Coord) -> Vec<Coord> {
    return vec![
        Coord { x: coord.x + 1, y: coord.y },
        Coord { x: coord.x - 1, y: coord.y },
        Coord { x: coord.x, y: coord.y + 1 },
        Coord { x: coord.x, y: coord.y - 1 },
    ];
}

pub fn eight_neighbors(coord: &Coord) -> Vec<Coord> {
    return vec![
        Coord { x: coord.x + 1, y: coord.y },
        Coord { x: coord.x - 1, y: coord.y },
        Coord { x: coord.x, y: coord.y + 1 },
        Coord { x: coord.x, y: coord.y - 1 },
        Coord { x: coord.x + 1, y: coord.y + 1 },
        Coord { x: coord.x + 1, y: coord.y - 1 },
        Coord { x: coord.x - 1, y: coord.y + 1 },
        Coord { x: coord.x - 1, y: coord.y - 1 },
    ];
}

pub fn get_indirect_neighbors<F, T>(
    coord: Coord,
    grid: &Grid<T>,
    get_neighbors: &mut F,
) -> HashSet<Coord>
where
    F: FnMut(&Coord) -> Vec<Coord>,
{
    let mut ret: HashSet<Coord> = HashSet::new();

    let mut working = HashSet::new();
    working.insert(coord);

    while !working.is_empty() {
        let cur = *working.iter().next().unwrap();
        working.remove(&cur);
        ret.insert(cur);

        for ngh in &get_neighbors(&cur) {
            if grid.coords.contains_key(ngh) && !ret.contains(ngh) {
                working.insert(*ngh);
            }
        }
    }

    return ret;
}

pub fn color_grid<F, T>(grid: &Grid<T>, get_neighbors: &mut F) -> Vec<HashSet<Coord>>
where
    F: FnMut(&Coord) -> Vec<Coord>,
{
    let mut unassigned: HashSet<Coord> = grid.coords.keys().map(|c| *c).collect();
    let mut groups: Vec<HashSet<Coord>> = vec![];

    while !unassigned.is_empty() {
        let cur = *unassigned.iter().next().unwrap();
        let group = get_indirect_neighbors(cur, grid, get_neighbors);
        unassigned.retain(|e| !group.contains(e));
        groups.push(group);
    }

    return groups;
}

pub fn print_grid<F, T>(grid: &Grid<T>, render_one: &mut F) -> ()
where
    F: FnMut(&Coord, Option<&T>) -> String,
{
    for y in grid.min.y..=grid.max.y {
        for x in grid.min.x..=grid.max.x {
            let coord = Coord { x: x, y: y };
            print!("{}", render_one(&coord, grid.coords.get(&coord)));
        }
        println!();
    }
}

// This assumes angles are multiples of 45 degrees
pub fn get_unit_direction(start: &Coord, end: &Coord) -> Direction {
    let mx = if start.x < end.x {
        1
    } else if start.x > end.x {
        -1
    } else {
        0
    };
    let my = if start.y < end.y {
        1
    } else if start.y > end.y {
        -1
    } else {
        0
    };

    return Direction { dx: mx, dy: my };
}

pub fn add_direction(start: &Coord, dir: &Direction) -> Coord {
    return Coord { x: start.x + dir.dx, y: start.y + dir.dy };
}

pub fn add_direction_wrapped<T>(start: &Coord, dir: &Direction, grid: &Grid<T>) -> Coord {
    let mut cur = *start;
    loop {
        cur = add_direction(&cur, dir);
        if grid.coords.contains_key(&cur) {
            return cur;
        }

        if cur.x > grid.max.x {
            cur.x = grid.min.x - 1;
        } else if cur.x < grid.min.x {
            cur.x = grid.max.x + 1;
        }

        if cur.y > grid.max.y {
            cur.y = grid.min.y - 1;
        } else if cur.y < grid.min.y {
            cur.y = grid.max.y + 1;
        }
    }
}

pub fn manhattan(start: &Coord, end: &Coord) -> i32 {
    return (end.x - start.x).abs() + (end.y - start.y).abs();
}

pub fn turn_left(dir: &Direction) -> Direction {
    return Direction { dx: dir.dy, dy: -dir.dx };
}

pub fn turn_right(dir: &Direction) -> Direction {
    return Direction { dx: -dir.dy, dy: dir.dx };
}
