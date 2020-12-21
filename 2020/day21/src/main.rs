#[macro_use] extern crate lazy_static;
#[macro_use] extern crate failure;

use failure::{Error, bail};
use itertools::Itertools;
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::io::{BufReader,BufRead};
use std::fs::File;

#[derive(Debug)]
struct Food {
    ingredients: HashSet<String>,
    allergens: HashSet<String>,
}

fn parse_line(line: &str) -> Result<Food, Error> {
    lazy_static! {
        static ref LINE_RE: Regex = Regex::new(r"^(.*) \(contains (.*)\)").unwrap();
        static ref SEP_RE: Regex = Regex::new(r"[,\s]+").unwrap();
    }
    match LINE_RE.captures(line) {
        Some(caps) => {
            let food = Food {
                ingredients: SEP_RE.split(caps.get(1).unwrap().as_str()).map(|s| s.to_string()).collect(),
                allergens: SEP_RE.split(caps.get(2).unwrap().as_str()).map(|s| s.to_string()).collect(),
            };
            return Ok(food);
        }
        None => {
            bail!("Bad line: {}", line);
        }
    }
}

fn read_input(file: &str) -> Result<Vec<Food>, Error> {
    let f = File::open(file)?;
    let br = BufReader::new(f);
    let parsed : Result<Vec<Food>, Error> = br.lines().map(|line| parse_line(&line?)).collect();
    return parsed;
}

fn find_known_safe(foods: &Vec<Food>) -> i64 {
    let (all_ingredients, all_contenders) = find_all_contenders(foods);

    let mut known_safe = all_ingredients.clone();
    for contenders in all_contenders.values() {
        known_safe = known_safe.difference(contenders).map(|s| s.clone()).collect();
    }

    return foods.iter().map(|f| f.ingredients.iter().filter(|i| known_safe.contains(*i)).count() as i64).sum::<i64>();
}

fn find_all_contenders(foods: &Vec<Food>) -> (HashSet<String>, HashMap<String, HashSet<String>>) {
    let mut all_ingredients = HashSet::new();
    let mut all_allergens = HashSet::new();
    for food in foods {
        all_ingredients.extend(food.ingredients.clone());
        all_allergens.extend(food.allergens.clone());
    }

    let mut all_contenders : HashMap<String, HashSet<String>> =
        all_allergens.iter().map(|al| (al.clone(), all_ingredients.clone())).collect();
    for food in foods {
        for allergen in &food.allergens {
            let contenders = all_contenders.get_mut(allergen).unwrap();
            *contenders = contenders.intersection(&food.ingredients).map(|s| s.clone()).collect();
        }
    }

    return (all_ingredients, all_contenders);
}

fn find_dangerous(foods: &Vec<Food>) -> HashMap<String, String> {
    let (_, mut all_contenders) = find_all_contenders(foods);

    let mut scrubbed : HashSet<String> = HashSet::new();
    loop {
        let mut scrub_allergens : HashSet<String> = HashSet::new();
        let mut scrub_ingredients  : HashSet<String> = HashSet::new();
        for (allergen, contenders) in &all_contenders {
            if contenders.len() == 1 && !scrubbed.contains(allergen) {
                scrub_allergens.insert(allergen.clone());
                scrub_ingredients.insert(contenders.iter().nth(0).unwrap().to_string());
            }
        }
        if scrub_allergens.len() == 0 {
            break;
        }
        for contenders in all_contenders.values_mut() {
            if contenders.len() == 1 {
                continue;
            }
            for ns in &scrub_ingredients {
                contenders.remove(ns);
            }
        }
        scrubbed.extend(scrub_allergens);
    }
    println!("ALL C: {:?}", all_contenders);
    return all_contenders.iter().map(|(al, con)| (con.iter().nth(0).unwrap().to_string(), al.clone())).collect();
}

fn main() {
    let args : Vec<String> = std::env::args().collect();
    if args[1] == "1" {
        println!("Doing part 1");
        let foods = read_input(&args[2]).unwrap();
        for food in &foods {
            println!("{:?}", food);
        }
        let known_safe = find_known_safe(&foods);
        println!("Known safe: {}", known_safe);
    } else {
        println!("Doing part 2");
        let foods = read_input(&args[2]).unwrap();
        let dangers = find_dangerous(&foods);
        let danger_str =
            dangers.keys().sorted_by_key(|k| dangers.get(*k).unwrap()).join(",");

        println!("Dangerous: {}", danger_str);
    }
}
