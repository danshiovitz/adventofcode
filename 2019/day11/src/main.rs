#[macro_use] extern crate lazy_static;

use failure::Error;
use itertools::Itertools;
use regex::Regex;
use std::cell::RefCell;
use std::collections::{HashMap,HashSet};
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

fn run_program(program_init: Vec<i64>, input: impl Fn() -> i64, mut output: impl FnMut(i64)) {
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
            set(pc, 1, modes, value, &mut program);
            pc += 2;
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
            return;
        } else {
            panic!("Unknown opcode at {}: {}", pc, get(pc, 0, 0, &program));
        }
    }
}

fn paint_squares(program: Vec<i64>, pt1: bool) -> (HashSet<(i32, i32)>, HashSet<(i32, i32)>) {
    let painted_ref : RefCell<HashSet<(i32, i32)>> = RefCell::new(HashSet::new());
    let ever_painted_ref : RefCell<HashSet<(i32, i32)>> = RefCell::new(HashSet::new());
    let pos_ref : RefCell<(i32, i32)> = RefCell::new((0, 0));
    let mut dir = (0, -1);

    if !pt1 {
        painted_ref.borrow_mut().insert((0, 0));
    }

    let input_fn = || -> i64 {
        let painted = painted_ref.borrow();
        let pos = pos_ref.borrow();
        if painted.contains(&*pos) {
            // println!("Scanned white");
            return 1;
        } else {
            // println!("Scanned black");
            return 0;
        }
    };

    let mut do_paint = true;
    let output_fn = |i: i64| {
        let mut painted = painted_ref.borrow_mut();
        let mut ever_painted = ever_painted_ref.borrow_mut();
        let mut pos = pos_ref.borrow_mut();
        if do_paint {
            if i == 0 {
                painted.remove(&*pos);
                ever_painted.insert(*pos);
                // println!("Paint black");
            } else {
                painted.insert(*pos);
                ever_painted.insert(*pos);
                // println!("Paint white");
            }
        } else {
            // 0 left, 1 right
            if dir == (0, -1) && i == 0 || dir == (0, 1) && i == 1 {
                dir = (-1, 0);
            } else if dir == (-1, 0) && i == 0 || dir == (1, 0) && i == 1 {
                dir = (0, 1);
            } else if dir == (0, 1) && i == 0 || dir == (0, -1) && i == 1 {
                dir = (1, 0);
            } else if dir == (1, 0) && i == 0 || dir == (-1, 0) && i == 1 {
                dir = (0, -1);
            } else {
                panic!("Bad dir {:?} or turn {:?}", dir, i);
            }
            *pos = (pos.0 + dir.0, pos.1 + dir.1);
            // println!("New position: {:?}\n", *pos);
        }
        do_paint = !do_paint;
    };

    run_program(program, input_fn, output_fn);
    return (painted_ref.into_inner(), ever_painted_ref.into_inner());
}

fn draw_painted(painted: &HashSet<(i32, i32)>) {
    let mut min_x = std::i32::MAX;
    let mut max_x = std::i32::MIN;
    let mut min_y = std::i32::MAX;
    let mut max_y = std::i32::MIN;

    for val in painted {
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
            if painted.contains(&(x, y)) {
                line.push('#');
            } else {
                line.push('.');
            }
        }
        println!("{}", line);
    }
}

fn main() {
    let args : Vec<String> = std::env::args().collect();
    if args[1] == "1" {
        println!("Doing part 1");
        let program = read_input(&args[2]).unwrap();
        let (painted, ever_painted) = paint_squares(program, true);
        draw_painted(&painted);
        println!("(Ever) painted {} squares", ever_painted.len());
    } else {
        println!("Doing part 2");
        let program = read_input(&args[2]).unwrap();
        let (painted, ever_painted) = paint_squares(program, false);
        draw_painted(&painted);
        println!("(Ever) painted {} squares", ever_painted.len());
    }
}
