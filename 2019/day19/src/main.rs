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

#[derive(PartialEq, Eq, Copy, Clone, Debug, Hash)]
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

fn print_points(points: &HashSet<(i64, i64)>) {
    let mut min_x = std::i64::MAX;
    let mut max_x = std::i64::MIN;
    let mut min_y = std::i64::MAX;
    let mut max_y = std::i64::MIN;

    for val in points {
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
            if points.contains(&(x, y)) {
                line.push('#');
            } else {
                line.push('.');
            }
        }
        println!("{}", line);
    }
}

fn check_square(x: i64, y: i64, program: &Vec<i64>) -> bool {
    let mut vals = vec![y, x]; // reverse so we can pop()
    let input_fn = || -> Option<i64> {
        return vals.pop();
    };

    let mut ret = false;
    let output_fn = |i: i64| {
        if i == 0 {
            ret = false;
        } else if i == 1 {
            ret = true;
        } else {
            panic!("Unknown output: {}", i);
        }
    };

    run_program(program, input_fn, output_fn);
    return ret;
}

fn beam_check(program: &Vec<i64>) -> i32 {
    let mut points : HashSet<(i64, i64)> = HashSet::new();

    for x in 0..50 {
        for y in 0..50 {
            if check_square(x, y, &program) {
                points.insert((x, y));
            }
        }
    }

    print_points(&points);
    return points.len() as i32;
}

fn find_bounds(y: i64, x_hint: (i64, i64), program: &Vec<i64>) -> (i64, i64) {
    let mut min_x = x_hint.0;
    while !check_square(min_x, y, &program) {
        min_x += 1;
        if min_x - x_hint.0 > 1000 {
            panic!("Min hint of {} is off by too much ({}), this is surprising", x_hint.0, min_x);
        }
    }

    let mut max_x = std::cmp::max(min_x, x_hint.1);
    while check_square(max_x + 1, y, &program) {
        max_x += 1;
        if max_x - x_hint.0 > 1000 {
            panic!("Max hint of {} is off by too much ({}), this is surprising", x_hint.1, min_x);
        }
    }

    return (min_x, max_x);
}

fn fit_ship(x_size: i64, y_size: i64, program: &Vec<i64>) -> (i64, i64) {
    let mut y = y_size;
    let mut x_bounds = (0,0);
    'OUTER: for y_bump in vec![100, 10, 1] {
        loop {
            let top_bounds = find_bounds(y, x_bounds, &program);
            let bottom_bounds = find_bounds(y + y_size - 1, top_bounds, &program);
            // println!("Line {} found ({},{}) / line {} found ({},{})",
            //     y, top_bounds.0, top_bounds.1, y+y_size-1, bottom_bounds.0, bottom_bounds.1);
            if top_bounds.1 - bottom_bounds.0 + 1 >= x_size {
                if y_bump == 1 {
                    return (bottom_bounds.0, y);
                } else {
                    // Back out the last adjustment:
                    y -= y_bump;
                    x_bounds = (x_bounds.0 - y_bump, x_bounds.1 - y_bump);
                    continue 'OUTER;
                }
            }
            y += y_bump;
            x_bounds = top_bounds;
        }
    }
    panic!("Didn't find a fit?");
}

fn print_around(corner: (i64, i64), program: &Vec<i64>) {
    let mut points : HashSet<(i64, i64)> = HashSet::new();

    for x in (corner.0 - 1)..(corner.0 + 30) {
        for y in (corner.1 - 1)..(corner.1 + 30) {
            if check_square(x, y, &program) {
                points.insert((x, y));
            }
        }
    }

    print_points(&points);
}

fn main() {
    let args : Vec<String> = std::env::args().collect();
    if args[1] == "1" {
        println!("Doing part 1");
        let program = read_input(&args[2]).unwrap();
        let beam_count = beam_check(&program);
        println!("Pulled squares: {}", beam_count);
    } else {
        println!("Doing part 2");
        let program = read_input(&args[2]).unwrap();
        let corner = fit_ship(100, 100, &program);
        print_around(corner, &program);
        println!("Found corner at {},{} = {}", corner.0, corner.1, (corner.0 * 10000 + corner.1));
    }
}
