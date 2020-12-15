#[macro_use] extern crate lazy_static;

use failure::Error;
use regex::Regex;
use std::collections::HashMap;
use std::io::{BufReader,BufRead};
use std::fs::File;

fn parse_int(val: &str) -> Result<i64, Error> {
    return Ok(val.parse::<i64>()?);
}

fn parse_line(line: &str) -> Result<Vec<i64>, Error> {
    lazy_static! {
        static ref COMMA_RE: Regex = Regex::new(r", *").unwrap();
    }
    return COMMA_RE.split(line).map(|v| parse_int(v)).collect::<Result<Vec<i64>, Error>>();
}

fn read_input(file: &str) -> Result<Vec<i64>, Error> {
    let f = File::open(file)?;
    let br = BufReader::new(f);
    let parsed : Result<Vec<Vec<i64>>, Error> = br.lines().map(|line| parse_line(&line?)).collect();
    return Ok(parsed?.into_iter().flatten().collect());
}

fn play_game(values: &Vec<i64>, turns: i64, verbose: bool) -> i64 {
    let mut said : HashMap<i64, i64> = HashMap::new();
    let mut last : i64 = 0;
    let mut t : i64 = 1;
    for val in values {
        said.insert(*val, t);
        if verbose {
            println!("Speak: {}", val);
        }
        t += 1;
    }


    while t < turns {
        if verbose {
            println!("Speak: {}", last);
        }
        last = match said.insert(last, t) {
            Some(lastlast_t) => t - lastlast_t,
            None => 0
        };
        t += 1;
    }

    if verbose {
        println!("Speak: {}", last);
    }
    return last;
}

fn main() {
    let args : Vec<String> = std::env::args().collect();
    if args[1] == "1" {
        println!("Doing part 1");
        let values = read_input(&args[2]).unwrap();
        let last_val = play_game(&values, 2020, true);
        println!("Last value: {}", last_val);
    } else {
        println!("Doing part 2");
        let values = read_input(&args[2]).unwrap();
        let last_val = play_game(&values, 30000000, false);
        println!("Last value: {}", last_val);
    }
}
