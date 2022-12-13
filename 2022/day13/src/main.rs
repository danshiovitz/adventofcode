use std::boxed::Box;
use std::cmp::Ordering;

extern crate common;

use common::framework::{parse_records, run_day, BaseDay, InputReader};

#[derive(Debug, Clone, Eq)]
enum Value {
    Atom(i32),
    List(Box<Vec<Value>>),
}

impl Ord for Value {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Value::Atom(x), Value::Atom(y)) => x.cmp(y),
            (Value::Atom(_x), Value::List(_y)) => {
                Value::List(Box::new(vec![self.clone()])).cmp(other)
            }
            (Value::List(_x), Value::Atom(_y)) => {
                self.cmp(&Value::List(Box::new(vec![other.clone()])))
            }
            (Value::List(x), Value::List(y)) => {
                for i in 0..(x.len() + y.len()) {
                    if i >= y.len() {
                        if i >= x.len() {
                            return Ordering::Equal;
                        } else {
                            return Ordering::Greater;
                        }
                    } else if i >= x.len() {
                        return Ordering::Less;
                    } else {
                        let c = x[i].cmp(&y[i]);
                        if c != Ordering::Equal {
                            return c;
                        }
                    }
                }
                return Ordering::Equal;
            }
        }
    }
}

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Value::Atom(x), Value::Atom(y)) => x == y,
            (Value::Atom(_x), Value::List(_y)) => false,
            (Value::List(_x), Value::Atom(_y)) => false,
            (Value::List(x), Value::List(y)) => {
                if x.len() != y.len() {
                    return false;
                }
                for i in 0..x.len() {
                    if x[i] != y[i] {
                        return false;
                    }
                }
                return true;
            }
        }
    }
}

struct Day13 {
    vals: Vec<(Value, Value)>,
}

fn parse_one(chs: &mut Vec<char>) -> Value {
    if chs.len() == 0 {
        panic!("Bad value (empty) [{} remaining]", chs.len());
    }

    if chs[0].is_digit(10) {
        let mut tot = 0;
        while chs.len() > 0 && chs[0].is_digit(10) {
            let nxt = chs.remove(0) as i32 - '0' as i32;
            tot *= 10;
            tot += nxt;
        }
        return Value::Atom(tot);
    }

    if chs[0] == '[' {
        chs.remove(0);
        let mut values = Vec::new();
        let mut first = true;
        loop {
            if chs.len() > 0 && chs[0] == ']' {
                chs.remove(0);
                return Value::List(Box::new(values));
            }

            if first {
                first = false;
            } else if chs.len() > 0 && chs[0] == ',' {
                chs.remove(0);
            } else {
                panic!("Bad value (no comma) [{} remaining]", chs.len());
            }

            values.push(parse_one(chs));
        }
    }

    panic!(
        "Bad value (unexpected: {}) [{} remaining]",
        chs[0],
        chs.len()
    );
}

impl BaseDay for Day13 {
    fn parse(&mut self, input: &mut InputReader) {
        fn parse_record(lines: &Vec<String>) -> (Value, Value) {
            return (
                parse_one(&mut lines[0].chars().collect::<Vec<char>>()),
                parse_one(&mut lines[1].chars().collect::<Vec<char>>()),
            );
        }
        self.vals = parse_records(input, &mut parse_record);
    }

    fn pt1(&mut self) -> String {
        let tot: i32 = (0..self.vals.len())
            .filter(|i| (self.vals[*i].0).cmp(&(self.vals[*i].1)) == Ordering::Less)
            .map(|i| i as i32 + 1)
            .sum();
        return tot.to_string();
    }

    fn pt2(&mut self) -> String {
        let mut all_v: Vec<Value> = Vec::new();
        for v in &self.vals {
            all_v.push(v.0.clone());
            all_v.push(v.1.clone());
        }
        let dividers: Vec<Value> = vec![
            Value::List(Box::new(vec![Value::List(Box::new(vec![Value::Atom(2)]))])),
            Value::List(Box::new(vec![Value::List(Box::new(vec![Value::Atom(6)]))])),
        ];
        all_v.extend(dividers.clone());
        all_v.sort();
        let key = (0..all_v.len())
            .filter(|i| dividers.contains(&all_v[*i]))
            .fold(1, |tot, v| tot * (v + 1) as i32);
        return key.to_string();
    }
}

fn main() {
    let mut day = Day13 { vals: Vec::new() };
    run_day(&mut day);
}
