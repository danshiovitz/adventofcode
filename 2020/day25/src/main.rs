#[macro_use] extern crate failure;
#[macro_use] extern crate lazy_static;

use failure::{Error, bail};
use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::io::{BufReader, BufRead};
use std::fs::File;

fn read_input(file: &str) -> Result<(i64, i64), Error> {
    let f = File::open(file)?;
    let br = BufReader::new(f);

    let vals : Vec<String> = br.lines().map(|line| line.unwrap()).collect();
    if vals.len() != 2 {
        bail!("Expected 2 lines in input, found {}", vals.len());
    }
    return Ok((vals[0].parse()?, vals[1].parse()?));
}

fn transform(subject: i64, loop_size: i64) -> i64 {
    let mut val = 1;
    for _ in 0..loop_size {
        val *= subject;
        val %= 20201227;
    }
    return val;
}

fn compute_loop(subject: i64, target: i64) -> i64 {
    let mut val = 1;
    let iterations = 100000000;
    for ls in 0..iterations {
        val *= subject;
        val %= 20201227;

        if val == target {
            return ls + 1;
        }
    }
    panic!("No match found after {}", iterations);
}

fn main() {
    let args : Vec<String> = std::env::args().collect();
    if args[1] == "1" {
        println!("Doing part 1");
        let (door_key, card_key) = read_input(&args[2]).unwrap();
        let door_loop = compute_loop(7, door_key);
        let card_loop = compute_loop(7, card_key);
        println!("door loop: {}, card loop: {}", door_loop, card_loop);
        let door_enc = transform(card_key, door_loop);
        let card_enc = transform(door_key, card_loop);
        assert_eq!(door_enc, card_enc);
        println!("Computed encryption key is {}", door_enc);
    } else {
        println!("Doing part 2");
    }
}
