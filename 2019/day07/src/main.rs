#[macro_use] extern crate lazy_static;

use failure::Error;
use itertools::Itertools;
use permute::permute;
use regex::Regex;
use std::io::{BufReader,BufRead};
use std::fs::File;
use std::sync::Mutex;
use std::sync::mpsc::channel;

fn parse_int(val: &str) -> Result<i32, Error> {
    return Ok(val.parse::<i32>()?);
}

fn parse_line(line: &str) -> Result<Vec<i32>, Error> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r",\s*").unwrap();
    }
    return RE.split(&line).map(|val| parse_int(&val)).collect();
}

fn read_input(file: &str) -> Result<Vec<i32>, Error> {
    let f = File::open(file)?;
    let br = BufReader::new(f);
    let parsed : Result<Vec<Vec<i32>>, Error> = br.lines().map(|line| parse_line(&line?)).collect();
    return Ok(parsed?.into_iter().flatten().collect());
}

fn run_program(mut program: Vec<i32>, input: impl Fn() -> i32, mut output: impl FnMut(i32)) -> Vec<i32> {
    let idx = |i: i32, program: &Vec<i32>| -> i32 {
        if i >= program.len() as i32 || i < 0 {
            panic!("Index {} outside of program bound ({})", i, program.len());
        }
        return program[i as usize];
    };
    let get = |pc: i32, num: i32, modes: i32, program: &Vec<i32>| -> i32 {
        let i = pc + num;
        let mut flag = modes;
        for _ in 0..num-1 {
            flag /= 10;
        }
        flag %= 10;
        if flag == 0 {
            return idx(idx(i, program), program);
        } else {
            return idx(i, program);
        }
    };

    let mut pc : i32 = 0;
    loop {
        if pc >= program.len() as i32 || pc < 0 {
            panic!("PC {} outside of program bound ({})", pc, program.len());
        }
        let opcode = program[pc as usize] % 100;
        let modes = program[pc as usize] / 100;
        if opcode == 1 {
            let ix = idx(pc+3, &program);
            program[ix as usize] = get(pc, 1, modes, &program) + get(pc, 2, modes, &program);
            pc += 4;
        } else if opcode == 2 {
            let ix = idx(pc+3, &program);
            program[ix as usize] = get(pc, 1, modes, &program) * get(pc, 2, modes, &program);
            pc += 4;
        } else if opcode == 3 {
            let ix = idx(pc+1, &program);
            program[ix as usize] = input();
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
            let ix = idx(pc+3, &program);
            if get(pc, 1, modes, &program) < get(pc, 2, modes, &program) {
                program[ix as usize] = 1;
            } else {
                program[ix as usize] = 0;
            }
            pc += 4;
        } else if opcode == 8 {
            let ix = idx(pc+3, &program);
            if get(pc, 1, modes, &program) == get(pc, 2, modes, &program) {
                program[ix as usize] = 1;
            } else {
                program[ix as usize] = 0;
            }
            pc += 4;
        } else if opcode == 99 {
            // println!("Saw 99, exiting");
            return program;
        } else {
            panic!("Unknown opcode at {}: {}", pc, program[pc as usize]);
        }
    }
}

fn run_amplifiers(program: Vec<i32>, phases: &Vec<i32>, pt1: bool) -> i32 {
    let mut txs = Vec::new();
    let mut rxs = Vec::new();
    for phase in phases {
        let (tx, rx) = channel();
        tx.send(*phase).unwrap();
        txs.push(tx);
        rxs.push(rx);
    }

    // Therefore the input for amp 0 is the tx in the last slot:
    let init_input = txs[0].clone();

    // Amp N should receive from socket N and send to socket N+1:
    txs.rotate_left(1);

    let last_outputs = Mutex::new(vec![0; phases.len()]);

    crossbeam::scope(|scope| {
        for amp_idx in 0..phases.len() {
            let rx = rxs.remove(0);
            let input_fn = move|| -> i32 {
                // println!("Thread #{} waiting for input ...", amp_idx);
                let val = rx.recv().unwrap();
                // println!("Thread #{} received {}", amp_idx, val);
                return val;
            };
            let last_outputs_ref = &last_outputs;
            let txo = if pt1 && amp_idx == phases.len() - 1 { None } else { Some(txs.remove(0)) };
            let output_fn = move|i: i32| {
                let mut outputs = last_outputs_ref.lock().unwrap();
                outputs[amp_idx] = i;
                // println!("Thread #{} sends: {}", amp_idx, i);
                // Send will fail when the other thread has ended:
                if let Some(ref tx) = txo {
                    let _ = tx.send(i);
                }
            };
            let program_copy = program.to_vec();
            scope.spawn(move || -> () {
                run_program(program_copy, input_fn, output_fn);
            });
        }
        init_input.send(0).unwrap();
    });

    let outputs = last_outputs.lock().unwrap();
    return outputs[outputs.len() - 1];
}

fn find_best_settings(program: Vec<i32>, phases: Vec<i32>, pt1: bool) -> (Vec<i32>, i32) {
    let mut best_order = vec![];
    let mut best_score = 0;
    for p in permute(phases) {
        let score = run_amplifiers(program.to_vec(), &p, pt1);
        println!("Found score of {} for {}", score, p.to_vec().into_iter().format(", "));
        if score > best_score {
            best_score = score;
            best_order = p.to_vec();
        }
    }
    return (best_order, best_score);
}

fn main() {
    let args : Vec<String> = std::env::args().collect();
    if args[1] == "1" {
        println!("Doing part 1");
        let program = read_input(&args[2]).unwrap();
        let phases = (0..=4).collect();
        let (best_order, best_score) = find_best_settings(program, phases, true);
        println!("Best score {} for {}", best_score, best_order.into_iter().map(|x| x.to_string()).format(", "));
    } else {
        println!("Doing part 2");
        let program = read_input(&args[2]).unwrap();
        let phases = (5..=9).collect();
        let (best_order, best_score) = find_best_settings(program, phases, false);
        println!("Best score {} for {}", best_score, best_order.into_iter().map(|x| x.to_string()).format(", "));
    }
}
