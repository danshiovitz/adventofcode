//#[macro_use] extern crate lazy_static;

use itertools::join;
use lazy_static::lazy_static;
use regex::Regex;
use failure::{Error,err_msg};
use std::collections::BTreeSet;
use std::collections::HashMap;
use std::io::{BufReader,BufRead};
use std::fs::File;

#[derive(Debug)]
struct Step {
    name: String,
    dependencies: i32,
    children: Vec<String>,
}

fn read_input(file: &str) -> Result<HashMap<String, Step>, Error> {
    let f = File::open(file)?;
    let br = BufReader::new(f);

    let mut ret : HashMap<String, Step> = HashMap::new();
    for line in br.lines() {
        lazy_static! {
            static ref RE: Regex = Regex::new("Step ([A-Z]+) must be finished before step ([A-Z]+) can begin.").unwrap();
        }
        let line = line?;
        let cap = RE.captures(&line).ok_or(err_msg("Bad line"))?;
        let parent = cap.get(1).map(|ym| ym.as_str()).ok_or(err_msg("Bad line"))?;
        let child = cap.get(2).map(|xm| xm.as_str()).ok_or(err_msg("Bad line"))?;

        let c = ret.get_mut(child);
        if c.is_some() {
            let cref = c.unwrap();
            cref.dependencies += 1;
        } else {
            let step = Step { name: child.to_string(), dependencies: 1, children: vec![] };
            ret.insert(step.name.clone(), step);
        }

        let p = ret.get_mut(parent);
        if p.is_some() {
            p.unwrap().children.push(child.to_string());
        } else {
            let step = Step { name: parent.to_string(), dependencies: 0, children: vec![child.to_string()] };
            ret.insert(step.name.clone(), step);
        }
    }

    return Ok(ret);
}

fn complete_time(step: &str, base_time: i32) -> i32 {
    return ((step.chars().next().unwrap() as i32) - ('@' as i32)) + base_time;
}

struct Worker {
    busy: bool,
    step: String,
    until: i32,
}

fn calc_dep_order(steps: &HashMap<String, Step>, num_workers: i32, base_time: i32) -> (String, i32) {
    let mut ret = vec![];
    let mut satisfied : HashMap<String, i32> = HashMap::new();
    let mut to_check : BTreeSet<&String> = steps.keys().collect();
    let mut workers = vec![];
    for _ in 0..num_workers {
        workers.push(Worker { busy: false, step: "".to_string(), until: 0 });
    }
    let mut now : i32 = 0;
    let end_of_time = steps.len() as i32 * 90;
    while to_check.len() > 0 {
        let mut next_now = end_of_time;
        for w in &mut workers {
            if w.busy {
                if w.until <= now {
                    // worker is finished!
                    let step = steps.get(&w.step).unwrap();
                    for ch in &step.children {
                        if let Some(v) = satisfied.get_mut(ch) {
                            *v += 1;
                        } else {
                            satisfied.insert(ch.to_string(), 1);
                        }
                    }
                    ret.push(w.step.clone());
                    w.busy = false;
                    w.step = "".to_string();
                    w.until = 0;
                    next_now = now;
                } else {
                    if w.until < next_now {
                        next_now = w.until;
                    }
                }
            } else {
                // find this worker some work
                let mut found : Option<String> = None;
                for k in &to_check {
                    if let Some(step) = steps.get(*k) {
                        if *satisfied.get(*k).unwrap_or(&0) == step.dependencies {
                            found = Some(k.to_string());
                            break;
                        }
                    }
                }
                if let Some(s) = found {
                    to_check.remove(&s);
                    w.busy = true;
                    w.until = now + complete_time(&s, base_time);
                    w.step = s;
                    next_now = now;
                }
                // else no work for this worker
            }
        }

        now = next_now;
    }
    workers.sort_by_key(|w| w.until);
    for w in workers {
        if w.busy && w.until > now {
            now = w.until;
            ret.push(w.step.clone());
        }
    }
    return (join(ret.iter(), ""), now);
}

fn main() {
    let args : Vec<String> = std::env::args().collect();
    let steps = read_input(&args[1]).unwrap();
    let workers = if &args[1] == "input.txt" { 5 } else { 2 };
    let base_time = if &args[1] == "input.txt" { 60 } else { 0 };
    let (order, time) = calc_dep_order(&steps, workers, base_time);
    println!("Calculated order = {}, time = {}", order, time);
}
