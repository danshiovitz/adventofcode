#[macro_use] extern crate failure;
#[macro_use] extern crate lazy_static;

use failure::{Error, bail};
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::io::{BufReader, BufRead};
use std::fs::File;

#[derive(Debug)]
struct Passport {
    fields: HashMap<String, String>,
}

fn parse_passport(line: &str) -> Result<Passport, Error> {
    lazy_static! {
        static ref WS_RE: Regex = Regex::new(r"\s+").unwrap();
        static ref KV_RE: Regex = Regex::new(r"(\S+):(\S+)").unwrap();
    }
    let mut passport = Passport { fields: HashMap::new() };
    for kv in WS_RE.split(line) {
        if kv.len() == 0 {
            continue;
        }
        match KV_RE.captures(kv) {
            Some(caps) => {
                passport.fields.insert(caps.get(1).unwrap().as_str().to_string(), caps.get(2).unwrap().as_str().to_string());
            },
            None => {
                bail!("Bad line: {}", line);
            }
        }
    }
    return Ok(passport);
}

fn read_input(file: &str) -> Result<Vec<Passport>, Error> {
    let f = File::open(file)?;
    let br = BufReader::new(f);
    let mut passports = Vec::new();
    let mut buf = String::new();
    for line in br.lines() {
        let line = line?;
        if line.len() == 0 {
            if buf.len() > 0 {
                passports.push(parse_passport(&buf)?);
                buf.clear();
            }
        } else {
            buf.push(' ');
            buf.push_str(&line);
        }
    }
    if buf.len() > 0 {
        passports.push(parse_passport(&buf)?);
        buf.clear();
    }
    return Ok(passports);
}

type Validator = fn(Option<&String>) -> bool;

fn make_validators1() -> Vec<(String, Validator)> {
    return vec![
        ("byr".to_string(), |v| v.is_some()),
        ("iyr".to_string(), |v| v.is_some()),
        ("eyr".to_string(), |v| v.is_some()),
        ("hgt".to_string(), |v| v.is_some()),
        ("hcl".to_string(), |v| v.is_some()),
        ("ecl".to_string(), |v| v.is_some()),
        ("pid".to_string(), |v| v.is_some()),
        ("cid".to_string(), |_v| true),
    ];
}

fn make_validators2() -> Vec<(String, Validator)> {
    lazy_static! {
        static ref YEAR_RE: Regex = Regex::new(r"^([0-9]{4})$").unwrap();
        static ref COLOR_RE: Regex = Regex::new(r"^#[a-f0-9]{6}$").unwrap();
        static ref HEIGHT_IN_RE: Regex = Regex::new(r"^([0-9]+)in$").unwrap();
        static ref HEIGHT_CM_RE: Regex = Regex::new(r"^([0-9]+)cm$").unwrap();
        static ref ECL_RE: Regex = Regex::new(r"amb|blu|brn|gry|grn|hzl|oth$").unwrap();
        static ref PID_RE: Regex = Regex::new(r"^[0-9]{9}$").unwrap();
    };

    return vec![
        ("byr".to_string(), |v| matches_rex_and_range(&YEAR_RE, 1920, 2002, v)),
        ("iyr".to_string(), |v| matches_rex_and_range(&YEAR_RE, 2010, 2020, v)),
        ("eyr".to_string(), |v| matches_rex_and_range(&YEAR_RE, 2020, 2030, v)),
        ("hgt".to_string(), |v| matches_rex_and_range(&HEIGHT_IN_RE, 59, 76, v) ||
                                matches_rex_and_range(&HEIGHT_CM_RE, 150, 193, v)),
        ("hcl".to_string(), |v| matches_rex(&COLOR_RE, v)),
        ("ecl".to_string(), |v| matches_rex(&ECL_RE, v)),
        ("pid".to_string(), |v| matches_rex(&PID_RE, v)),
        ("cid".to_string(), |_v| true),
    ];
}

fn matches_rex(rex: &Regex, val: Option<&String>) -> bool {
    return match val {
        Some(v) => rex.is_match(v),
        None => false,
    };
}

fn matches_rex_and_range(rex: &Regex, min: i32, max: i32, val: Option<&String>) -> bool {
    return match val {
        Some(v) => {
            match rex.captures(v) {
                Some(caps) => {
                    match caps.get(1).unwrap().as_str().parse::<i32>() {
                        Ok(iv) => iv >= min && iv <= max,
                        Err(_) => false,
                    }
                },
                None => false,
            }
        },
        None => false,
    };
}

fn is_valid(passport: &Passport, validators: &Vec<(String, Validator)>) -> bool {
    let mut fields : HashSet<String> = passport.fields.keys().cloned().collect();
    for (f, validator) in validators.iter() {
        if !validator(passport.fields.get(f)) {
            println!("Failed validation on field {}", f);
            return false;
        }
        fields.remove(f);
    }
    if fields.len() > 0 {
        println!("Failed validation on field {:?}", fields);
        return false;
    }
    return true;
}

fn main() {
    let args : Vec<String> = std::env::args().collect();
    if args[1] == "1" {
        println!("Doing part 1");
        let passports = read_input(&args[2]).unwrap();
        let validators = make_validators1();
        let valid_count = passports.iter().filter(|p| is_valid(p, &validators)).count();
        println!("Number valid: {}", valid_count);
    } else {
        println!("Doing part 2");
        let passports = read_input(&args[2]).unwrap();
        let validators = make_validators2();
        let valid_count = passports.iter().filter(|p| is_valid(p, &validators)).count();
        println!("Number valid: {}", valid_count);
    }
}
