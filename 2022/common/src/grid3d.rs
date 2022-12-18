use lazy_regex::regex;

#[derive(PartialEq, Eq, PartialOrd, Ord, Copy, Clone, Debug, Hash)]
pub struct Coord3d {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl Coord3d {
    pub fn parse(val: &str) -> Coord3d {
        match regex!(r#"(\d+)\s*,\s*(\d+),\s*(\d+)"#).captures(val) {
            Some(c) => {
                return Coord3d {
                    x: c[1].parse::<i32>().unwrap(),
                    y: c[2].parse::<i32>().unwrap(),
                    z: c[3].parse::<i32>().unwrap(),
                }
            }
            None => {
                panic!("Bad value for coord3d: {}", val);
            }
        }
    }
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
