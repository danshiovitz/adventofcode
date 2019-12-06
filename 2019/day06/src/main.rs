#[macro_use] extern crate failure;
#[macro_use] extern crate lazy_static;

use failure::Error;
use itertools::Itertools;
use regex::Regex;
use std::collections::HashMap;
use std::io::{BufReader,BufRead};
use std::fs::File;

fn parse_line(line: &str) -> Result<(String, String), Error> {
    let parts : Vec<&str> = line.split(")").collect();
    ensure!(parts.len() == 2);
    return Ok((parts[0].to_string(), parts[1].to_string()));
}

fn read_input(file: &str) -> Result<HashMap<String, Vec<String>>, Error> {
    let f = File::open(file)?;
    let br = BufReader::new(f);
    let parsed : Result<Vec<(String, String)>, Error> = br.lines().map(|line| parse_line(&line?)).collect();
    let mut ret : HashMap<String, Vec<String>> = HashMap::new();
    for (parent, child) in parsed? {
        match ret.get_mut(&parent) {
            Some(children) => { children.push(child); },
            None => { ret.insert(parent, vec![child]); },
        }
    }
    return Ok(ret);
}

fn calc_depths(tree: &HashMap<String, Vec<String>>) -> HashMap<String, i32> {
    let mut ret : HashMap<String, i32> = HashMap::new();
    let mut working = vec![("COM", 0)];
    while working.len() > 0 {
        let (name, depth) = working.pop().unwrap();
        ret.insert(name.to_string(), depth);
        match tree.get(name) {
            Some(children) => {
                for child in children {
                    working.push((child, depth+1));
                }
            },
            None => {}
        }
    }
    return ret;
}

fn flip_tree(tree: &HashMap<String, Vec<String>>) -> HashMap<String, String> {
    let mut ret : HashMap<String, String> = HashMap::new();
    for (parent, children) in tree.iter() {
        for child in children {
            ret.insert(child.to_string(), parent.to_string());
        }
    }
    return ret;
}

fn path_between(a: &str, b: &str, tree: &HashMap<String, String>) -> Vec<String> {
    let path_up = |n: &str, t| -> Vec<String> {
        let mut up = Vec::new();
        let mut cur = n;
        loop {
            match tree.get(cur) {
                Some(parent) => { up.push(parent.to_string()); cur = parent; },
                None => { return up; }
            }
        }
    };

    let mut ap = path_up(&a, &tree);
    let mut bp = path_up(&b, &tree);
    let mut last = None;
    while ap[ap.len() - 1] == bp[bp.len() - 1] {
        last = ap.pop();
        bp.pop();
    }
    bp.reverse();
    ap.push(last.unwrap()); // put lynchpin back in
    ap.append(&mut bp);
    return ap;
}

fn main() {
    let args : Vec<String> = std::env::args().collect();
    if args[1] == "1" {
        println!("Doing part 1");
        let tree = read_input(&args[2]).unwrap();
        let depths = calc_depths(&tree);
        println!("Sum of depths: {}", depths.values().sum::<i32>());
    } else {
        println!("Doing part 2");
        let tree = read_input(&args[2]).unwrap();
        let flipped = flip_tree(&tree);
        let path = path_between("YOU", "SAN", &flipped);
        println!("Transfers required are {}: {}", path.len() - 1, path.into_iter().take(20).format(", "));
    }
}
