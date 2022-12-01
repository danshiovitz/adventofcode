#[derive(PartialEq, Eq, PartialOrd, Ord, Copy, Clone, Debug, Hash)]
pub struct Coord3d {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

#[derive(PartialEq, Eq, Copy, Clone, Debug, Hash)]
pub struct Direction3d {
    pub dx: i32,
    pub dy: i32,
    pub dz: i32,
}

pub fn add_direction3d(start: &Coord3d, dir: &Direction3d) -> Coord3d {
    return Coord3d {
        x: start.x + dir.dx,
        y: start.y + dir.dy,
        z: start.z + dir.dz,
    };
}

pub fn sub_direction3d(start: &Coord3d, dir: &Direction3d) -> Coord3d {
    return Coord3d {
        x: start.x - dir.dx,
        y: start.y - dir.dy,
        z: start.z - dir.dz,
    };
}

pub fn find_direction3d(start: &Coord3d, end: &Coord3d) -> Direction3d {
    return Direction3d {
        dx: end.x - start.x,
        dy: end.y - start.y,
        dz: end.z - start.z,
    };
}

pub fn manhattan3d(start: &Coord3d, end: &Coord3d) -> i32 {
    return (end.x - start.x).abs() + (end.y - start.y).abs() + (end.z - start.z).abs();
}
