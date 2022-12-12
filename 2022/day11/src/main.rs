use lazy_regex::{regex, Captures, Lazy, Regex};
use std::collections::HashMap;

extern crate common;

use common::framework::{
    parse_records, parse_regexp, parse_regexps, parse_vals, run_day, BaseDay, InputReader,
};

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
            let mut parsers: Vec<(&Lazy<Regex>, Box<dyn FnMut(Captures) -> Op>)> = vec![
                (
                    regex!(r#"old\s*\+\s*(\d+)"#),
                    Box::new(move |c: Captures| Op::Add(c[1].parse::<i64>().unwrap())),
                ),
                (
                    regex!(r#"old\s*\*\s*(\d+)"#),
                    Box::new(move |c: Captures| Op::Mult(c[1].parse::<i64>().unwrap())),
                ),
                (
                    regex!(r#"old\s*\*\s*old"#),
                    Box::new(move |_c: Captures| Op::Square()),
                ),
            ];
            return parse_regexps(exp, &mut parsers);
        }

        fn parse_monkey(lines: &Vec<String>) -> Monkey {
            Monkey {
                items: parse_regexp(
                    &lines[1],
                    regex!(r#"\s*Starting items:\s*(.*)"#),
                    &mut |c: Captures| parse_vals::<i64>(&c[1]),
                ),
                op: parse_regexp(
                    &lines[2],
                    regex!(r#"\s*Operation:\s*new\s*=\s*(.*)"#),
                    &mut |c: Captures| parse_exp(&c[1]),
                ),
                test_by: parse_regexp(
                    &lines[3],
                    regex!(r#"\s*Test:\s+divisible\s+by\s+(\d+)"#),
                    &mut |c: Captures| c[1].parse::<i64>().unwrap(),
                ),
                test_to: (
                    parse_regexp(
                        &lines[4],
                        regex!(r#"\s*If\s+true:\s+throw\s+to\s+monkey\s+(\d+)"#),
                        &mut |c: Captures| c[1].parse::<usize>().unwrap(),
                    ),
                    parse_regexp(
                        &lines[5],
                        regex!(r#"\s*If\s+false:\s+throw\s+to\s+monkey\s+(\d+)"#),
                        &mut |c: Captures| c[1].parse::<usize>().unwrap(),
                    ),
                ),
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
