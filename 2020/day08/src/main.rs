#[macro_use] extern crate lazy_static;
#[macro_use] extern crate failure;

use failure::{Error, bail};
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::io::{BufReader,BufRead};
use std::fs::File;

#[derive(PartialEq, Eq, Clone, Debug, Hash)]
struct Instruction {
    opcode: String,
    val1: i64,
}

#[derive(PartialEq, Eq, Clone, Copy, Debug, Hash)]
struct State {
    pc: i64,
    accumulator: i64,
    exit_code: i64,
}

fn parse_int(val: &str) -> Result<i64, Error> {
    return Ok(val.parse::<i64>()?);
}

fn parse_line(line: &str) -> Result<Instruction, Error> {
    lazy_static! {
        static ref LINE_RE: Regex = Regex::new(r"^(\w+) ([+-]?\d+)$").unwrap();
    }
    match LINE_RE.captures(line) {
        Some(caps) => {
            return Ok(Instruction {
                opcode: caps.get(1).unwrap().as_str().to_string(),
                val1: parse_int(caps.get(2).unwrap().as_str())?,
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

type InstructionFn = fn(&Instruction, &mut State) -> ();

fn do_acc(inst: &Instruction, mut state: &mut State) -> () {
    state.accumulator += inst.val1;
    state.pc += 1;
}

fn do_jmp(inst: &Instruction, mut state: &mut State) -> () {
    state.pc += inst.val1;
}

fn do_nop(_inst: &Instruction, mut state: &mut State) -> () {
    state.pc += 1;
}

fn run_program(instructions: &Vec<Instruction>, init_state: State) -> State {
    lazy_static! {
        static ref OPCODES: HashMap<&'static str, InstructionFn> = {
            let mut m : HashMap<&'static str, InstructionFn> = HashMap::new();
            m.insert("acc", do_acc);
            m.insert("jmp", do_jmp);
            m.insert("nop", do_nop);
            m
        };
    }

    let mut state = init_state;
    let mut prev_pcs : HashSet<i64> = HashSet::new();
    loop {
        if prev_pcs.contains(&state.pc) {
            state.exit_code = 99;
            return state;
        }
        prev_pcs.insert(state.pc);
        if state.pc < 0 || state.pc >= instructions.len() as i64 {
            state.exit_code = 0;
            return state;
        }
        let inst = &instructions[state.pc as usize];
        OPCODES.get(inst.opcode.as_str()).unwrap()(&inst, &mut state);
    }
}

fn flip_some(instructions: &Vec<Instruction>, init_state: State) -> State {
    for f in 0..instructions.len() {
        let mut cpy : Vec<Instruction> = instructions.iter().cloned().collect();
        if cpy[f].opcode == "jmp" {
            cpy[f].opcode = "nop".to_string();
        } else if cpy[f].opcode == "nop" {
            cpy[f].opcode = "jmp".to_string();
        } else {
            continue;
        }
        let ret = run_program(&cpy, init_state);
        if ret.exit_code == 0 {
            return ret;
        }
    }
    panic!("Got through to the end without finding termination!");
}

fn main() {
    let args : Vec<String> = std::env::args().collect();
    if args[1] == "1" {
        println!("Doing part 1");
        let instructions = read_input(&args[2]).unwrap();
        let state = run_program(&instructions, State { pc: 0, accumulator: 0, exit_code: -1 });
        println!("Final state is {:?}", state);
    } else {
        println!("Doing part 2");
        let instructions = read_input(&args[2]).unwrap();
        let state = flip_some(&instructions, State { pc: 0, accumulator: 0, exit_code: -1 });
        println!("Final state is {:?}", state);
    }
}
