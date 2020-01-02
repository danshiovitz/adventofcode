#[macro_use] extern crate lazy_static;
#[macro_use] extern crate failure;

use failure::Error;
use itertools::Itertools;
use mod_exp::mod_exp;
use modinverse::modinverse;
use regex::Regex;
use std::cell::RefCell;
use std::collections::{BinaryHeap,HashMap,HashSet};
use std::cmp::Ordering;
use std::io::{BufReader,BufRead};
use std::fs::File;
use std::path::Path;

#[derive(PartialEq, Eq, Debug, Hash)]
enum Technique {
    DealNewStack,
    Cut { n: i64 },
    DealWithIncrement { inc: i64 },
}

fn parse_int(val: &str) -> Result<i64, Error> {
    return Ok(val.parse::<i64>()?);
}

fn parse_line(line: &str) -> Result<Technique, Error> {
    lazy_static! {
        static ref DEAL_NEW_STACK_RE: Regex = Regex::new(r"deal into new stack").unwrap();
        static ref CUT_RE: Regex = Regex::new(r"cut (-?\d+)$").unwrap();
        static ref DEAL_INC_RE: Regex = Regex::new(r"deal with increment (-?\d+)$").unwrap();
    }
    match DEAL_NEW_STACK_RE.captures(line) {
        Some(_caps) => {
            return Ok(Technique::DealNewStack);
        },
        None => {}
    }
    match CUT_RE.captures(line) {
        Some(caps) => {
            return Ok(Technique::Cut { n: parse_int(caps.get(1).unwrap().as_str())? });
        },
        None => {}
    }
    match DEAL_INC_RE.captures(line) {
        Some(caps) => {
            return Ok(Technique::DealWithIncrement { inc: parse_int(caps.get(1).unwrap().as_str())? });
        },
        None => {}
    }
    bail!("Bad line: {}", line);
}

fn read_input(file: &str) -> Result<Vec<Technique>, Error> {
    let f = File::open(file)?;
    let br = BufReader::new(f);
    return br.lines().map(|line| parse_line(&line?)).collect();
}

fn shuffle_deck(techniques: &Vec<Technique>, deck_size: i64, reps: i64) -> Box<dyn Fn(i64) -> i64> {
    let mut deck = vec![0; deck_size as usize];
    for i in 0..deck.len() {
        deck[i] = i as i64;
    }
    for _ in 0..reps {
        for tech in techniques {
            match tech {
                Technique::DealNewStack => {
                    deck.reverse();
                },
                Technique::Cut { n } => {
                    if *n > 0 {
                        deck.rotate_left(*n as usize);
                    } else {
                        deck.rotate_right(n.abs() as usize);
                    }
                },
                Technique::DealWithIncrement { inc } => {
                    let mut new_deck = vec![-1; deck_size as usize];
                    let mut new_idx = 0;
                    for old_idx in 0..deck.len() {
                        new_deck[new_idx] = deck[old_idx];
                        new_idx = (new_idx as i64 + *inc + deck.len() as i64) as usize % deck.len();
                    }
                    deck = new_deck;
                }
                t => {
                    panic!("Unknown technique: {:?}", t);
                }
            }
        }
    }
    return Box::new(move|idx| {
        return deck[idx as usize];
    });
}

fn shuffle_big_deck(techniques: &Vec<Technique>, deck_size: i64, reps: i64) -> Box<dyn Fn(i64) -> i64> {
    let mut simplified = Vec::new();
    for tech in techniques  {
        match tech {
            Technique::DealNewStack => {
                match simplified.last_mut() {
                    Some(Technique::DealNewStack {}) => { simplified.pop(); },
                    _ => { simplified.push(Technique::DealNewStack {}) }
                }
            },
            Technique::Cut { n } => {
                match simplified.last_mut() {
                    Some(Technique::Cut { n: last_n }) => { *last_n += n; },
                    _ => { simplified.push(Technique::Cut { n: *n }) }
                }
            },
            Technique::DealWithIncrement { inc } => {
                match simplified.last_mut() {
                    Some(Technique::DealWithIncrement { inc: last_inc }) => {
                        *last_inc = (*last_inc * inc) % deck_size;
                    },
                    _ => { simplified.push(Technique::DealWithIncrement { inc: *inc }) }
                }
            }
            t => {
                panic!("Unknown technique: {:?}", t);
            }
        }
    }

    let mut offset = 0;
    let mut spread : i64 = 1;
    let mut reverse = false;

    for tech in &simplified {
        match tech {
            Technique::DealNewStack {} => {
                offset *= -1;
                offset -= (spread - 1);
                offset %= deck_size;
                //println!("offset i {}", offset);
                reverse = !reverse;
            },
            Technique::Cut { n } => {
                offset += n;
                offset %= deck_size;
                //println!("offset i {}", offset);
            },
            Technique::DealWithIncrement { inc } => {
                spread = ((spread as i128 * *inc as i128) % deck_size as i128) as i64;
                offset = ((offset as i128 * *inc as i128) % deck_size as i128) as i64;
                //println!("offset i {}", offset);
            }
            t => {
                panic!("Unknown technique: {:?}", t);
            }
        }
    }

    if offset < 0 {
        offset += deck_size;
    }
    println!("Reverse: {}, Offset: {}; Spread: {}", reverse, offset, spread);

    if reps > 1 {
        let final_reverse = if reverse && reps % 2 == 0 { !reverse } else { reverse };
        let final_spread = mod_exp(spread as i128, reps as i128, deck_size as i128) as i64;

        // final offset is much more complicated -
        // let mut calc = offset1 * -1; // based on prev offset
        // calc -= spread1 - 1; // based on prev spread
        // calc *= spread1; // based on initial spread
        // calc += offset1; // based on initial offset
        // calc %= deck_sz;
        //
        // f(n) = -spread * f(n-1) - spread^n + spread + offset
        //
        // Grinding out the recurrence relation I get
        //
        // f(n) = spread + offset + (sum i in 1..(n-1) of (-spread^i * (offset - 1))) - spread^n
        // But the final spread^n term only appears when n is even for some reason
        // This is only correct when reverse = true, but I'm not going to bother to handle the other case
        // for now
        let mminv = modinverse(-spread - 1 + deck_size, deck_size).unwrap();
        let sq_sum = (((mod_exp(-spread as i128, reps as i128, deck_size as i128) + spread as i128) * mminv as i128) % deck_size as i128);
        let sq_sum_by_offset = ((sq_sum * (offset as i128 - 1)) % deck_size as i128) as i64;
        let mut final_offset = (sq_sum_by_offset + offset) % deck_size;
        if reps % 2 == 0 {
            final_offset -= mod_exp(spread as i128, reps as i128, deck_size as i128) as i64;
            final_offset %= deck_size;
        }
        while final_offset < 0 {
            final_offset += deck_size;
        }

        reverse = final_reverse;
        offset = final_offset;
        spread = final_spread;
        println!("After {} total reps - Reverse: {}, Offset: {}; Spread: {}", reps, reverse, offset, spread);
    }

    // I don't know enough math to know why this is a thing, but it is:
    let spread_inverse = modinverse(spread, deck_size).unwrap();

    return Box::new(move|idx| {
        let mut orig_idx : i64 = idx;
        if offset != 0 {
            orig_idx += offset;
            orig_idx = (orig_idx + deck_size) % deck_size;
        }
        if spread != 1 {
            let sim : i128 = (spread_inverse % deck_size).into();
            let oim : i128 = (orig_idx % deck_size).into();
            orig_idx = ((sim * oim) % deck_size as i128) as i64;
        }
        if reverse {
            orig_idx = deck_size - orig_idx - 1;
        }
        return orig_idx;
    });
}

fn do_compare(deck_size: i64, techniques: &Vec<Technique>, reps: i64) {
    let normal_func = shuffle_deck(&techniques, deck_size, reps);
    let faster_func = shuffle_big_deck(&techniques, deck_size, reps);
    if deck_size <= 100 {
        let mut deck : Vec<i64> = (0..deck_size).map(|i| normal_func(i)).collect();
        println!("Normal shuffle: {:?}", deck);
        deck = (0..deck_size).map(|i| faster_func(i)).collect();
        println!("Faster shuffle: {:?}", deck);
    }
    for idx in 0..deck_size {
        let nval = normal_func(idx);
        let fval = faster_func(idx);
        assert!(nval == fval, "Mismatch at {}: {} vs {}", idx, nval, fval);
    }
}

fn run_unit_tests() {
    let tests = vec![
        (10, 1, vec![
            "deal with increment 9",
            "deal with increment 3",
            "deal into new stack",
        ]),
        (10, 1, vec![
            "deal into new stack",
            "deal with increment 9",
            "deal with increment 3",
        ]),
        (10, 1, vec![
            "deal with increment 9",
            "deal into new stack",
            "deal with increment 3",
        ]),
        (10, 1, vec![
            "deal with increment 7",
            "deal into new stack",
            "deal into new stack",
        ]),
        (10, 1, vec![
            "cut 6",
            "deal with increment 7",
            "deal into new stack",
        ]),
        (10, 1, vec![
            "deal with increment 7",
            "deal with increment 9",
            "cut -2",
        ]),
        (10, 1, vec![
            "deal into new stack",
            "cut -2",
            "deal with increment 7",
            "cut 8",
            "cut -4",
            "deal with increment 7",
            "cut 3",
            "deal with increment 9",
            "deal with increment 3",
            "cut -1",
        ]),
        (10, 1, vec![
            "deal into new stack",
            "deal with increment 3",
            "deal into new stack",
        ]),
        (10, 1, vec![
            "deal with increment 3",
            "deal into new stack",
            "deal into new stack",
        ]),
        (10007, 2, vec![
            "cut 3",
            // "deal into new stack",
            // "deal with increment 3",
            // "deal into new stack",
        ]),
    ];

    for idx in 0..tests.len() {
        let deck_size = tests[idx].0;
        let reps = tests[idx].1;
        let techniques: Vec<Technique> = tests[idx].2.iter().map(|line| parse_line(&line).unwrap()).collect();
        println!("Running test {}", idx);
        do_compare(deck_size, &techniques, reps);
    }
}

fn main() {
    let args : Vec<String> = std::env::args().collect();
    if args[1] == "tests" {
        run_unit_tests();
    } else if args[1] == "cmp" {
        assert!(args.len() > 3, "Usage: day22 cmp <deck size> <technique or file>...");
        let deck_size = parse_int(&args[2]).unwrap();
        let techniques = if Path::new(&args[3]).exists() {
            read_input(&args[3]).unwrap()
        } else {
            args[3..].iter().map(|line| parse_line(&line).unwrap()).collect()
        };
        do_compare(deck_size, &techniques, 1009);
        println!("Everything's dandy!");
    } else if args[1] == "1" {
        println!("Doing part 1");
        let techniques = read_input(&args[2]).unwrap();
        let deck_size = 10007;
        let shuffle_func = shuffle_big_deck(&techniques, deck_size, 1);
        let pos = (0..deck_size).find(|p| shuffle_func(*p) == 2019);
        println!("Position of card 2019: {}", pos.unwrap());
    } else {
        println!("Doing part 2");
        let techniques = read_input(&args[2]).unwrap();
        let deck_size = 119315717514047;
        let reps = 101741582076661;
        let pos = 2020;
        let shuffle_func = shuffle_big_deck(&techniques, deck_size, reps);
        println!("The value in position {} is {}", pos, shuffle_func(pos));
    }
}
