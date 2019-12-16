#[macro_use] extern crate lazy_static;
#[macro_use] extern crate failure;

use failure::Error;
use itertools::Itertools;
use regex::Regex;
use std::cell::RefCell;
use std::collections::{BinaryHeap,HashMap,HashSet};
use std::cmp::Ordering;
use std::io::{BufReader,BufRead};
use std::fs::File;

#[derive(PartialEq, Eq, Debug, Hash)]
struct Quantity {
    chemical: String,
    amount: i64,
}

#[derive(Debug)]
struct Rule {
    inputs: Vec<Quantity>,
    output: Quantity,
}

fn parse_int(val: &str) -> Result<i64, Error> {
    return Ok(val.parse::<i64>()?);
}

fn parse_quantity(val: &str) -> Result<Quantity, Error> {
    lazy_static! {
        static ref QUANTITY_RE: Regex = Regex::new(r"(\d+)\s+(\w+)").unwrap();
    }
    match QUANTITY_RE.captures(val) {
        Some(caps) => {
            return Ok(Quantity {
                chemical: caps.get(2).unwrap().as_str().to_string(),
                amount: parse_int(caps.get(1).unwrap().as_str())?
            });
        }
        None => {
            bail!("Bad quantity: {}", val);
        }
    }
}

fn parse_line(line: &str) -> Result<Rule, Error> {
    lazy_static! {
        static ref LINE_RE: Regex = Regex::new(r"(.*)\s*=>\s*(.*)$").unwrap();
        static ref SEP_RE: Regex = Regex::new(r"\s*,\s*").unwrap();
    }
    match LINE_RE.captures(line) {
        Some(caps) => {
            let inputs : Result<Vec<Quantity>, Error> =
                SEP_RE.split(caps.get(1).unwrap().as_str())
                .map(|val| parse_quantity(&val)).collect();
            let output = parse_quantity(caps.get(2).unwrap().as_str());
            return Ok(Rule { inputs: inputs?, output: output? });
        }
        None => {
            bail!("Bad line: {}", line);
        }
    }
}

fn read_input(file: &str) -> Result<Vec<Rule>, Error> {
    let f = File::open(file)?;
    let br = BufReader::new(f);
    return br.lines().map(|line| parse_line(&line?)).collect();
}

fn calc_ore_for_fuel(rules: &Vec<Rule>) -> i64 {
    let by_output : HashMap<String, &Rule> = rules.into_iter().map(|r| { (r.output.chemical.to_string(), r) }).collect();
    let mut extra : HashMap<String, i64> = rules.into_iter().map(|r| { (r.output.chemical.to_string(), 0) }).collect();

    return calc_ore_recurse("FUEL", 1, &by_output, &mut extra);
}

fn calc_fuel_for_ore(rules: &Vec<Rule>, ore_amt: i64) -> i64 {
    let by_output : HashMap<String, &Rule> = rules.into_iter().map(|r| { (r.output.chemical.to_string(), r) }).collect();
    let mut extra : HashMap<String, i64> = rules.into_iter().map(|r| { (r.output.chemical.to_string(), 0) }).collect();

    let times : i64 = 1000;
    let chunk = ore_amt / times as i64;
    let mut chunk_fuel = 0;
    let mut chunk_ore = 0;
    loop {
        let cur_ore = calc_ore_recurse("FUEL", 1, &by_output, &mut extra);
        if chunk_ore + cur_ore <= chunk {
            chunk_fuel += 1;
            chunk_ore += cur_ore;
        } else {
            break;
        }
    }
    let mut est_fuel = chunk_fuel * times;
    loop {
        extra = rules.into_iter().map(|r| { (r.output.chemical.to_string(), 0) }).collect();
        let cur_ore = calc_ore_recurse("FUEL", est_fuel+1, &by_output, &mut extra);
        if cur_ore > ore_amt {
            return est_fuel;
        } else {
            est_fuel += 1;
        }
    }
}

fn calc_ore_recurse(ch: &str, amt: i64, by_output: &HashMap<String, &Rule>, extra: &mut HashMap<String, i64>) -> i64 {
    if ch == "ORE" {
        return amt;
    }
    let mut actual_amt = amt;
    match extra.get_mut(ch) {
        Some(ea) => {
            if actual_amt > *ea {
                // let reduce = *ea;
                actual_amt -= *ea;
                *ea = 0;
                // if reduce > 0 {
                //     println!("Amt reduced by {}", reduce);
                // }
            } else {
                *ea -= actual_amt;
                return 0;
            }
        },
        None => {
            panic!("No extra found for {}", ch);
        }
    }
    let mut tot = 0;
    match by_output.get(ch) {
        Some(rule) => {
            let mult = (actual_amt as f64 / rule.output.amount as f64).ceil() as i64;
            let rem = (mult * rule.output.amount) - actual_amt;
            let new_extra = extra.get(&rule.output.chemical).unwrap_or(&0) + rem;
            extra.insert(rule.output.chemical.to_string(), new_extra);
            for inp in &rule.inputs {
                tot += calc_ore_recurse(&inp.chemical, inp.amount * mult, &by_output, extra);
            }
            // println!("ch: {}, amt: {}, mult: {}, rem: {} -> {}", ch, actual_amt, mult, rem, tot);
            return tot;
        }
        None => {
            panic!("No recipe found for {}", ch);
        }
    }
}

fn main() {
    let args : Vec<String> = std::env::args().collect();
    if args[1] == "1" {
        println!("Doing part 1");
        let rules = read_input(&args[2]).unwrap();
        let ore_count = calc_ore_for_fuel(&rules);
        println!("Ore to generate a fuel: {}", ore_count);
    } else {
        println!("Doing part 2");
        let rules = read_input(&args[2]).unwrap();
        let fuel_count = calc_fuel_for_ore(&rules, 1000000000000_i64);
        println!("Fuel from a trillion ore: {}", fuel_count);
    }
}
