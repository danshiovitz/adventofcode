use std::collections::{HashMap, HashSet};

#[derive(PartialEq, Eq, PartialOrd, Ord, Copy, Clone, Debug, Hash)]
pub struct Coord {
    pub x: i32,
    pub y: i32,
}

#[derive(PartialEq, Eq, Copy, Clone, Debug, Hash)]
pub struct Direction {
    pub dx: i32,
    pub dy: i32,
}

#[derive(PartialEq, Eq, Copy, Clone, Debug, Hash)]
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

pub fn manhattan(start: &Coord, end: &Coord) -> i32 {
    return (end.x - start.x).abs() + (end.y - start.y).abs();
}
