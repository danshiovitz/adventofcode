#[macro_use] extern crate lazy_static;

use failure::Error;
use itertools::Itertools;
use regex::Regex;
use std::cell::RefCell;
use std::collections::{BinaryHeap,HashMap,HashSet};
use std::cmp::Ordering;
use std::io::{BufReader,BufRead};
use std::fs::File;

#[derive(PartialEq, Eq, Copy, Clone, Debug, Hash)]
enum TileType {
    WALL,
    EMPTY,
    OXYGEN,
}

static TILE_TYPES : [TileType; 3] = [
    TileType::WALL, TileType::EMPTY, TileType::OXYGEN,
];

#[derive(PartialEq, Eq, Copy, Clone, Debug, Hash)]
struct Direction {
    value: i64,
    x: i64,
    y: i64,
}

static NORTH : Direction = Direction { value: 1, x: 0, y: -1 };
static SOUTH : Direction = Direction { value: 2, x: 0, y: 1 };
static WEST : Direction = Direction { value: 3, x: -1, y: 0 };
static EAST : Direction = Direction { value: 4, x: 1, y: 0 };

static DIRECTIONS : [Direction; 4] = [
    NORTH, SOUTH, WEST, EAST
];

fn parse_int(val: &str) -> Result<i64, Error> {
    return Ok(val.parse::<i64>()?);
}

fn parse_line(line: &str) -> Result<Vec<i64>, Error> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r",\s*").unwrap();
    }
    return RE.split(&line).map(|val| parse_int(&val)).collect();
}

fn read_input(file: &str) -> Result<Vec<i64>, Error> {
    let f = File::open(file)?;
    let br = BufReader::new(f);
    let parsed : Result<Vec<Vec<i64>>, Error> = br.lines().map(|line| parse_line(&line?)).collect();
    return Ok(parsed?.into_iter().flatten().collect());
}

enum ExitType {
    Finish99,
    InputAbort,
    OutputAbort,
}

fn run_program(
    program_init: &Vec<i64>,
    mut input: impl FnMut() -> Option<i64>,
    mut output: impl FnMut(i64, &HashMap<i64, i64>) -> bool,
) -> ExitType {
    let mut program : HashMap<i64, i64> = HashMap::new();
    for i in 0..program_init.len() {
        program.insert(i as i64, program_init[i]);
    }
    return run_program_state(program, input, output);
}

fn run_program_state(
    mut program: HashMap<i64, i64>,
    mut input: impl FnMut() -> Option<i64>,
    mut output: impl FnMut(i64, &HashMap<i64, i64>) -> bool,
) -> ExitType {
    let pc_key = -98;
    let relative_base = -99;

    if !program.contains_key(&pc_key) {
        program.insert(pc_key, 0);
    }
    if !program.contains_key(&relative_base) {
        program.insert(relative_base, 0);
    }

    let read_idx = |i: i64, program: &HashMap<i64, i64>| -> i64 {
        if i < 0 {
            panic!("Index {} outside of program bound ({})", i, program.len());
        }
        match program.get(&i) {
            Some(&v) => { return v; },
            None => { return 0; },
        };
    };

    let write_idx = |i: i64, value: i64, program: &mut HashMap<i64, i64>| {
        if i < 0 {
            panic!("Index {} outside of program bound ({})", i, program.len());
        }
        program.insert(i, value);
    };

    let resolve_idx = |pc: i64, num: i64, modes: i64, program: &HashMap<i64, i64>| -> i64 {
        let i = pc + num;
        let mut flag;
        if num > 0 {
            flag = modes;
            for _ in 0..num-1 {
                flag /= 10;
            }
            flag %= 10;
        } else {
            flag = 1;
        }
        if flag == 0 {
            return read_idx(i, program);
        } else if flag == 1 {
            return i;
        } else if flag == 2 {
            return read_idx(i, program) + program[&relative_base];
        } else {
            panic!("Bad flag at {}: {}", i, flag);
        }
    };

    let get = |pc: i64, num: i64, modes: i64, program: &HashMap<i64, i64>| -> i64 {
        return read_idx(resolve_idx(pc, num, modes, program), program);
    };

    let set = |pc: i64, num: i64, modes: i64, value: i64, program: &mut HashMap<i64, i64>| {
        return write_idx(resolve_idx(pc, num, modes, program), value, program);
    };

    loop {
        let pc = program[&pc_key];
        if pc >= program.len() as i64 || pc < 0 {
            panic!("PC {} outside of program bound ({})", pc, program.len());
        }
        let opcode = get(pc, 0, 0, &program) % 100;
        let modes = get(pc, 0, 0, &program) / 100;
        // println!("Executing at pc={} opcode={}, modes={}", pc, opcode, modes);
        if opcode == 1 {
            let value = get(pc, 1, modes, &program) + get(pc, 2, modes, &program);
            set(pc, 3, modes, value, &mut program);
            *program.get_mut(&pc_key).unwrap() += 4;
        } else if opcode == 2 {
            let value = get(pc, 1, modes, &program) * get(pc, 2, modes, &program);
            set(pc, 3, modes, value, &mut program);
            *program.get_mut(&pc_key).unwrap() += 4;
        } else if opcode == 3 {
            let value = input();
            match value {
                Some(v) => {
                    set(pc, 1, modes, v, &mut program);
                    *program.get_mut(&pc_key).unwrap() += 2;
                },
                None => {
                    return ExitType::InputAbort;
                }
            }
        } else if opcode == 4 {
            let val = get(pc, 1, modes, &program);
            *program.get_mut(&pc_key).unwrap() += 2;
            if output(val, &program) {
                return ExitType::OutputAbort;
            }
        } else if opcode == 5 || opcode == 6 {
            if (get(pc, 1, modes, &program) == 0) == (opcode == 6) {
                *program.get_mut(&pc_key).unwrap() = get(pc, 2, modes, &program);
            } else {
                *program.get_mut(&pc_key).unwrap() += 3;
            }
        } else if opcode == 7 {
            let value = if get(pc, 1, modes, &program) < get(pc, 2, modes, &program) { 1 } else { 0 };
            set(pc, 3, modes, value, &mut program);
            *program.get_mut(&pc_key).unwrap() += 4;
        } else if opcode == 8 {
            let value = if get(pc, 1, modes, &program) == get(pc, 2, modes, &program) { 1 } else { 0 };
            set(pc, 3, modes, value, &mut program);
            *program.get_mut(&pc_key).unwrap() += 4;
        } else if opcode == 9 {
            let value = get(pc, 1, modes, &program);
            *program.get_mut(&relative_base).unwrap() += value;
            *program.get_mut(&pc_key).unwrap() += 2;
        } else if opcode == 99 {
            // println!("Saw 99, exiting");
            return ExitType::Finish99;
        } else {
            panic!("Unknown opcode at {}: {}", pc, get(pc, 0, 0, &program));
        }
    }
}

fn explore_maze2(program: Vec<i64>) -> HashMap<(i64, i64), TileType> {
    let mut dirs = vec![1, 1, 3, 3, 3];
    let input_fn = || -> Option<i64> {
        return Some(dirs.remove(0));
    };

    let output_fn = |i: i64, program: &HashMap<i64, i64>| -> bool {
        println!("Output: {}", i);
        println!("HM: {:?}", program);
        return false;
    };
    run_program(&program, input_fn, output_fn);
    return HashMap::new();
}

struct State {
    distance: i32,
    program: HashMap<i64, i64>,
}

fn explore_maze(program: Vec<i64>) -> (HashMap<(i64, i64), TileType>, i32) {
    let mut grid : HashMap<(i64, i64), TileType> = HashMap::new();
    let mut program_states : HashMap<(i64, i64), State> = HashMap::new();
    let mut positions = Vec::new();

    grid.insert((0, 0), TileType::EMPTY);
    let hash_p = program.iter().enumerate().map(|(i, item)| (i as i64, *item)).collect();
    program_states.insert((0, 0), State { distance: 0, program: hash_p });
    positions.push((0, 0));

    let mut oxy_distance = -1;

    while let Some(pos) = positions.pop() {
        // println!("Considering pos {:?}", pos);
        let state = program_states.get(&pos).unwrap();
        let mut new_states = Vec::new();
        for dir in &DIRECTIONS {
            let next_p = (pos.0 + dir.x, pos.1 + dir.y);
            if grid.get(&next_p).is_none() {
                // println!("No data yet for {:?}", dir);
                match explore_dir(&state.program, dir.clone()) {
                    (TileType::WALL, _) => {
                        grid.insert(next_p, TileType::WALL);
                        // println!("Is a wall: {:?}", next_p);
                    },
                    (TileType::EMPTY, new_program) => {
                        grid.insert(next_p, TileType::EMPTY);
                        positions.push(next_p);
                        new_states.push((next_p, State { distance: state.distance + 1, program: new_program }));
                        // println!("Is empty: {:?}", next_p);
                    },
                    (TileType::OXYGEN, new_program) => {
                        grid.insert(next_p, TileType::OXYGEN);
                        positions.push(next_p);
                        new_states.push((next_p, State { distance: state.distance + 1, program: new_program }));
                        oxy_distance = state.distance + 1;
                        // println!("Is oxy: {:?}", next_p);
                    },
                }
            }
        }
        for (next_p, new_program) in new_states {
            program_states.insert(next_p, new_program);
        }
    }

    return (grid, oxy_distance);
}

fn explore_dir(program: &HashMap<i64, i64>, dir: Direction) -> (TileType, HashMap<i64, i64>) {
    let input_fn = || -> Option<i64> {
        return Some(dir.value);
    };

    let mut ret = (TileType::WALL, HashMap::new());
    let output_fn = |i: i64, program: &HashMap<i64, i64>| -> bool {
        if i == 0 {
            return true;
        } else if i == 1 || i == 2 {
            ret = (if i == 2 { TileType::OXYGEN } else { TileType::EMPTY }, program.clone());
            return true;
        } else {
            panic!("Unexpected output: {}", i);
        }
    };

    match run_program_state(program.clone(), input_fn, output_fn) {
        ExitType::OutputAbort => { return ret; },
        ExitType::InputAbort => { panic!("Unexpected input abort"); },
        ExitType::Finish99 => { panic!("Unexpected finish"); },
    }
}

fn print_grid(grid: &HashMap<(i64, i64), TileType>) {
    let mut min_x = std::i64::MAX;
    let mut max_x = std::i64::MIN;
    let mut min_y = std::i64::MAX;
    let mut max_y = std::i64::MIN;

    for val in grid.keys() {
        let (x, y) = *val;
        if x < min_x {
            min_x = x;
        }
        if x > max_x {
            max_x = x;
        }
        if y < min_y {
            min_y = y;
        }
        if y > max_y {
            max_y = y;
        }
    }

    for y in min_y..=max_y {
        let mut line = String::new();
        line.reserve((max_x - min_x + 1) as usize);
        for x in min_x..=max_x {
            if x == 0 && y == 0 {
                line.push('@');
                continue;
            }
            match grid.get(&(x, y)) {
                Some(TileType::EMPTY) => line.push(' '),
                Some(TileType::WALL) => line.push('#'),
                Some(TileType::OXYGEN) => line.push('O'),
                None => line.push('?'),
            }
        }
        println!("{}", line);
    }
}

fn floodfill_grid(grid: &HashMap<(i64, i64), TileType>, init_pos: (i64, i64)) -> i32 {
    let mut seen = HashMap::new();
    let mut working = vec![(init_pos, 0)];
    let mut max_time = -1;
    while let Some((pos, time)) = working.pop() {
        seen.insert(pos, time);
        if time > max_time {
            max_time = time;
        }
        for dir in &DIRECTIONS {
            let next_p = (pos.0 + dir.x, pos.1 + dir.y);
            if seen.contains_key(&next_p) {
                continue;
            }
            match grid.get(&next_p) {
                Some(TileType::EMPTY) => {
                    working.push((next_p, time + 1));
                },
                Some(TileType::OXYGEN) => {
                    working.push((next_p, time + 1));
                },
                Some(TileType::WALL) => {},
                None => {},
            }
        }
    }
    println!("Seen of 0,0: {}", seen.get(&(0, 0)).unwrap());
    return max_time;
}

fn main() {
    let args : Vec<String> = std::env::args().collect();
    if args[1] == "1" {
        println!("Doing part 1");
        let program = read_input(&args[2]).unwrap();
        let (maze, dist) = explore_maze(program);
        print_grid(&maze);
        println!("Distance: {}", dist);
    } else {
        println!("Doing part 2");
        let program = read_input(&args[2]).unwrap();
        let (maze, dist) = explore_maze(program);
        let oxy_loc = maze.iter().filter(|e| *e.1 == TileType::OXYGEN).map(|(k, v)| k).next().unwrap();
        let ff_time = floodfill_grid(&maze, *oxy_loc);
        println!("Total floodfill time: {}", ff_time);
    }
}
