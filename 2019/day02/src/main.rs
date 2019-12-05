#[macro_use] extern crate lazy_static;

use failure::Error;
use itertools::Itertools;
use regex::Regex;
use std::io::{BufReader,BufRead};
use std::fs::File;

fn parse_int(val: &str) -> Result<usize, Error> {
    return Ok(val.parse::<usize>()?);
}

fn parse_line(line: &str) -> Result<Vec<usize>, Error> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r",\s*").unwrap();
    }
    return RE.split(&line).map(|val| parse_int(&val)).collect();
}

fn read_input(file: &str) -> Result<Vec<usize>, Error> {
    let f = File::open(file)?;
    let br = BufReader::new(f);
    let parsed : Result<Vec<Vec<usize>>, Error> = br.lines().map(|line| parse_line(&line?)).collect();
    return Ok(parsed?.into_iter().flatten().collect());
}

fn run_program(mut program: Vec<usize>) -> Vec<usize> {
    let idx = |i: usize, program: &Vec<usize>| -> usize {
        if i >= program.len() {
            panic!("Index {} outside of program bound ({})", i, program.len());
        }
        return program[i];
    };
    let get = |i: usize, program: &Vec<usize>| -> usize {
        return idx(idx(i, program), program);
    };

    let mut pc = 0;
    loop {
        if pc >= program.len() {
            panic!("PC {} outside of program bound ({})", pc, program.len());
        }
        if program[pc] == 1 {
            let ix = idx(pc+3, &program);
            program[ix] = get(pc+1, &program) + get(pc+2, &program);
            pc += 4;
        } else if program[pc] == 2 {
            let ix = idx(pc+3, &program);
            program[ix] = get(pc+1, &program) * get(pc+2, &program);
            pc += 4;
        } else if program[pc] == 99 {
            println!("Saw 99, exiting");
            return program;
        } else {
            panic!("Unknown opcode at {}: {}", pc, program[pc]);
        }
    }
}

fn print_program(program: &Vec<usize>) {
    println!("Program: {}", program.into_iter().map(|v| v.to_string()).format(","));
}

fn main() {
    let args : Vec<String> = std::env::args().collect();
    if args[1] == "1" {
        println!("Doing part 1");
        let mut vals = read_input(&args[2]).unwrap();
        print_program(&vals);
        vals[1] = 12;
        vals[2] = 2;
        vals = run_program(vals);
        print_program(&vals);
    } else {
        println!("Doing part 2");
        let mut vals = read_input(&args[2]).unwrap();
        print_program(&vals);
        'outer: for n in 0..100 {
            for v in 0..100 {
                vals[1] = n;
                vals[2] = v;
                let new_vals = run_program(vals.to_vec());
                println!("Return is {} x {} = {}", n, v, new_vals[0]);
                if new_vals[0] == 19690720 {
                    println!("Value is {}", 100 * n + v);
                    break 'outer;
                }
            }
        }
    }
}
