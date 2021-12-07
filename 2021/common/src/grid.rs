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
