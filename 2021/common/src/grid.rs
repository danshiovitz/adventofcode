use std::collections::HashMap;

#[derive(PartialEq, Eq, Copy, Clone, Debug, Hash)]
pub struct Coord {
    pub x: i32,
    pub y: i32,
}

pub struct Grid<T> {
    pub coords: HashMap<Coord, T>,
    pub min: Coord,
    pub max: Coord,
}

impl<T> Grid<T> {
    pub fn new() -> Self {
        Grid {
            coords: HashMap::new(),
            min: Coord {x: 0, y: 0},
            max: Coord {x: 0, y: 0},
        }
    }
}

pub fn four_neighbors(coord: &Coord) -> Vec<Coord> {
    return vec![
        Coord { x: coord.x + 1, y: coord.y },
        Coord { x: coord.x - 1, y: coord.y },
        Coord { x: coord.x, y: coord.y + 1 },
        Coord { x: coord.x, y: coord.y - 1},
    ];
}
