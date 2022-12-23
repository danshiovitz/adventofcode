use std::collections::HashMap;

extern crate common;

use common::framework::{parse_grid_record, parse_lines, run_day, BaseDay, InputReader};
use common::grid::{
    add_direction, add_direction_wrapped, print_grid, turn_left, turn_right, Coord, Direction, Grid,
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

fn walk_grid(grid: &Grid<char>, steps: &Vec<Step>, cube: bool) -> (Coord, Direction) {
    let mut cur_pos = *grid
        .coords
        .keys()
        .filter(|c| c.y == grid.min.y)
        .min()
        .unwrap();
    let mut cur_facing = EAST;
    let warps = make_cube_warp_map(grid);

    let advance = |pos: &Coord, dir: &Direction| -> Option<(Coord, Direction)> {
        let (nxt, nxtdir);
        if cube {
            (nxt, nxtdir) = add_direction_cube(&pos, &dir, &warps);
        } else {
            (nxt, nxtdir) = (add_direction_wrapped(&pos, &dir, grid), *dir);
        }
        if *grid.coords.get(&nxt).unwrap() == '#' {
            println!("Bonk!");
            return None;
        } else {
            println!("Stepped to {:?}, {:?}", nxt, nxtdir);
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

pub fn add_direction_cube(
    start: &Coord,
    dir: &Direction,
    warps: &HashMap<(Coord, Direction), (Coord, Direction)>,
) -> (Coord, Direction) {
    if let Some((new_pos, new_dir)) = warps.get(&(*start, *dir)) {
        return (*new_pos, *new_dir);
    } else {
        return (add_direction(start, dir), *dir);
    }
}

fn make_cube_warp_map(grid: &Grid<char>) -> HashMap<(Coord, Direction), (Coord, Direction)> {
    struct Transform {
        x: i32,
        y: i32,
        // turns are relative to the 0 side
        turns: i32,
    }

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
            Transform { x: 2 * size, y: 0 * size, turns: 0 },
            Transform { x: 0 * size, y: 1 * size, turns: 2 },
            Transform { x: 1 * size, y: 1 * size, turns: 3 },
            Transform { x: 2 * size, y: 1 * size, turns: 0 },
            Transform { x: 2 * size, y: 2 * size, turns: 0 },
            Transform { x: 3 * size, y: 2 * size, turns: 2 },
        ];
    } else if max_x % 3 == 2 && (max_x + 1) / 3 == min_x {
        // Unrolled like
        //    01
        //    2
        //   34
        //   5
        size = top_len / 2;
        panic!("Not currently supported");
    } else {
        panic!(
            "Unexpected top row: len={}, min={}, max={}",
            top_row.len(),
            min_x,
            max_x
        );
    }

    let mut ret = HashMap::new();
    for i in 0..size {
        let r = -1 - i;
        // A
        ret.insert(
            (Coord { x: size * 2 + i, y: size * 0 }, NORTH),
            (Coord { x: size * 1 + r, y: size * 1 }, SOUTH),
        );
        ret.insert(
            (Coord { x: size * 2, y: size * 0 + i }, WEST),
            (Coord { x: size * 1 + i, y: size * 1 }, SOUTH),
        );
        ret.insert(
            (Coord { x: size * 3 - 1, y: size * 0 + i }, EAST),
            (Coord { x: size * 4 - 1, y: size * 3 + r }, WEST),
        );

        // B
        ret.insert(
            (Coord { x: size * 0 + i, y: size * 1 }, NORTH),
            (Coord { x: size * 3 + r, y: size * 0 }, SOUTH),
        );
        ret.insert(
            (Coord { x: size * 0, y: size * 1 + i }, WEST),
            (Coord { x: size * 4 + r, y: size * 3 - 1 }, NORTH),
        );
        ret.insert(
            (Coord { x: size * 0 + i, y: size * 2 - 1 }, SOUTH),
            (Coord { x: size * 3 + r, y: size * 3 - 1 }, NORTH),
        );

        // C
        ret.insert(
            (Coord { x: size * 1 + i, y: size * 1 }, NORTH),
            (Coord { x: size * 2, y: size * 0 + i }, EAST),
        );
        ret.insert(
            (Coord { x: size * 1 + i, y: size * 2 - 1 }, SOUTH),
            (Coord { x: size * 2, y: size * 3 + r }, EAST),
        );

        // D
        ret.insert(
            (Coord { x: size * 3 - 1, y: size * 1 + i }, EAST),
            (Coord { x: size * 4 + r, y: size * 2 }, SOUTH),
        );

        // E
        ret.insert(
            (Coord { x: size * 2, y: size * 2 + i }, WEST),
            (Coord { x: size * 2 + r, y: size * 2 - 1 }, NORTH),
        );
        ret.insert(
            (Coord { x: size * 2 + i, y: size * 3 - 1 }, SOUTH),
            (Coord { x: size * 1 + r, y: size * 2 - 1 }, NORTH),
        );

        // F
        ret.insert(
            (Coord { x: size * 3 + i, y: size * 2 }, NORTH),
            (Coord { x: size * 3 - 1, y: size * 2 + r }, WEST),
        );
        ret.insert(
            (Coord { x: size * 4 - 1, y: size * 2 + i }, EAST),
            (Coord { x: size * 3 - 1, y: size * 1 + r }, WEST),
        );
        ret.insert(
            (Coord { x: size * 3 + i, y: size * 3 - 1 }, SOUTH),
            (Coord { x: size * 0, y: size * 2 + r }, EAST),
        );
    }

    let mut altret = HashMap::new();

    // edges are indexes into the transforms array,
    // ordered NORTH SOUTH EAST WEST,
    // from the unfolded orientation
    let edges: [[usize; 4]; 6] = [
        [1, 3, 5, 2],
        [0, 4, 2, 5],
        [0, 4, 3, 1],
        [0, 4, 5, 2],
        [3, 1, 5, 2],
        [3, 1, 0, 4],
    ];

    for (tf_idx, tf) in transforms.iter().enumerate() {
        for (d_idx, dir) in vec![NORTH, SOUTH, EAST, WEST].iter().enumerate() {
            for i in 0..size {
                let fx = *&[0, i as i32, size - 1][(dir.dx + 1) as usize];
                let fy = *&[0, i as i32, size - 1][(dir.dy + 1) as usize];

                let from_pos = Coord { x: tf.x + fx, y: tf.y + fy };
                let from_dir = *dir;

                let ngh_tf = &transforms[edges[tf_idx][d_idx]];
                let net_turns = ((ngh_tf.turns - tf.turns) + 4) % 4;

                let mut tx = fx;
                let mut ty = fy;
                if *dir == NORTH {
                    ty = size - 1;
                } else if *dir == SOUTH {
                    ty = 0;
                } else if *dir == EAST {
                    tx = 0;
                } else if *dir == WEST {
                    tx = size - 1;
                }

                if from_pos == (Coord { x: 0, y: 4 }) && from_dir == (Direction { dx: -1, dy: 0 }) {
                    println!(
                        "tf turns: {}, ngh turns: {}, turn 0: {:?}, 1: {:?}, 2: {:?}, 3: {:?}",
                        tf.turns,
                        ngh_tf.turns,
                        from_dir,
                        turn_right(&from_dir),
                        turn_right(&turn_right(&from_dir)),
                        turn_right(&turn_right(&turn_right(&from_dir)))
                    );
                }

                let mut to_dir = from_dir;
                for _ in 0..net_turns {
                    to_dir = turn_right(&to_dir);
                    let tmp = ty;
                    ty = tx;
                    tx = size - 1 - tmp;
                }
                let to_pos = Coord { x: ngh_tf.x + tx, y: ngh_tf.y + ty };

                if (tf_idx == 0 && (d_idx == 0 || d_idx == 2 || d_idx == 3))
                    || (tf_idx == 1 && (d_idx == 0 || d_idx == 1 || d_idx == 3))
                    || (tf_idx == 2 && (d_idx == 0 || d_idx == 1))
                    || (tf_idx == 3 && (d_idx == 2))
                    || (tf_idx == 4 && (d_idx == 1 || d_idx == 3))
                    || (tf_idx == 5 && (d_idx == 0 || d_idx == 1 || d_idx == 2))
                {
                    altret.insert((from_pos, from_dir), (to_pos, to_dir));
                }
            }
        }
    }

    if ret != altret {
        {
            let k = (Coord { x: 8, y: 2 }, Direction { dx: -1, dy: 0 });
            let rv = ret.get(&k).unwrap();
            let av = altret.get(&k).unwrap();
            if rv != av {
                panic!("Mismatch on value for k={:?}, {:?}, {:?}", k, rv, av);
            } else {
                panic!("Not a mismatch!");
            }
        }

        for xx in 0..20 {
            let mut rkeys = ret
                .keys()
                .filter(|k| k.0.x == xx)
                .collect::<Vec<&(Coord, Direction)>>();
            rkeys.sort();
            let mut akeys = altret
                .keys()
                .filter(|k| k.0.x == xx)
                .collect::<Vec<&(Coord, Direction)>>();
            akeys.sort();
            if rkeys != akeys {
                println!("Mismatch on keys: {:?} // {:?}", rkeys, akeys);
            }
            for k in rkeys {
                let rv = ret.get(k).unwrap();
                let av = altret.get(k).unwrap();
                if rv != av {
                    panic!(
                        "Mismatch on value for xx={}, k={:?}, {:?}, {:?}",
                        xx, k, rv, av
                    );
                }
            }
        }
        panic!("Mismatch on something else??");
    }
    return ret;
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
        let (pos, dir) = walk_grid(&self.grid, &self.steps, false);
        return compute_password(&pos, &dir).to_string();
    }

    fn pt2(&mut self) -> String {
        let (pos, dir) = walk_grid(&self.grid, &self.steps, true);
        return compute_password(&pos, &dir).to_string();
    }
}

fn main() {
    let mut day = Day22 { grid: Grid::new(), steps: Vec::new() };
    run_day(&mut day);
}
