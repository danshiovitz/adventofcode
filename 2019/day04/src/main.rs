#[macro_use] extern crate failure;
#[macro_use] extern crate lazy_static;

use failure::{Error, err_msg};
use itertools::Itertools;
use regex::Regex;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::io::{BufReader,BufRead};
use std::fs::File;

fn parse_int(val: &str) -> Result<i32, Error> {
    return Ok(val.parse::<i32>()?);
}

struct Range {
    start: Vec<char>,
    finish: Vec<char>,
}

fn read_input(val: &str) -> Result<Range, Error> {
    lazy_static! {
        static ref SINGLE_RE: Regex = Regex::new(r"^\d+$").unwrap();
        static ref RANGE_RE: Regex = Regex::new(r"^(\d+)\s*-\s*(\d+)$").unwrap();
    }
    if SINGLE_RE.is_match(val) {
        return Ok(Range { start: val.chars().collect(), finish: val.chars().collect() });
    }
    match RANGE_RE.captures(val) {
        Some(caps) => {
            return Ok(Range { start: caps.get(1).unwrap().as_str().chars().collect(),
                              finish: caps.get(2).unwrap().as_str().chars().collect() });
        }
        None => {
            let f = File::open(val)?;
            let br = BufReader::new(f);
            let line = br.lines().next().unwrap()?;
            return read_input(&line);
        }
    }
}

fn check_pw(pw: &Vec<char>, pt1: bool) -> bool {
    if pw.len() != 6 {
        panic!("Bad pw: {:?}", pw);
    }
    let mut same = false;
    let mut i = 0;
    loop {
        if i >= pw.len() - 1 {
            return same;
        }
        else if pw[i] > pw[i+1] {
            return false;
        } else if pw[i] == pw[i+1] {
            if pt1 || i == pw.len() - 2 || pw[i] != pw[i+2] {
                same = true;
            }
            let orig = pw[i];
            while i < pw.len() - 1 && pw[i+1] == orig {
                i += 1;
            }
        } else {
            i += 1;
        }
    }
}

fn inc_pw(pw: &mut Vec<char>) {
    let mut i = pw.len() - 1;
    loop {
        if pw[i] == '9' {
            pw[i] = '0';
            i -= 1;
        } else {
            pw[i] = (pw[i] as u8 + 1) as char;
            return;
        }
    }
}

fn check_range(range: Range, pt1: bool) -> Vec<String> {
    let mut ret = Vec::new();
    let mut cur = range.start.to_vec();
    loop {
        if check_pw(&cur, pt1) {
            // sanity:
            let ss : Vec<char> = cur.to_vec().into_iter().sorted().collect();
            assert!(ss == cur);
            let mut found = false;
            for i in 0..cur.len() - 1 {
                if cur[i] == cur[i+1] && (i == cur.len() - 2 || cur[i+1] != cur[i+2]) {
                    found = true;
                }
            }
            assert!(found, "Oops: {:?} {:?} {:?} {:?} {:?}", ss, cur[4], cur[5], 4 == cur.len() - 2, 0);
            ret.push(cur.to_vec().into_iter().collect());
        }
        if cur == range.finish {
            break;
        }
        inc_pw(&mut cur);
    }
    return ret;
}

fn main() {
    let args : Vec<String> = std::env::args().collect();
    if args[1] == "1" {
        println!("Doing part 1");
        let range = read_input(&args[2]).unwrap();
        let pws = check_range(range, true);
        println!("Found: {} = {}", pws.len(), pws.into_iter().take(20).format(", "));
    } else {
        println!("Doing part 2");
        let range = read_input(&args[2]).unwrap();
        let pws = check_range(range, false);
        println!("Found: {} = {}", pws.len(), pws.into_iter().take(20).format(", "));
    }
}
