use std::collections::HashMap;

extern crate common;

use common::framework::{parse_grid_record, parse_lines, run_day, BaseDay, InputReader};
use common::grid::{
    add_direction, print_grid, rotate_right, turn_left, turn_right, Coord, Direction, Grid,
};

#[derive(Debug)]
enum Step {
    Forward(i32),
    Left(),
    Right(),
}

struct Day22 {
    grid: Grid<char>,
    steps: Vec<Step>,
}

const NORTH: Direction = Direction { dx: 0, dy: -1 };
const SOUTH: Direction = Direction { dx: 0, dy: 1 };
const EAST: Direction = Direction { dx: 1, dy: 0 };
const WEST: Direction = Direction { dx: -1, dy: 0 };

fn compute_password(coord: &Coord, dir: &Direction) -> i32 {
    println!("Received for password: {:?}, {:?}", coord, dir);

    let dirscore;
    if *dir == NORTH {
        dirscore = 3;
    } else if *dir == SOUTH {
        dirscore = 1;
    } else if *dir == EAST {
        dirscore = 0;
    } else if *dir == WEST {
        dirscore = 2;
    } else {
        panic!("Unexpected direction: {:?}", dir);
    }

    return dirscore + ((coord.x + 1) * 4) + ((coord.y + 1) * 1000);
}

fn walk_grid(
    grid: &Grid<char>,
    steps: &Vec<Step>,
    warps: &HashMap<(Coord, Direction), (Coord, Direction)>,
) -> (Coord, Direction) {
    let mut cur_pos = *grid
        .coords
        .keys()
        .filter(|c| c.y == grid.min.y)
        .min()
        .unwrap();
    let mut cur_facing = EAST;

    let advance = |pos: &Coord, dir: &Direction| -> Option<(Coord, Direction)> {
        let (nxt, nxtdir) = if let Some((new_pos, new_dir)) = warps.get(&(*pos, *dir)) {
            (*new_pos, *new_dir)
        } else {
            (add_direction(pos, dir), *dir)
        };

        if *grid.coords.get(&nxt).unwrap() == '#' {
            // println!("Bonk!");
            return None;
        } else {
            // println!("Stepped to {:?}, {:?}", nxt, nxtdir);
            return Some((nxt, nxtdir));
        }
    };

    for step in steps {
        // println!("pos: {:?}, facing: {:?}, doing: {:?}", cur_pos, cur_facing, step);
        match step {
            Step::Left() => {
                cur_facing = turn_left(&cur_facing);
            }
            Step::Right() => {
                cur_facing = turn_right(&cur_facing);
            }
            Step::Forward(v) => {
                for _ in 0..*v {
                    if let Some((nxt, nxtdir)) = advance(&cur_pos, &cur_facing) {
                        cur_pos = nxt;
                        cur_facing = nxtdir;
                    } else {
                        break;
                    }
                }
            }
        }
    }

    return (cur_pos, cur_facing);
}

#[derive(Debug)]
struct Edge {
    idx: usize,
    turns: i32,
}

#[derive(Debug)]
struct Transform {
    x: i32,
    y: i32,
    edges: [Edge; 4], // NORTH SOUTH EAST WEST
}

fn make_flat_warp_map(grid: &Grid<char>) -> HashMap<(Coord, Direction), (Coord, Direction)> {
    let top_row = grid
        .coords
        .keys()
        .filter(|c| c.y == grid.min.y)
        .collect::<Vec<&Coord>>();
    let top_len = top_row.len() as i32;
    let min_x = top_row.iter().min().unwrap().x;
    let max_x = top_row.iter().max().unwrap().x;
    let size: i32;
    let transforms: Vec<Transform>;

    if max_x % 3 == 2 && min_x == top_len * 2 && max_x == top_len * 3 - 1 {
        // Unrolled like
        //     0
        //   123
        //     45
        size = top_len;
        transforms = vec![
            Transform {
                x: 2 * size,
                y: 0 * size,
                edges: [
                    Edge { idx: 4, turns: 0 },
                    Edge { idx: 3, turns: 0 },
                    Edge { idx: 1, turns: 0 },
                    Edge { idx: 2, turns: 0 },
                ],
            },
            Transform {
                x: 0 * size,
                y: 1 * size,
                edges: [
                    Edge { idx: 1, turns: 0 },
                    Edge { idx: 1, turns: 0 },
                    Edge { idx: 2, turns: 0 },
                    Edge { idx: 3, turns: 0 },
                ],
            },
            Transform {
                x: 1 * size,
                y: 1 * size,
                edges: [
                    Edge { idx: 2, turns: 0 },
                    Edge { idx: 2, turns: 0 },
                    Edge { idx: 3, turns: 0 },
                    Edge { idx: 1, turns: 0 },
                ],
            },
            Transform {
                x: 2 * size,
                y: 1 * size,
                edges: [
                    Edge { idx: 0, turns: 0 },
                    Edge { idx: 4, turns: 0 },
                    Edge { idx: 1, turns: 0 },
                    Edge { idx: 2, turns: 0 },
                ],
            },
            Transform {
                x: 2 * size,
                y: 2 * size,
                edges: [
                    Edge { idx: 3, turns: 0 },
                    Edge { idx: 0, turns: 0 },
                    Edge { idx: 5, turns: 0 },
                    Edge { idx: 5, turns: 0 },
                ],
            },
            Transform {
                x: 3 * size,
                y: 2 * size,
                edges: [
                    Edge { idx: 5, turns: 0 },
                    Edge { idx: 5, turns: 0 },
                    Edge { idx: 4, turns: 0 },
                    Edge { idx: 4, turns: 0 },
                ],
            },
        ];
    } else if max_x % 3 == 2 && (max_x + 1) / 3 == min_x {
        // Unrolled like
        //    01
        //    2
        //   34
        //   5
        size = top_len / 2;
        transforms = vec![
            Transform {
                x: 1 * size,
                y: 0 * size,
                edges: [
                    Edge { idx: 4, turns: 0 },
                    Edge { idx: 2, turns: 0 },
                    Edge { idx: 1, turns: 0 },
                    Edge { idx: 1, turns: 0 },
                ],
            },
            Transform {
                x: 2 * size,
                y: 0 * size,
                edges: [
                    Edge { idx: 1, turns: 0 },
                    Edge { idx: 1, turns: 0 },
                    Edge { idx: 0, turns: 0 },
                    Edge { idx: 0, turns: 0 },
                ],
            },
            Transform {
                x: 1 * size,
                y: 1 * size,
                edges: [
                    Edge { idx: 0, turns: 0 },
                    Edge { idx: 4, turns: 0 },
                    Edge { idx: 2, turns: 0 },
                    Edge { idx: 2, turns: 0 },
                ],
            },
            Transform {
                x: 0 * size,
                y: 2 * size,
                edges: [
                    Edge { idx: 5, turns: 0 },
                    Edge { idx: 5, turns: 0 },
                    Edge { idx: 4, turns: 0 },
                    Edge { idx: 4, turns: 0 },
                ],
            },
            Transform {
                x: 1 * size,
                y: 2 * size,
                edges: [
                    Edge { idx: 2, turns: 0 },
                    Edge { idx: 0, turns: 0 },
                    Edge { idx: 3, turns: 0 },
                    Edge { idx: 3, turns: 0 },
                ],
            },
            Transform {
                x: 0 * size,
                y: 3 * size,
                edges: [
                    Edge { idx: 3, turns: 0 },
                    Edge { idx: 3, turns: 0 },
                    Edge { idx: 5, turns: 0 },
                    Edge { idx: 5, turns: 0 },
                ],
            },
        ];
    } else {
        panic!(
            "Unexpected top row: len={}, min={}, max={}",
            top_row.len(),
            min_x,
            max_x
        );
    }

    return warp_map_helper(size, &transforms);
}

fn make_cube_warp_map(grid: &Grid<char>) -> HashMap<(Coord, Direction), (Coord, Direction)> {
    let top_row = grid
        .coords
        .keys()
        .filter(|c| c.y == grid.min.y)
        .collect::<Vec<&Coord>>();
    let top_len = top_row.len() as i32;
    let min_x = top_row.iter().min().unwrap().x;
    let max_x = top_row.iter().max().unwrap().x;
    let size: i32;
    let transforms: Vec<Transform>;

    if max_x % 3 == 2 && min_x == top_len * 2 && max_x == top_len * 3 - 1 {
        // Unrolled like
        //     0
        //   123
        //     45
        size = top_len;
        transforms = vec![
            Transform {
                x: 2 * size,
                y: 0 * size,
                edges: [
                    Edge { idx: 1, turns: 2 },
                    Edge { idx: 3, turns: 0 },
                    Edge { idx: 5, turns: 2 },
                    Edge { idx: 2, turns: 3 },
                ],
            },
            Transform {
                x: 0 * size,
                y: 1 * size,
                edges: [
                    Edge { idx: 0, turns: 2 },
                    Edge { idx: 4, turns: 2 },
                    Edge { idx: 2, turns: 0 },
                    Edge { idx: 5, turns: 1 },
                ],
            },
            Transform {
                x: 1 * size,
                y: 1 * size,
                edges: [
                    Edge { idx: 0, turns: 1 },
                    Edge { idx: 4, turns: 3 },
                    Edge { idx: 3, turns: 0 },
                    Edge { idx: 1, turns: 0 },
                ],
            },
            Transform {
                x: 2 * size,
                y: 1 * size,
                edges: [
                    Edge { idx: 0, turns: 0 },
                    Edge { idx: 4, turns: 0 },
                    Edge { idx: 5, turns: 1 },
                    Edge { idx: 2, turns: 0 },
                ],
            },
            Transform {
                x: 2 * size,
                y: 2 * size,
                edges: [
                    Edge { idx: 3, turns: 0 },
                    Edge { idx: 1, turns: 2 },
                    Edge { idx: 5, turns: 0 },
                    Edge { idx: 2, turns: 1 },
                ],
            },
            Transform {
                x: 3 * size,
                y: 2 * size,
                edges: [
                    Edge { idx: 3, turns: 3 },
                    Edge { idx: 1, turns: 3 },
                    Edge { idx: 0, turns: 2 },
                    Edge { idx: 4, turns: 0 },
                ],
            },
        ];
    } else if max_x % 3 == 2 && (max_x + 1) / 3 == min_x {
        // Unrolled like
        //    01
        //    2
        //   34
        //   5
        size = top_len / 2;
        transforms = vec![
            Transform {
                x: 1 * size,
                y: 0 * size,
                edges: [
                    Edge { idx: 5, turns: 1 },
                    Edge { idx: 2, turns: 0 },
                    Edge { idx: 1, turns: 0 },
                    Edge { idx: 3, turns: 2 },
                ],
            },
            Transform {
                x: 2 * size,
                y: 0 * size,
                edges: [
                    Edge { idx: 5, turns: 0 },
                    Edge { idx: 2, turns: 1 },
                    Edge { idx: 4, turns: 2 },
                    Edge { idx: 0, turns: 0 },
                ],
            },
            Transform {
                x: 1 * size,
                y: 1 * size,
                edges: [
                    Edge { idx: 0, turns: 0 },
                    Edge { idx: 4, turns: 0 },
                    Edge { idx: 1, turns: 3 },
                    Edge { idx: 3, turns: 3 },
                ],
            },
            Transform {
                x: 0 * size,
                y: 2 * size,
                edges: [
                    Edge { idx: 2, turns: 1 },
                    Edge { idx: 5, turns: 0 },
                    Edge { idx: 4, turns: 0 },
                    Edge { idx: 0, turns: 2 },
                ],
            },
            Transform {
                x: 1 * size,
                y: 2 * size,
                edges: [
                    Edge { idx: 2, turns: 0 },
                    Edge { idx: 5, turns: 1 },
                    Edge { idx: 1, turns: 2 },
                    Edge { idx: 3, turns: 0 },
                ],
            },
            Transform {
                x: 0 * size,
                y: 3 * size,
                edges: [
                    Edge { idx: 3, turns: 0 },
                    Edge { idx: 1, turns: 0 },
                    Edge { idx: 4, turns: 3 },
                    Edge { idx: 0, turns: 3 },
                ],
            },
        ];
    } else {
        panic!(
            "Unexpected top row: len={}, min={}, max={}",
            top_row.len(),
            min_x,
            max_x
        );
    }

    let mut ec: [i32; 6] = [0, 0, 0, 0, 0, 0];
    let mut ttc = 0;
    for tf in &transforms {
        for e in &tf.edges {
            ec[e.idx] += 1;
            ttc += e.turns;
        }
    }
    for i in 0..6 {
        if ec[i] != 4 {
            panic!("Edge count of idx {} is not 4!", i);
        }
        if ttc % 4 != 0 {
            panic!("Total turns count is not divisible by 4, val={}", ttc);
        }
    }

    return warp_map_helper(size, &transforms);
}

fn warp_map_helper(
    size: i32,
    transforms: &Vec<Transform>,
) -> HashMap<(Coord, Direction), (Coord, Direction)> {
    let mut ret = HashMap::new();

    for tf in transforms.iter() {
        for (d_idx, dir) in vec![NORTH, SOUTH, EAST, WEST].iter().enumerate() {
            for i in 0..size {
                let fx = *&[0, i as i32, size - 1][(dir.dx + 1) as usize];
                let fy = *&[0, i as i32, size - 1][(dir.dy + 1) as usize];

                let from_pos = Coord { x: tf.x + fx, y: tf.y + fy };
                let from_dir = *dir;

                let ngh_tf = &transforms[tf.edges[d_idx].idx];
                let turns = tf.edges[d_idx].turns;

                let mut to_vec = default_point_of_entry(&Coord { x: fx, y: fy }, dir, size);
                let mut to_dir = from_dir;
                for _ in 0..turns {
                    to_dir = turn_right(&to_dir);
                    to_vec = rotate_right(&to_vec, size);
                }
                let to_pos = Coord { x: ngh_tf.x + to_vec.x, y: ngh_tf.y + to_vec.y };
                ret.insert((from_pos, from_dir), (to_pos, to_dir));
            }
        }
    }

    return ret;
}

// figure the default point of entry if we are walking into the square
// from the given point and direction (where the given point is relative
// within the source square, not absolute, and the poe returned is also
// relative)
fn default_point_of_entry(from: &Coord, dir: &Direction, size: i32) -> Coord {
    if *dir == NORTH {
        return Coord { x: from.x, y: size - 1 };
    } else if *dir == SOUTH {
        return Coord { x: from.x, y: 0 };
    } else if *dir == EAST {
        return Coord { x: 0, y: from.y };
    } else if *dir == WEST {
        return Coord { x: size - 1, y: from.y };
    } else {
        panic!("Unexpected direction: {:?}", dir);
    }
}

impl BaseDay for Day22 {
    fn parse(&mut self, input: &mut InputReader) {
        self.grid = parse_grid_record(input, true, &mut |c: char, _c: &Coord| {
            if c != ' ' {
                Some(c)
            } else {
                None
            }
        });
        parse_lines(input, &mut |line: String| {
            let mut val = 0;
            for ch in line.chars() {
                if ch.is_digit(10) {
                    val *= 10;
                    val += ch.to_digit(10).unwrap() as i32;
                } else {
                    if val != 0 {
                        self.steps.push(Step::Forward(val));
                        val = 0;
                    }
                    if ch == 'L' {
                        self.steps.push(Step::Left());
                    } else if ch == 'R' {
                        self.steps.push(Step::Right());
                    } else {
                        panic!("Unexpected turn: {}", ch);
                    }
                }
            }
            if val != 0 {
                self.steps.push(Step::Forward(val));
            }
        });
    }

    fn pt1(&mut self) -> String {
        let (pos, dir) = walk_grid(&self.grid, &self.steps, &make_flat_warp_map(&self.grid));
        return compute_password(&pos, &dir).to_string();
    }

    fn pt2(&mut self) -> String {
        let (pos, dir) = walk_grid(&self.grid, &self.steps, &make_cube_warp_map(&self.grid));
        return compute_password(&pos, &dir).to_string();
    }
}

fn main() {
    let mut day = Day22 { grid: Grid::new(), steps: Vec::new() };
    run_day(&mut day);
}
