#[macro_use] extern crate failure;

use failure::Error;
use std::collections::HashSet;
use std::io::{BufReader, BufRead};
use std::iter::FromIterator;
use std::fs::File;

#[derive(Debug)]
struct Group {
    answers: Vec<HashSet<char>>,
}

fn parse_group(lines: &Vec<String>) -> Result<Group, Error> {
    let answers : Vec<HashSet<char>> = lines.iter().map(|line| HashSet::from_iter(line.chars())).collect();
    return Ok(Group { answers: answers });
}

fn read_input(file: &str) -> Result<Vec<Group>, Error> {
    let f = File::open(file)?;
    let br = BufReader::new(f);
    let mut groups = Vec::new();
    let mut buf : Vec<String> = Vec::new();
    for line in br.lines() {
        let line = line?;
        if line.len() == 0 {
            if buf.len() > 0 {
                groups.push(parse_group(&buf)?);
                buf.clear();
            }
        } else {
            buf.push(line);
        }
    }
    if buf.len() > 0 {
        groups.push(parse_group(&buf)?);
        buf.clear();
    }
    return Ok(groups);
}

fn count_combined1(grp: &Group) -> i32 {
    if grp.answers.len() == 0 {
        return 0;
    }
    return grp.answers.iter().fold(HashSet::new(), |acc, x| acc.union(&x).cloned().collect()).len() as i32;
}

fn count_combined2(grp: &Group) -> i32 {
    if grp.answers.len() == 0 {
        return 0;
    }
    return grp.answers.iter().fold(grp.answers[0].iter().cloned().collect::<HashSet<char>>(), |acc, x| acc.intersection(&x).cloned().collect()).len() as i32;
}

fn main() {
    let args : Vec<String> = std::env::args().collect();
    if args[1] == "1" {
        println!("Doing part 1");
        let groups = read_input(&args[2]).unwrap();
        let combined_count : i32 = groups.iter().map(|g| count_combined1(&g)).sum();
        println!("Combined count: {}", combined_count);
    } else {
        println!("Doing part 2");
        let groups = read_input(&args[2]).unwrap();
        let combined_count : i32 = groups.iter().map(|g| count_combined2(&g)).sum();
        println!("Combined count: {}", combined_count);
    }
}
