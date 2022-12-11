use lazy_regex::regex;
use std::collections::HashMap;

extern crate common;

use common::framework::{parse_records, run_day, BaseDay, InputReader};

#[derive(Debug)]
enum Op {
    Add(i64),
    Mult(i64),
    Square(),
}

#[derive(Debug)]
struct Monkey {
    items: Vec<i64>,
    op: Op,
    test_by: i64,
    test_to: (usize, usize),
}

struct Day11 {
    vals: Vec<Monkey>,
}

fn run_rounds(monkeys: &Vec<Monkey>, less_worry: bool, rounds: i32) -> i64 {
    let mut items = monkeys
        .iter()
        .enumerate()
        .map(|(idx, m)| (idx, m.items.clone()))
        .collect::<HashMap<usize, Vec<i64>>>();
    let mut inspects = (0..monkeys.len())
        .map(|idx| (idx, 0))
        .collect::<HashMap<usize, i64>>();

    let allmod = monkeys.iter().fold(1, |sum, m| sum * m.test_by);

    for _round in 1..=rounds {
        for m in 0..monkeys.len() {
            let cur_items = items.get(&m).unwrap().clone();
            items.get_mut(&m).unwrap().clear();
            for item in cur_items.into_iter() {
                let mut new_worry = match monkeys[m].op {
                    Op::Add(val) => item + val,
                    Op::Mult(val) => item * val,
                    Op::Square() => item * item,
                };
                if less_worry {
                    new_worry = new_worry / 3;
                }
                new_worry %= allmod;
                let target = if new_worry % monkeys[m].test_by == 0 {
                    monkeys[m].test_to.0
                } else {
                    monkeys[m].test_to.1
                };
                items.get_mut(&target).unwrap().push(new_worry);
                *inspects.get_mut(&m).unwrap() += 1;
            }
        }
        // println!("items: {:?}", items);
        // println!("inspects: {:?}", inspects);
    }

    let mut inspects: Vec<i64> = inspects
        .values()
        .into_iter()
        .map(|i| *i)
        .collect::<Vec<i64>>();
    inspects.sort();
    inspects.reverse();
    return inspects[0] * inspects[1];
}

impl BaseDay for Day11 {
    fn parse(&mut self, input: &mut InputReader) {
        fn parse_exp(exp: &str) -> Op {
            match regex!(r#"old\s*\+\s*(\d+)"#).captures(&exp) {
                Some(c) => {
                    return Op::Add(c[1].parse::<i64>().unwrap());
                }
                None => {}
            }
            match regex!(r#"old\s*\*\s*(\d+)"#).captures(&exp) {
                Some(c) => {
                    return Op::Mult(c[1].parse::<i64>().unwrap());
                }
                None => {}
            }
            match regex!(r#"old\s*\*\s*old"#).captures(&exp) {
                Some(_) => {
                    return Op::Square();
                }
                None => {}
            }
            panic!("Bad exp: {}", exp);
        }

        fn parse_monkey(lines: &Vec<String>) -> Monkey {
            let mut items = Vec::new();
            match regex!(r#"\s*Starting items:\s*(.*)"#).captures(&lines[1]) {
                Some(c) => {
                    let sep = regex!(r#"\s*,\s*"#);
                    let ws = sep.split(c[1].trim());
                    items.extend(ws.map(|n| n.parse::<i64>().unwrap()));
                }
                None => {
                    panic!("Bad line 1: {}", lines[1]);
                }
            }

            let op: Op;
            match regex!(r#"\s*Operation:\s*new\s*=\s*(.*)"#).captures(&lines[2]) {
                Some(c) => {
                    op = parse_exp(&c[1]);
                }
                None => {
                    panic!("Bad line 2: {}", lines[2]);
                }
            }

            let test_by: i64;
            match regex!(r#"\s*Test:\s+divisible\s+by\s+(\d+)"#).captures(&lines[3]) {
                Some(c) => {
                    test_by = c[1].parse::<i64>().unwrap();
                }
                None => {
                    panic!("Bad line 3: {}", lines[3]);
                }
            }

            let test_true: usize;
            match regex!(r#"\s*If\s+true:\s+throw\s+to\s+monkey\s+(\d+)"#).captures(&lines[4]) {
                Some(c) => {
                    test_true = c[1].parse::<usize>().unwrap();
                }
                None => {
                    panic!("Bad line 4: {}", lines[4]);
                }
            }

            let test_false: usize;
            match regex!(r#"\s*If\s+false:\s+throw\s+to\s+monkey\s+(\d+)"#).captures(&lines[5]) {
                Some(c) => {
                    test_false = c[1].parse::<usize>().unwrap();
                }
                None => {
                    panic!("Bad line 5: {}", lines[5]);
                }
            }

            Monkey {
                items: items,
                op: op,
                test_by: test_by,
                test_to: (test_true, test_false),
            }
        }

        self.vals = parse_records(input, &mut parse_monkey);
    }

    fn pt1(&mut self) -> String {
        let business = run_rounds(&self.vals, true, 20);
        return business.to_string();
    }

    fn pt2(&mut self) -> String {
        let business = run_rounds(&self.vals, false, 10000);
        return business.to_string();
    }
}

fn main() {
    let mut day = Day11 { vals: Vec::new() };
    run_day(&mut day);
}
