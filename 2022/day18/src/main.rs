use std::collections::HashSet;

extern crate common;

use common::framework::{parse_lines, run_day, BaseDay, InputReader};
use common::grid3d::{add_direction3d, Coord3d, Direction3d};

struct Day18 {
    vals: HashSet<Coord3d>,
}

fn count_free_sides(cubes: &HashSet<Coord3d>, dirs: &Vec<Direction3d>) -> i32 {
    return cubes
        .iter()
        .map(|c| {
            dirs.iter()
                .filter(|d| !cubes.contains(&add_direction3d(&c, d)))
                .count() as i32
        })
        .sum();
}

fn compute_min_max(cubes: &HashSet<Coord3d>) -> (Coord3d, Coord3d) {
    let mut min_x = i32::MAX;
    let mut min_y = i32::MAX;
    let mut min_z = i32::MAX;
    let mut max_x = i32::MIN;
    let mut max_y = i32::MIN;
    let mut max_z = i32::MIN;
    for coord in cubes.iter() {
        if coord.x < min_x {
            min_x = coord.x;
        }
        if coord.y < min_y {
            min_y = coord.y;
        }
        if coord.z < min_z {
            min_z = coord.z;
        }
        if coord.x > max_x {
            max_x = coord.x;
        }
        if coord.y > max_y {
            max_y = coord.y;
        }
        if coord.z > max_z {
            max_z = coord.z;
        }
    }
    return (
        Coord3d { x: min_x, y: min_y, z: min_z },
        Coord3d { x: max_x, y: max_y, z: max_z },
    );
}

fn compute_exterior_cubes(cubes: &HashSet<Coord3d>, dirs: &Vec<Direction3d>) -> HashSet<Coord3d> {
    let (min, max) = compute_min_max(cubes);
    let mut visited = HashSet::new();
    let mut working = Vec::new();
    for x in (min.x - 1)..=(max.x + 1) {
        for y in (min.y - 1)..=(max.y + 1) {
            working.push(Coord3d { x: x, y: y, z: min.z - 1 });
            working.push(Coord3d { x: x, y: y, z: max.z + 1 });
        }
    }
    for x in (min.x - 1)..=(max.x + 1) {
        for z in (min.z - 1)..=(max.z + 1) {
            working.push(Coord3d { x: x, y: min.y - 1, z: z });
            working.push(Coord3d { x: x, y: max.y + 1, z: z });
        }
    }
    for y in (min.y - 1)..=(max.y + 1) {
        for z in (min.z - 1)..=(max.z + 1) {
            working.push(Coord3d { x: min.x - 1, y: y, z: z });
            working.push(Coord3d { x: max.x + 1, y: y, z: z });
        }
    }

    while working.len() > 0 {
        let cur = working.remove(0);
        if visited.contains(&cur) {
            continue;
        }
        visited.insert(cur);

        for d in dirs {
            let ngh = add_direction3d(&cur, d);
            if visited.contains(&ngh)
                || cubes.contains(&ngh)
                || ngh.x < min.x - 1
                || ngh.x > max.x + 1
                || ngh.y < min.y - 1
                || ngh.y > max.y + 1
                || ngh.z < min.z - 1
                || ngh.z > max.z + 1
            {
                continue;
            }
            working.push(ngh);
        }
    }
    return visited;
}

fn count_exterior_free_sides(cubes: &HashSet<Coord3d>, dirs: &Vec<Direction3d>) -> i32 {
    let exterior_cubes = compute_exterior_cubes(cubes, dirs);

    return cubes
        .iter()
        .map(|c| {
            dirs.iter()
                .filter(|d| exterior_cubes.contains(&add_direction3d(&c, d)))
                .count() as i32
        })
        .sum();
}

fn dirs() -> Vec<Direction3d> {
    return vec![
        Direction3d { dx: 1, dy: 0, dz: 0 },
        Direction3d { dx: -1, dy: 0, dz: 0 },
        Direction3d { dx: 0, dy: 1, dz: 0 },
        Direction3d { dx: 0, dy: -1, dz: 0 },
        Direction3d { dx: 0, dy: 0, dz: 1 },
        Direction3d { dx: 0, dy: 0, dz: -1 },
    ];
}

impl BaseDay for Day18 {
    fn parse(&mut self, input: &mut InputReader) {
        self.vals.extend(parse_lines(input, &mut |line: String| {
            Coord3d::parse(&line)
        }));
    }

    fn pt1(&mut self) -> String {
        let cnt = count_free_sides(&self.vals, &dirs());
        return cnt.to_string();
    }

    fn pt2(&mut self) -> String {
        let cnt = count_exterior_free_sides(&self.vals, &dirs());
        return cnt.to_string();
    }
}

fn main() {
    let mut day = Day18 { vals: HashSet::new() };
    run_day(&mut day);
}
