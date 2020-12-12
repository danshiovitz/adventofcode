#[macro_use] extern crate lazy_static;
#[macro_use] extern crate failure;

use failure::{Error, bail};
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::io::{BufReader,BufRead};
use std::fs::File;

#[derive(PartialEq, Eq, Clone, Debug, Hash)]
struct Instruction {
    op: char,
    val: i64,
}

#[derive(PartialEq, Eq, Copy, Clone, Debug, Hash)]
struct Direction {
    value: i64,
    x: i64,
    y: i64,
    c: char,
}

static NORTH : Direction = Direction { value: 1, x: 0, y: -1, c: '^' };
static SOUTH : Direction = Direction { value: 2, x: 0, y: 1, c: 'v' };
static WEST : Direction = Direction { value: 3, x: -1, y: 0, c: '<' };
static EAST : Direction = Direction { value: 4, x: 1, y: 0, c: '>' };

static DIRECTIONS : [Direction; 4] = [
    NORTH, SOUTH, WEST, EAST
];

fn turn_left(dir: Direction) -> Direction {
    if dir == NORTH { return WEST; }
    if dir == WEST { return SOUTH; }
    if dir == SOUTH { return EAST; }
    if dir == EAST { return NORTH; }
    panic!("Bad left direction: {:?}", dir);
}

fn turn_right(dir: Direction) -> Direction {
    if dir == NORTH { return EAST; }
    if dir == EAST { return SOUTH; }
    if dir == SOUTH { return WEST; }
    if dir == WEST { return NORTH; }
    panic!("Bad right direction: {:?}", dir);
}

#[derive(PartialEq, Eq, Clone, Copy, Debug, Hash)]
struct State {
    dir: Direction,
    pos: (i64, i64),
    waypoint: (i64, i64),
}

fn parse_int(val: &str) -> Result<i64, Error> {
    return Ok(val.parse::<i64>()?);
}

fn parse_line(line: &str) -> Result<Instruction, Error> {
    lazy_static! {
        static ref LINE_RE: Regex = Regex::new(r"^(\w)([+-]?\d+)$").unwrap();
    }
    match LINE_RE.captures(line) {
        Some(caps) => {
            return Ok(Instruction {
                op: caps.get(1).unwrap().as_str().chars().nth(0).unwrap(),
                val: parse_int(caps.get(2).unwrap().as_str())?,
            });
        }
        None => {
            bail!("Bad line: {}", line);
        }
    }
}

fn read_input(file: &str) -> Result<Vec<Instruction>, Error> {
    let f = File::open(file)?;
    let br = BufReader::new(f);
    let parsed : Result<Vec<Instruction>, Error> = br.lines().map(|line| parse_line(&line?)).collect();
    return parsed;
}

fn move_ship1(instructions: &Vec<Instruction>, init_state: State) -> State {
    let mut state = init_state;
    for inst in instructions {
        // println!("Cur state: {:?}, will {:?}", state, inst);
        if inst.op == 'N' {
            state.pos = (state.pos.0, state.pos.1 - inst.val);
        } else if inst.op == 'S' {
            state.pos = (state.pos.0, state.pos.1 + inst.val);
        } else if inst.op == 'E' {
            state.pos = (state.pos.0 + inst.val, state.pos.1);
        } else if inst.op == 'W' {
            state.pos = (state.pos.0 - inst.val, state.pos.1);
        } else if inst.op == 'F' {
            state.pos = (state.pos.0 + state.dir.x * inst.val,
                         state.pos.1 + state.dir.y * inst.val);
        } else if inst.op == 'L' {
            for _ in 0..(inst.val / 90) {
                state.dir = turn_left(state.dir);
            }
        } else if inst.op == 'R' {
            for _ in 0..(inst.val / 90) {
                state.dir = turn_right(state.dir);
            }
        } else {
            panic!("Unknown op: {}", inst.op);
        }
    }
    return state;
}

fn move_ship2(instructions: &Vec<Instruction>, init_state: State) -> State {
    let mut state = init_state;
    for inst in instructions {
        // println!("Cur state: {:?}, will {:?}", state, inst);
        if inst.op == 'N' {
            state.waypoint = (state.waypoint.0, state.waypoint.1 - inst.val);
        } else if inst.op == 'S' {
            state.waypoint = (state.waypoint.0, state.waypoint.1 + inst.val);
        } else if inst.op == 'E' {
            state.waypoint = (state.waypoint.0 + inst.val, state.waypoint.1);
        } else if inst.op == 'W' {
            state.waypoint = (state.waypoint.0 - inst.val, state.waypoint.1);
        } else if inst.op == 'F' {
            state.pos = (state.pos.0 + state.waypoint.0 * inst.val,
                         state.pos.1 + state.waypoint.1 * inst.val);
        } else if inst.op == 'L' {
            for _ in 0..(inst.val / 90) {
                state.waypoint = (state.waypoint.1, -state.waypoint.0);
            }
        } else if inst.op == 'R' {
            for _ in 0..(inst.val / 90) {
                state.waypoint = (-state.waypoint.1, state.waypoint.0);
            }
        } else {
            panic!("Unknown op: {}", inst.op);
        }
    }
    return state;
}

fn main() {
    let args : Vec<String> = std::env::args().collect();
    if args[1] == "1" {
        println!("Doing part 1");
        let instructions = read_input(&args[2]).unwrap();
        let state = move_ship1(&instructions, State { dir: EAST, pos: (0, 0), waypoint: (0, 0) });
        println!("Final state is {:?} = {}", state, state.pos.0.abs() + state.pos.1.abs());
    } else {
        println!("Doing part 2");
        let instructions = read_input(&args[2]).unwrap();
        let state = move_ship2(&instructions, State { dir: EAST, pos: (0, 0), waypoint: (10, -1) });
        println!("Final state is {:?} = {}", state, state.pos.0.abs() + state.pos.1.abs());
    }
}
