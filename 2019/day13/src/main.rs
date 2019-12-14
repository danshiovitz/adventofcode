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
    EMPTY,
    WALL,
    BLOCK,
    PADDLE,
    BALL,
}

static TILE_TYPES : [TileType; 5] = [
    TileType::EMPTY, TileType::WALL, TileType::BLOCK, TileType::PADDLE, TileType::BALL,
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
}

fn run_program(program_init: Vec<i64>, mut input: impl FnMut() -> Option<i64>, mut output: impl FnMut(i64)) -> ExitType {
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

fn play_game(program: Vec<i64>, mut input: impl FnMut(&HashMap<(i64, i64), TileType>) -> Option<i64>) ->
        (HashMap<(i64, i64), TileType>, Option<i64>) {
    let mut stage = 0;
    let mut cur_x = 0;
    let mut cur_y = 0;
    let mut score = 0;
    let grid_ref : RefCell<HashMap<(i64, i64), TileType>> = RefCell::new(HashMap::new());

    let input_fn = || -> Option<i64> {
        let grid = grid_ref.borrow();
        return input(&*grid);
    };

    let output_fn = |i: i64| {
        if stage == 0 {
            cur_x = i;
            stage += 1;
        } else if stage == 1 {
            cur_y = i;
            stage += 1;
        } else if stage == 2 {
            if cur_x == -1 && cur_y == 0 {
                score = i;
            } else {
                let cur_type = TILE_TYPES[i as usize];
                let mut grid = grid_ref.borrow_mut();
                grid.insert((cur_x, cur_y), cur_type);
            }
            stage = 0;
        } else {
            panic!("Bad stage: {}", stage);
        }
    };

    match run_program(program, input_fn, output_fn) {
        ExitType::InputAbort => { return (grid_ref.into_inner(), None); },
        ExitType::Finish99 => { return (grid_ref.into_inner(), Some(score)); },
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
            match grid.get(&(x, y)) {
                Some(TileType::EMPTY) => line.push(' '),
                Some(TileType::WALL) => line.push('.'),
                Some(TileType::BLOCK) => line.push('X'),
                Some(TileType::PADDLE) => line.push('@'),
                Some(TileType::BALL) => line.push('O'),
                None => line.push('?'),
            }
        }
        println!("{}", line);
    }
}

#[derive(Debug, Eq, PartialEq)]
struct State {
    blocks: usize,
    seq: Vec<i64>,
}

// The priority queue depends on `Ord`.
// Explicitly implement the trait so the queue becomes a min-heap
// instead of a max-heap.
impl Ord for State {
    fn cmp(&self, other: &State) -> Ordering {
        if self.blocks != other.blocks {
            return other.blocks.cmp(&self.blocks);
        }
        if self.seq.len() != other.seq.len() {
            return other.seq.len().cmp(&self.seq.len());
        }
        for i in 0..self.seq.len() {
            if self.seq[i] != other.seq[i] {
                return other.seq[i].cmp(&self.seq[i]);
            }
        }
        return Ordering::Equal;
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &State) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn solve_game_tree(program: Vec<i64>) -> (Vec<i64>, i64) {
    let moves = vec![-1, 0, 1];

    let mut heap = BinaryHeap::new();
    heap.push(State { blocks: std::usize::MAX, seq: vec![] });
    while let Some(State { blocks, seq }) = heap.pop() {
        let mut cpseq = seq.to_vec();
        let mut input = |_g: &HashMap<(i64, i64), TileType>| { return Some(cpseq.remove(0)); };
        let (tiles, score) = play_game(program.to_vec(), input);
        let block_count = tiles.values().filter(|v| *v == &TileType::BLOCK).count();
        match score {
            Some(s) => {
                println!("Exit after {:?}, score is {}, blocks is {}", seq, s, block_count);
                if block_count == 0 {
                    return (seq, s);
                }
            },
            None => {
                println!("Ran out after {:?}, blocks is {}", seq, block_count);
                for m in &moves {
                    let mut next = seq.to_vec();
                    next.push(*m);
                    heap.push(State { blocks: block_count, seq: next });
                }
            }
        }
    }
    panic!("Ran out of sequences with the game unsolved!");
}

fn prompt(msg: &str) -> i64 {
    print!("{}: ", msg);
    let mut input = String::new();
    std::io::stdin().read_line(&mut input);

    let value: i64 = input.trim().parse().unwrap();
    return value;
}

fn solve_game_manual(program: Vec<i64>) -> (Vec<i64>, i64) {
    let moves = vec![-1, 0, 1];
    let mut seq = Vec::new();
    let input = |grid: &HashMap<(i64, i64), TileType>| {
        print_grid(&grid);
        let mmove = prompt("Go");
        seq.push(mmove);
        return Some(mmove);
    };

    let (tiles, score) = play_game(program.to_vec(), input);
    let block_count = tiles.values().filter(|v| *v == &TileType::BLOCK).count();
    match score {
        Some(s) => {
            println!("Exit after {:?}, score is {}, blocks is {}", seq, s, block_count);
            return (seq, s);
        },
        None => {
            println!("Ran out after {:?}, blocks is {}", seq, block_count);
            return (seq, -1);
        }
    }
}

fn solve_game_under(program: Vec<i64>) -> (Vec<i64>, i64) {
    let moves = vec![-1, 0, 1];
    let mut seq = Vec::new();
    let input = |grid: &HashMap<(i64, i64), TileType>| {
        let mut ball_x = 0;
        let mut paddle_x = 0;
        for (key, val) in grid.iter() {
            if val == &TileType::BALL {
                ball_x = key.0;
            } else if val == &TileType::PADDLE {
                paddle_x = key.0;
            }
        }
        let mmove =
            if ball_x < paddle_x { -1 }
            else if ball_x > paddle_x { 1 }
            else { 0 };
        seq.push(mmove);
        return Some(mmove);
    };

    let (tiles, score) = play_game(program.to_vec(), input);
    let block_count = tiles.values().filter(|v| *v == &TileType::BLOCK).count();
    match score {
        Some(s) => {
            println!("Exit after {:?}, score is {}, blocks is {}", seq, s, block_count);
            return (seq, s);
        },
        None => {
            println!("Ran out after {:?}, blocks is {}", seq, block_count);
            return (seq, -1);
        }
    }
}

fn main() {
    let args : Vec<String> = std::env::args().collect();
    if args[1] == "1" {
        println!("Doing part 1");
        let program = read_input(&args[2]).unwrap();
        let (tiles, _) = play_game(program, |_g| { None });
        let block_count = tiles.values().filter(|v| *v == &TileType::BLOCK).count();
        print_grid(&tiles);
        println!("Found {} blocks", block_count);
    } else {
        println!("Doing part 2");
        let mut program = read_input(&args[2]).unwrap();
        program[0] = 2;
        let (seq, score) = solve_game_under(program);
        println!("Final score: {} after {} moves", score, seq.len());
    }
}
