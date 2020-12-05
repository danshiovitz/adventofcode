#[macro_use] extern crate lazy_static;
#[macro_use] extern crate failure;

use failure::{Error, bail};
use regex::Regex;
use std::io::{BufReader,BufRead};
use std::fs::File;

#[derive(PartialEq, Eq, Clone, Debug, Hash)]
struct Password {
    range: (i64, i64),
    ch: char,
    password: String,
}

fn parse_int(val: &str) -> Result<i64, Error> {
    return Ok(val.parse::<i64>()?);
}

fn parse_line(line: &str) -> Result<Password, Error> {
    lazy_static! {
        static ref LINE_RE: Regex = Regex::new(r"(\d+)-(\d+)\s+(\w):\s+(\w+)").unwrap();
    }
    match LINE_RE.captures(line) {
        Some(caps) => {
            return Ok(Password {
                range: (parse_int(caps.get(1).unwrap().as_str())?, parse_int(caps.get(2).unwrap().as_str())?),
                ch: caps.get(3).unwrap().as_str().chars().nth(0).unwrap(),
                password: caps.get(4).unwrap().as_str().to_string(),
            });
        }
        None => {
            bail!("Bad line: {}", line);
        }
    }
}

fn read_input(file: &str) -> Result<Vec<Password>, Error> {
    let f = File::open(file)?;
    let br = BufReader::new(f);
    let parsed : Result<Vec<Password>, Error> = br.lines().map(|line| parse_line(&line?)).collect();
    return parsed;
}

fn passes_rule1(pw: &Password) -> bool {
    let cnt = pw.password.chars().filter(|&c| c == pw.ch).count() as i64;
    return cnt >= pw.range.0 && cnt <= pw.range.1;
}

fn passes_rule2(pw: &Password) -> bool {
    let chs : Vec<char> = pw.password.chars().collect();
    let r1 = (pw.range.0 - 1) as usize;
    let r2 = (pw.range.1 - 1) as usize;
    let m1 = chs.len() > r1 && chs[r1] == pw.ch;
    let m2 = chs.len() > r2 && chs[r2] == pw.ch;
    return m1 != m2;
}

fn main() {
    let args : Vec<String> = std::env::args().collect();
    if args[1] == "1" {
        println!("Doing part 1");
        let vals = read_input(&args[2]).unwrap();
        let match_cnt = vals.iter().filter(|p| passes_rule1(p)).count();
        println!("Matching: {}", match_cnt);
    } else {
        println!("Doing part 2");
        let vals = read_input(&args[2]).unwrap();
        let match_cnt = vals.iter().filter(|p| passes_rule2(p)).count();
        println!("Matching: {}", match_cnt);
    }
}
