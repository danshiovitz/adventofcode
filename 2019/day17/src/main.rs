#[macro_use] extern crate lazy_static;

use failure::Error;
use itertools::Itertools;
use regex::Regex;
use std::cell::RefCell;
use std::collections::{BinaryHeap,HashMap,HashSet};
use std::cmp::Ordering;
use std::io::{BufReader,BufRead};
use std::fs::File;

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
}

fn run_program(program_init: &Vec<i64>, mut input: impl FnMut() -> Option<i64>, mut output: impl FnMut(i64)) -> ExitType {
    let relative_base = -99;

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

    let mut program : HashMap<i64, i64> = HashMap::new();
    for i in 0..program_init.len() {
        program.insert(i as i64, program_init[i]);
    }
    program.insert(relative_base, 0);

    let mut pc : i64 = 0;
    loop {
        if pc >= program.len() as i64 || pc < 0 {
            panic!("PC {} outside of program bound ({})", pc, program.len());
        }
        let opcode = get(pc, 0, 0, &program) % 100;
        let modes = get(pc, 0, 0, &program) / 100;
        // println!("Executing at pc={} opcode={}, modes={}", pc, opcode, modes);
        if opcode == 1 {
            let value = get(pc, 1, modes, &program) + get(pc, 2, modes, &program);
            set(pc, 3, modes, value, &mut program);
            pc += 4;
        } else if opcode == 2 {
            let value = get(pc, 1, modes, &program) * get(pc, 2, modes, &program);
            set(pc, 3, modes, value, &mut program);
            pc += 4;
        } else if opcode == 3 {
            let value = input();
            match value {
                Some(v) => {
                    set(pc, 1, modes, v, &mut program);
                    pc += 2;
                },
                None => {
                    return ExitType::InputAbort;
                }
            }
        } else if opcode == 4 {
            output(get(pc, 1, modes, &program));
            pc += 2;
        } else if opcode == 5 || opcode == 6 {
            if (get(pc, 1, modes, &program) == 0) == (opcode == 6) {
                pc = get(pc, 2, modes, &program);
            } else {
                pc += 3;
            }
        } else if opcode == 7 {
            let value = if get(pc, 1, modes, &program) < get(pc, 2, modes, &program) { 1 } else { 0 };
            set(pc, 3, modes, value, &mut program);
            pc += 4;
        } else if opcode == 8 {
            let value = if get(pc, 1, modes, &program) == get(pc, 2, modes, &program) { 1 } else { 0 };
            set(pc, 3, modes, value, &mut program);
            pc += 4;
        } else if opcode == 9 {
            let value = get(pc, 1, modes, &program);
            *program.get_mut(&relative_base).unwrap() += value;
            pc += 2;
        } else if opcode == 99 {
            // println!("Saw 99, exiting");
            return ExitType::Finish99;
        } else {
            panic!("Unknown opcode at {}: {}", pc, get(pc, 0, 0, &program));
        }
    }
}

#[derive(PartialEq, Eq, Copy, Clone, Debug, Hash)]
enum TileType {
    EMPTY,
    SCAFFOLD,
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

struct Robot {
    x: i64,
    y: i64,
    dir: Option<Direction>,
}

fn view_cameras(program: &Vec<i64>) -> (Robot, HashMap<(i64, i64), TileType>) {
    let robot_ref : RefCell<Robot> = RefCell::new(Robot { x: 0, y: 0, dir: None });
    let grid_ref : RefCell<HashMap<(i64, i64), TileType>> = RefCell::new(HashMap::new());

    let input_fn = || -> Option<i64> {
        panic!("Didn't expect input!");
    };

    let dir_map : HashMap<char, Direction> = DIRECTIONS.iter().map(|d| (d.c, *d)).collect();
    let mut grid_x = 0;
    let mut grid_y = 0;
    let output_fn = |i: i64| {
        let mut robot = robot_ref.borrow_mut();
        let mut grid = grid_ref.borrow_mut();

        let c = std::char::from_u32(i as u32).unwrap();
        if c == '\n' {
            grid_y += 1;
            grid_x = 0;
        } else {
            if c == '.' {
                grid.insert((grid_x, grid_y), TileType::EMPTY);
            } else if c == '#' || dir_map.contains_key(&c) || c == 'X' {
                grid.insert((grid_x, grid_y), TileType::SCAFFOLD);
                if c != '#' {
                    robot.x = grid_x;
                    robot.y = grid_y;
                    robot.dir = dir_map.get(&c).map(|d| *d);
                }
            } else {
                panic!("Unexpected char: {}", c);
            }

            grid_x += 1;
        }
    };

    run_program(program, input_fn, output_fn);
    return (robot_ref.into_inner(), grid_ref.into_inner());
}

fn print_grid(grid: &HashMap<(i64, i64), TileType>, robot: &Robot) {
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
            if x == robot.x && y == robot.y {
                match robot.dir {
                    Some(d) => line.push(d.c),
                    None => line.push('X'),
                }
                continue;
            }
            match grid.get(&(x, y)) {
                Some(TileType::EMPTY) => line.push('.'),
                Some(TileType::SCAFFOLD) => line.push('#'),
                None => line.push('?'),
            }
        }
        println!("{}", line);
    }
}

fn sget(x: i64, y: i64, grid: &HashMap<(i64, i64), TileType>) -> TileType {
    return *grid.get(&(x, y)).unwrap_or(&TileType::EMPTY);
}

fn can_move(x: i64, y: i64, dir: Direction, grid: &HashMap<(i64, i64), TileType>) -> bool {
    return sget(x + dir.x, y + dir.y, grid) == TileType::SCAFFOLD;
}

fn sum_alignments(grid: &HashMap<(i64, i64), TileType>) -> i64 {
    fn alignment_score(x: i64, y: i64, grid: &HashMap<(i64, i64), TileType>) -> Option<i64> {
        if sget(x, y, grid) != TileType::SCAFFOLD {
            return None;
        }
        for dir in DIRECTIONS.iter() {
            if can_move(x, y, *dir, grid) {
                return None;
            }
        }
        println!("Found alignment at {},{}", x, y);
        return Some(x * y);
    }
    return grid.keys().filter_map(|(x,y)| alignment_score(*x, *y, &grid)).sum();
}

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

fn calc_path(grid: &HashMap<(i64, i64), TileType>, init_robot: &Robot) -> Vec<String> {
    let mut cur_x = init_robot.x;
    let mut cur_y = init_robot.y;
    let mut cur_dir = init_robot.dir.unwrap();
    let mut moves = Vec::new();
    loop {
        let mut d = 0;
        while can_move(cur_x, cur_y, cur_dir, grid) {
            d += 1;
            cur_x += cur_dir.x;
            cur_y += cur_dir.y;
        }
        if d > 0 {
            moves.push(d.to_string());
        }
        let left_dir = turn_left(cur_dir);
        let right_dir = turn_right(cur_dir);
        let can_left = can_move(cur_x, cur_y, left_dir, grid);
        let can_right = can_move(cur_x, cur_y, right_dir, grid);
        if !can_left && !can_right {
            // println!("Finished at {},{}", cur_x, cur_y);
            // let path = moves.into_iter().format(",").to_string();
            // println!("Moves ({}): {}", path.len(), path);
            return moves;
        } else if can_left && can_right {
            panic!("There's a t junction at {},{}?", cur_x, cur_y);
        } else if can_left {
            moves.push("L".to_string());
            cur_dir = left_dir;
        } else if can_right {
            moves.push("R".to_string());
            cur_dir = right_dir;
        } else {
            panic!("Weird.");
        }
    }
}

fn compress_path<'a>(path: Vec<String>, func_names: Vec<&'a str>) -> (Vec<&'a str>, Vec<(&'a str, Vec<String>)>) {
    // println!("Considering: {}", path.iter().format(","));
    // assume that path is made up of a series of func_names.len() funcs
    // in arbitrary order, where each func is called at least twice
    let longest_match_at = |idx1: usize, idx2: usize, colors: &Vec<Option<&'a str>>| -> usize {
        let max_sz = vec![idx2 - idx1, path.len() - idx2, 10].into_iter().min().unwrap();
        for i in 0..max_sz {
            if path[idx1+i] != path[idx2+i] || colors[idx1+i].is_some() {
                return i;
            }
        }
        return max_sz;
    };

    let color_func = |func: &(&'a str, Vec<String>), colors: &mut Vec<Option<&'a str>>| {
        'OUTER: for p in 0..(path.len() - func.1.len() + 1) {
            for idx in 0..func.1.len() {
                if colors[p+idx].is_some() || path[p+idx] != func.1[idx] {
                    continue 'OUTER;
                }
            }
            for idx in 0..func.1.len() {
                colors[p+idx] = Some(func.0);
            }
        }
    };

    let mut colors : Vec<Option<&str>> = path.iter().map(|_| None).collect();
    let mut funcs : Vec<(&str, Vec<String>)> = Vec::new();

    for i in 0..path.len() {
        if colors[i].is_some() {
            continue;
        }
        let match_len = (i+1..path.len()).map(|j| longest_match_at(i, j, &colors)).max().unwrap();
        if match_len == 0 {
            panic!("Couldn't find a longest match at {}", i);
        }
        if funcs.len() >= func_names.len() {
            panic!("Too many functions ({}) at {}", funcs.len(), i);
        }
        let func_name = func_names[funcs.len()];
        funcs.push((func_name, path[i..i+match_len].to_vec()));
        // println!("Added function {}: {}", func_names[funcs.len() - 1], path[i..i+match_len].iter().format(","));
        color_func(&funcs[funcs.len() - 1], &mut colors);
        // println!("Colors: {}", colors.iter().map(|v| v.unwrap_or("_")).format(","));
    }
    let mut top : Vec<&str> = Vec::new();
    while colors.len() > 0 {
        let c : &str = colors.remove(0).unwrap();
        top.push(c);
        for (fname, fval) in &funcs {
            if fname == &c {
                for _ in 0..fval.len() - 1 {
                    colors.remove(0);
                }
                break;
            }
        }
    }

    for cw in colors {
        let c = cw.unwrap();
        if top.len() == 0 || *top.last().unwrap() != c {
            top.push(c);
        }
    }
    return (top, funcs);
}

fn execute_path(
    program: &Vec<i64>,
    grid: &HashMap<(i64, i64), TileType>,
    robot: &Robot,
    top_func: &Vec<&str>,
    sub_funcs: &Vec<(&str, Vec<String>)>
) -> i64 {
    let mut live_program = program.to_vec();
    live_program[0] = 2;
    println!("Top: {}", top_func.iter().format(","));
    for (sf_name, sf_val) in sub_funcs {
        println!("{}: {}", sf_name, sf_val.iter().format(","));
    }

    let video_feed = false;

    let mut line = String::new();
    line.push_str(top_func.iter().format(",").to_string().as_str());
    line.push('\n');
    for (sf_name, sf_val) in sub_funcs {
        line.push_str(sf_val.iter().format(",").to_string().as_str());
        line.push('\n');
    }
    line.push(if video_feed { 'y' } else { 'n' });
    line.push('\n');

    let mut inch : Vec<i64> = line.chars().map(|c| c as u32 as i64).collect();
    let input_fn = || -> Option<i64> {
        if inch.len() > 0 {
            return Some(inch.remove(0));
        } else {
            return None;
        }
    };

    let mut dust_count = 0;
    let output_fn = |i: i64| {
        if i > 127 || dust_count != 0 {
            println!("Received dust report: {}", i);
            dust_count = i;
        } else {
            let c = std::char::from_u32(i as u32).unwrap();
            print!("{}", c);
        }
    };

    run_program(&live_program, input_fn, output_fn);
    return dust_count;
}

fn main() {
    let args : Vec<String> = std::env::args().collect();
    if args[1] == "1" {
        println!("Doing part 1");
        let program = read_input(&args[2]).unwrap();
        let (robot, grid) = view_cameras(&program);
        print_grid(&grid, &robot);
        let alignments = sum_alignments(&grid);
        println!("Alignments: {}", alignments);
    } else {
        println!("Doing part 2");
        let program = read_input(&args[2]).unwrap();
        let (robot, grid) = view_cameras(&program);
        let path = calc_path(&grid, &robot);
        let (top, funcs) = compress_path(path, vec!["A", "B", "C"]);
        let dust = execute_path(&program, &grid, &robot, &top, &funcs);
        println!("Collected {} dust", dust);
    }
}
