#[macro_use] extern crate lazy_static;
#[macro_use] extern crate failure;

use failure::{Error, bail};
use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader,BufRead};

#[derive(PartialEq, Eq, Clone, Debug, Hash)]
struct Rule {
    name: String,
    contents: Vec<(i64, String)>,
}

fn parse_int(val: &str) -> Result<i64, Error> {
    return Ok(val.parse::<i64>()?);
}

fn parse_phrase(val: &str) -> Result<(i64, String), Error> {
    lazy_static! {
        static ref SINGLE_RE: Regex = Regex::new(r"^(\d+) (.*) bags?").unwrap();
    }
    match SINGLE_RE.captures(val) {
        Some(caps) => {
            let num = parse_int(caps.get(1).unwrap().as_str())?;
            let name = caps.get(2).unwrap().as_str().to_string();
            return Ok((num, name));
        },
        None => {
            bail!("Bad phrase: {}", val);
        }
    }
}

fn parse_line(line: &str) -> Result<Rule, Error> {
    lazy_static! {
        // vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
        static ref LINE_RE: Regex = Regex::new(r"^(.*) bags? contain (.*)\.$").unwrap();
        static ref COMMA_RE: Regex = Regex::new(r", *").unwrap();
    }
    match LINE_RE.captures(line) {
        Some(caps) => {
            let name = caps.get(1).unwrap().as_str().to_string();
            let contents_txt = caps.get(2).unwrap().as_str().to_string();
            if contents_txt == "no other bags" {
                return Ok(Rule{ name: name, contents: Vec::new() });
            }
            let contents = COMMA_RE.split(&contents_txt).map(|st| parse_phrase(st)).collect::<Result<Vec<(i64, String)>, Error>>();
            return Ok(Rule{ name: name, contents: contents? });
        }
        None => {
            bail!("Bad line: {}", line);
        }
    }
}

type Rules = HashMap<String, Rule>;

fn read_input(file: &str) -> Result<Rules, Error> {
    let f = File::open(file)?;
    let br = BufReader::new(f);
    let parsed : Vec<Rule> = br.lines().map(|line| parse_line(&line?)).collect::<Result<Vec<Rule>, Error>>()?;
    return Ok(parsed.into_iter().map(|rule| (rule.name.to_string(), rule)).collect());
}

fn can_contain_count(inner: &str, rules: &Rules) -> i64 {
    let mut cache = HashMap::new();
    return rules.iter().filter(|(name, _rule)| {
        *name != inner && can_contain_caching(name, inner, rules, &mut cache)
    }).count() as i64;
}

fn can_contain_caching(outer: &str, inner: &str, rules: &Rules, mut cache: &mut HashMap<String, bool>) -> bool {
    if outer == inner {
        println!("For {} / {}, match", outer, inner);
        return true;
    } else if let Some(val) = cache.get(outer) {
        println!("For {} / {}, cached as {:?}", outer, inner, *val);
        return *val;
    } else {
        let children = &rules.get(outer).unwrap().contents;
        let result = children.iter().any(|c| can_contain_caching(&c.1, inner, rules, &mut cache));
        println!("For {} / {}, recursive as {:?}", outer, inner, result);
        cache.insert(outer.to_string(), result);
        return result;
    }
}

fn num_contained(val: &str, rules: &Rules) -> i64 {
    let mut cache = HashMap::new();
    // recur_count_caching counts the bag itself in the total,
    // but we don't want that here
    return recur_count_caching(val, rules, &mut cache) - 1;
}

fn recur_count_caching(name: &str, rules: &Rules, mut cache: &mut HashMap<String, i64>) -> i64 {
    if let Some(val) = cache.get(name) {
        println!("For {}, cached as {:?}", name, *val);
        return *val;
    } else {
        let children = &rules.get(name).unwrap().contents;
        let result = children.iter().map(|c| recur_count_caching(&c.1, rules, &mut cache) * c.0).sum::<i64>() + 1;
        println!("For {}, recursive as {:?}", name, result);
        cache.insert(name.to_string(), result);
        return result;
    }
}

fn main() {
    let args : Vec<String> = std::env::args().collect();
    if args[1] == "1" {
        println!("Doing part 1");
        let rules = read_input(&args[2]).unwrap();
        let cnt = can_contain_count("shiny gold", &rules);
        println!("Possible containers: {}", cnt);
    } else {
        println!("Doing part 2");
        let rules = read_input(&args[2]).unwrap();
        let cnt = num_contained("shiny gold", &rules);
        println!("Total contained: {}", cnt);
    }
}
