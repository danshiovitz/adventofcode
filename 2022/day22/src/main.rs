use std::collections::{HashMap, HashSet};

extern crate common;

use common::framework::{parse_grid_record, parse_lines, run_day, BaseDay, InputReader};
use common::grid::{
    add_direction, rotate_right, turn_left, turn_right, Coord, Direction, Grid,
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

fn identify_squares(grid: &Grid<char>) -> (HashSet<Coord>, i32) {
    let size = f64::sqrt(grid.coords.len() as f64 / 6.0) as i32;
    // and double-check:
    if size * size * 6 != grid.coords.len() as i32 || (grid.max.x - grid.min.x + 1) % size != 0 || (grid.max.y - grid.min.y + 1) % size != 0 {
        panic!("Size {} doesn't seem right?", size);
    }

    let mut squares = HashSet::new();
    let mut x = 0;
    loop {
        let cx = grid.min.x + x * size;
        if cx > grid.max.x {
            break;
        }
        let mut y = 0;
        loop {
            let cy = grid.min.y + y * size;
            if cy > grid.max.y {
                break;
            }
            if grid.coords.contains_key(&Coord { x: cx, y: cy }) {
                squares.insert(Coord { x: x, y: y });
            }
            y += 1;
        }
        x += 1;
    }

    return (squares, size);
}

#[derive(Debug, Eq, PartialEq)]
struct Edge {
    idx: usize,
    turns: i32,
}

#[derive(Debug, Eq, PartialEq)]
struct Transform {
    x: i32,
    y: i32,
    edges: [Edge; 4], // NORTH SOUTH EAST WEST
}

fn make_flat_warp_map(grid: &Grid<char>) -> HashMap<(Coord, Direction), (Coord, Direction)> {
    let (squares, size) = identify_squares(grid);

    let mut sorted = squares.iter().map(|c| *c).collect::<Vec<Coord>>();
    // Make the ids match the samples I've done by hand:
    sorted.sort_by_key(|c| (c.y, c.x));
    let idxs = sorted.iter().enumerate().map(|(idx, c)| (*c, idx)).collect::<HashMap<Coord, usize>>();

    let mut transforms = Vec::new();
    for sq in &sorted {
        let nxt_north = add_direction(sq, &NORTH);
        let n_idx = *idxs.get(
            if squares.contains(&nxt_north) {
                &nxt_north
            } else {
                squares.iter().filter(|c| c.x == sq.x).max().unwrap()
            }
        ).unwrap();

        let nxt_south = add_direction(sq, &SOUTH);
        let s_idx = *idxs.get(
            if squares.contains(&nxt_south) {
                &nxt_south
            } else {
                squares.iter().filter(|c| c.x == sq.x).min().unwrap()
            }
        ).unwrap();

        let nxt_east = add_direction(sq, &EAST);
        let e_idx = *idxs.get(
            if squares.contains(&nxt_east) {
                &nxt_east
            } else {
                squares.iter().filter(|c| c.y == sq.y).min().unwrap()
            }
        ).unwrap();

        let nxt_west = add_direction(sq, &WEST);
        let w_idx = *idxs.get(
            if squares.contains(&nxt_west) {
                &nxt_west
            } else {
                squares.iter().filter(|c| c.y == sq.y).max().unwrap()
            }
        ).unwrap();

        // this is ok because we're going through the sorted vec
        // in order to match the idxs
        transforms.push(Transform {
            x: sq.x * size,
            y: sq.y * size,
            edges: [
                Edge { idx: n_idx, turns: 0 },
                Edge { idx: s_idx, turns: 0 },
                Edge { idx: e_idx, turns: 0 },
                Edge { idx: w_idx, turns: 0 },
            ]
        });
    }

    return warp_map_helper(size, &transforms);
}

fn make_cube_warp_map(grid: &Grid<char>) -> HashMap<(Coord, Direction), (Coord, Direction)> {
    // these are the neighboring side ids of a side looking at that side,
    // arranged north east south west (the actual rotation gets adjusted later)
    fn get_neighbors(side: usize) -> Vec<usize> {
        if side == 0 {
            return vec![2, 3, 4, 5];
        } else if side == 1 {
            return vec![4, 3, 2, 5];
        } else if side == 2 {
            return vec![0, 5, 1, 3];
        } else if side == 3 {
            return vec![0, 2, 1, 4];
        } else if side == 4 {
            return vec![0, 3, 1, 5];
        } else if side == 5 {
            return vec![0, 4, 1, 2];
        } else {
            panic!("Bad side: {}", side);
        }
    }

    // these are the expected turns when going to a neighboring side, for a side
    // looking at that side in default rotation, arranged north east south west
    // (the actual rotation gets adjusted later)
    fn get_expected_turns(side: usize) -> Vec<i32> {
        if side == 0 {
            return vec![2, 1, 0, 3];
        } else if side == 1 {
            return vec![0, 3, 2, 1];
        } else if side == 2 {
            return vec![2, 0, 2, 0];
        } else if side == 3 {
            return vec![3, 0, 1, 0];
        } else if side == 4 {
            return vec![0, 0, 0, 0];
        } else if side == 5 {
            return vec![1, 0, 3, 0];
        } else {
            panic!("Bad side: {}", side);
        }
    }

    let (squares, size) = identify_squares(grid);

    let mut sorted = squares.iter().map(|c| *c).collect::<Vec<Coord>>();
    // Make the ids match the samples I've done by hand:
    sorted.sort_by_key(|c| (c.y, c.x));
    let idxs = sorted.iter().enumerate().map(|(idx, c)| (*c, idx)).collect::<HashMap<Coord, usize>>();
    let start = &sorted[0];

    // this array maps side value to (Coord, idx, turns)
    // side values: 0 = up, 1 = down, 2 = front, 3 = right, 4 = back, 5 = left
    let mut sides: [(Coord, usize, i32); 6] = [
        (Coord { x: 99, y: 99 }, 0, 0),
        (Coord { x: 99, y: 99 }, 0, 0),
        (Coord { x: 99, y: 99 }, 0, 0),
        (Coord { x: 99, y: 99 }, 0, 0),
        (Coord { x: 99, y: 99 }, 0, 0),
        (Coord { x: 99, y: 99 }, 0, 0),
    ];
    let mut idx_to_side: [usize; 6] = [usize::MAX, usize::MAX, usize::MAX, usize::MAX, usize::MAX, usize::MAX];

    // first we populate the sides array with the square idx and global rotation
    // for each of the six sides
    let mut seen = HashSet::new();
    let mut working = Vec::new();
    working.push((*start, 0, 0));

    while working.len() > 0 {
        let (cur, side, turns) = working.remove(0);
        if seen.contains(&cur) {
            continue;
        }
        seen.insert(cur);

        let mut nghs = get_neighbors(side);
        // rotate this list of neighbors to match how this side is turned
        nghs.rotate_left(turns as usize);

        let idx = *idxs.get(&cur).unwrap();
        sides[side] = (cur, idx, turns);
        idx_to_side[idx] = side;
        // println!("recorded idx {} as side {} with turns {}", idx, side, turns);
        let dirs = [NORTH, EAST, SOUTH, WEST];
        for i in 0..4 {
            let nxt = add_direction(&cur, &dirs[i]);
            if !squares.contains(&nxt) || seen.contains(&nxt) {
                continue;
            }

            let nxt_side = nghs[i];
            let mut nxt_turns = 0;
            // if we're going, eg, east to reach this square,
            // they should be turned such that they go west to reach us
            let e = (i + 2) % 4;
            let mut nxt_nghs = get_neighbors(nxt_side);
            for _ in 0..4 {
                if nxt_nghs[e] == side {
                    break;
                }
                nxt_nghs.rotate_left(1);
                nxt_turns += 1;
            }
            if nxt_turns == 4 {
                panic!("Couldn't find reverse side");
            }
            working.push((nxt, nxt_side, nxt_turns));
        }
    }

    // for i in 0..6 {
    //     println!("sides {}: coord {:?}, idx {}, turns {}", i, sides[i].0, sides[i].1, sides[i].2);
    // }

    // now we can generate the transform objects by comparing the expected
    // rotation for the direction to the actual rotation (eg, we expect that
    // going north to have a rotation of 3 but that side's global rotation
    // is 2, so the turns for that edge is 3 - 2 = 1)
    let make_transform = |idx: usize| -> Transform {
        let side = idx_to_side[idx];
        let (cur, _idx, turns) = &sides[side];
        let mut nghs = get_neighbors(side);
        let mut expected_turns = get_expected_turns(side);
        // rotate this list of neighbors to match how this side is turned
        nghs.rotate_left(*turns as usize);
        expected_turns.rotate_left(*turns as usize);

        Transform {
            x: cur.x * size,
            y: cur.y * size,
            edges: [
                Edge { idx: sides[nghs[0]].1, turns: (expected_turns[0] + turns - sides[nghs[0]].2 + 8) % 4 },
                Edge { idx: sides[nghs[2]].1, turns: (expected_turns[2] + turns - sides[nghs[2]].2 + 8) % 4 },
                Edge { idx: sides[nghs[1]].1, turns: (expected_turns[1] + turns - sides[nghs[1]].2 + 8) % 4 },
                Edge { idx: sides[nghs[3]].1, turns: (expected_turns[3] + turns - sides[nghs[3]].2 + 8) % 4 },
            ]
        }
    };

    let transforms = (0..6).map(|idx| make_transform(idx)).collect::<Vec<Transform>>();
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
