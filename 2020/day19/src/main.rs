#[macro_use] extern crate failure;
#[macro_use] extern crate lazy_static;

use failure::{Error, bail};
use regex::Regex;
use std::collections::{HashSet, HashMap};
use std::io::{BufReader, BufRead};
use std::fs::File;

#[derive(Debug)]
enum Subrule {
    Literal(char),
    References(Vec<i64>),
}

#[derive(Debug)]
struct Rule {
    id: i64,
    subrules: Vec<Subrule>,
}

#[derive(Debug)]
struct Input {
    rules: HashMap<i64, Rule>,
    messages: Vec<String>,
}

fn parse_rule(line: &str) -> Result<Rule, Error> {
    lazy_static! {
        static ref RULE_RE: Regex = Regex::new(r"^(\d+)\s*:\s*(.*)$").unwrap();
        static ref PIPE_RE: Regex = Regex::new(r"\s*\|\s*").unwrap();
        static ref LITERAL_RE: Regex = Regex::new("\"(.*?)\"").unwrap();
        static ref SPACE_RE: Regex = Regex::new(r"\s+").unwrap();
    }
    let mut ret = Rule { id: -1, subrules: Vec::new() };
    if let Some(rule_caps) = RULE_RE.captures(line) {
        ret.id = rule_caps.get(1).unwrap().as_str().parse()?;
        let body = rule_caps.get(2).unwrap().as_str();
        for subrule in PIPE_RE.split(body) {
            if let Some(lit_caps) = LITERAL_RE.captures(subrule) {
                let ch = lit_caps.get(1).unwrap().as_str().chars().nth(0).unwrap();
                ret.subrules.push(Subrule::Literal(ch));
            } else {
                let rfs : Vec<i64> = SPACE_RE.split(subrule).map(|r| r.parse::<i64>().unwrap()).collect();
                ret.subrules.push(Subrule::References(rfs));
            }
        }
        return Ok(ret);
    } else {
        bail!("Bad line: {}", line);
    }
}

fn read_input(file: &str) -> Result<Input, Error> {
    let f = File::open(file)?;
    let br = BufReader::new(f);

    let mut ret = Input { rules: HashMap::new(), messages: Vec::new() };

    let mut parse_state = 0;
    for line in br.lines() {
        let line = line?;
        if parse_state == 0 {
            if line.len() > 0 {
                let rule = parse_rule(&line)?;
                ret.rules.insert(rule.id, rule);
            } else {
                parse_state += 1;
            }
        } else if parse_state == 1 {
            ret.messages.push(line);
        }
    }
    if parse_state != 1 {
        bail!("Bad parse at end: {}", parse_state);
    }
    return Ok(ret);
}

fn is_match(message: &str, rules: &HashMap<i64, Rule>) -> bool {
    let chs = message.chars().collect();
    return recur_match(&chs, 0, 0, rules).contains(&chs.len());
}

fn recur_match(message: &Vec<char>, idx: usize, rule: i64, rules: &HashMap<i64, Rule>) -> HashSet<usize> {
    let mut ret = HashSet::new();
    for subrule in &rules.get(&rule).unwrap().subrules {
        match subrule {
            Subrule::Literal(ch) => {
                if message[idx] == *ch {
                    ret.insert(idx + 1);
                }
            },
            Subrule::References(rfs) => {
                let mut cur = HashSet::new();
                cur.insert(idx);
                for (rf_pos, rf) in rfs.iter().enumerate() {
                    let mut nxt : HashSet<usize> = HashSet::new();
                    for p in cur {
                        nxt.extend(recur_match(message, p, *rf, rules));
                    }
                    // each succeeding rule has to generate at least one char
                    let max_idx = message.len() - (rfs.len() - rf_pos) + 1;
                    cur = nxt.iter().filter(|p| *p <= &max_idx).cloned().collect();
                }
                ret.extend(cur);
            }
        }
    }
    return ret;
}

fn main() {
    let args : Vec<String> = std::env::args().collect();
    if args[1] == "1" {
        println!("Doing part 1");
        let input = read_input(&args[2]).unwrap();
        let verbose = args.len() > 3 && args[3].chars().nth(0).unwrap() == 't';
        if verbose {
            for msg in &input.messages {
                println!("{}: {}", msg, is_match(msg.as_str(), &input.rules));
            }
        }
        let matching = input.messages.iter().filter(|msg| is_match(msg.as_str(), &input.rules)).count();
        println!("Matching: {}", matching);
    } else {
        println!("Doing part 2");
        let mut input = read_input(&args[2]).unwrap();
        input.rules.insert(8, Rule { id: 8, subrules: vec![
            Subrule::References(vec![42]),
            Subrule::References(vec![42, 8]),
        ] });
        input.rules.insert(11, Rule { id: 11, subrules: vec![
            Subrule::References(vec![42, 31]),
            Subrule::References(vec![42, 11, 31]),
        ] });

        let verbose = args.len() > 3 && args[3].chars().nth(0).unwrap() == 't';
        if verbose {
            for msg in &input.messages {
                println!("{}: {}", msg, is_match(msg.as_str(), &input.rules));
            }
        }
        let matching = input.messages.iter().filter(|msg| is_match(msg.as_str(), &input.rules)).count();
        println!("Matching: {}", matching);
    }
}
