#[macro_use] extern crate lazy_static;

use failure::Error;
use itertools::Itertools;
use regex::Regex;
use std::cell::RefCell;
use std::collections::{BinaryHeap,HashMap,HashSet};
use std::cmp::Ordering;
use std::io::{BufReader,BufRead};
use std::fs::File;
use std::sync::Mutex;

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

fn setup_network(program: &Vec<i64>, num_computers: i32, pt1: bool) -> i64 {
    let input_queue : Mutex<Vec<Vec<i64>>> = Mutex::new(Vec::new());
    {
        let mut queue_ref = input_queue.lock().unwrap();
        for i in 0..num_computers {
            queue_ref.push(vec![i as i64]);
        }
    }

    let nat : Mutex<Vec<i64>> = Mutex::new(Vec::new());
    let last_restart = Mutex::new(-1);
    let return_code : Mutex<Option<i64>> = Mutex::new(None);

    crossbeam::scope(|scope| {
        for idx in 0..num_computers {
            let input_queue_ref = &input_queue;
            let return_code_ref = &return_code;

            let input_fn = move|| -> Option<i64> {
                let mut queue_ref = input_queue_ref.lock().unwrap();
                if queue_ref[idx as usize].len() > 0 {
                    return Some(queue_ref[idx as usize].remove(0));
                } else {
                    if return_code_ref.lock().unwrap().is_none() {
                        return Some(-1);
                    } else {
                        return None;
                    }
                }
            };

            let mut packet = Vec::new();
            let nat_ref = &nat;
            let last_restart_ref = &last_restart;
            let output_fn = move|i: i64| {
                packet.push(i);
                if packet.len() == 3 {
                    if packet[0] == 255 {
                        println!("Sent to 255: {} {}", packet[1], packet[2]);
                        if pt1 {
                            *return_code_ref.lock().unwrap() = Some(packet[2]);
                        } else {
                            let mut nat_val = nat_ref.lock().unwrap();
                            nat_val.clear();
                            nat_val.push(packet[1]);
                            nat_val.push(packet[2]);

                            let mut queue_ref = input_queue_ref.lock().unwrap();
                            if queue_ref.iter().all(|q| q.len() == 0) {
                                println!("Everything's idle, restarting network with {} {}", packet[1], packet[2]);
                                let mut last_restart_val = last_restart_ref.lock().unwrap();
                                if *last_restart_val == packet[2] {
                                    *return_code_ref.lock().unwrap() = Some(packet[2]);
                                } else {
                                    *last_restart_val = packet[2];
                                    queue_ref[0].push(nat_val[0]);
                                    queue_ref[0].push(nat_val[1]);
                                }
                            }
                        }
                    } else {
                        let mut queue_ref = input_queue_ref.lock().unwrap();
                        queue_ref[packet[0] as usize].push(packet[1]);
                        queue_ref[packet[0] as usize].push(packet[2]);
                    }
                    packet.clear();
                }
            };

            let program_copy = program.to_vec();
            scope.spawn(move || -> () {
                run_program(&program_copy, input_fn, output_fn);
            });
        }
    });

    return (*return_code.lock().unwrap()).unwrap();
}

fn main() {
    let args : Vec<String> = std::env::args().collect();
    if args[1] == "1" {
        println!("Doing part 1");
        let program = read_input(&args[2]).unwrap();
        let num_computers = 50;
        let rc = setup_network(&program, num_computers, true);
        println!("Return code: {}", rc);
    } else {
        println!("Doing part 2");
        let program = read_input(&args[2]).unwrap();
        let num_computers = 50;
        let rc = setup_network(&program, num_computers, false);
        println!("Return code: {}", rc);
    }
}
