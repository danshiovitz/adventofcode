use lazy_regex::{regex, Captures, Lazy, Regex};
use std::collections::HashMap;

extern crate common;

use common::framework::{parse_lines, parse_regexp, parse_regexps, run_day, BaseDay, InputReader};

enum OpType {
    Add,
    Subtract,
    Multiply,
    Divide,
}

enum Atom {
    Op(OpType, String, String),
    Value(i64),
}

struct Day21 {
    vals: HashMap<String, Atom>,
}

const HUMAN: &str = "humn";

fn eval_monkey(name: &str, vals: &HashMap<String, Atom>) -> i64 {
    let mut cache = HashMap::new();
    return eval_monkey_cached(name, vals, &mut cache);
}

fn eval_monkey_for_human(name: &str, vals: &HashMap<String, Atom>) -> i64 {
    let mut cache = HashMap::new();
    eval_monkey_cached(name, vals, &mut cache);
    clean_human(vals, &mut cache);

    if let Some(Atom::Op(_, left, right)) = vals.get(name) {
        if let Some(left_val) = cache.get(left) {
            return pushdown_monkey(right, vals, &cache, *left_val);
        } else if let Some(right_val) = cache.get(right) {
            return pushdown_monkey(left, vals, &cache, *right_val);
        }
    }
    panic!("Unexpected data for root");
}

fn eval_monkey_cached(
    name: &str,
    vals: &HashMap<String, Atom>,
    cache: &mut HashMap<String, i64>,
) -> i64 {
    if let Some(result) = cache.get(name) {
        return *result;
    }

    let result = match vals.get(name).unwrap() {
        Atom::Op(t, x, y) => {
            let xv = eval_monkey_cached(x, vals, cache);
            let yv = eval_monkey_cached(y, vals, cache);
            match t {
                OpType::Add => xv + yv,
                OpType::Subtract => xv - yv,
                OpType::Multiply => xv * yv,
                OpType::Divide => xv / yv,
            }
        }
        Atom::Value(x) => *x,
    };
    cache.insert(name.to_string(), result);
    return result;
}

fn clean_human(vals: &HashMap<String, Atom>, cache: &mut HashMap<String, i64>) {
    fn get_parent<'a>(name: &str, vals: &'a HashMap<String, Atom>) -> &'a str {
        for (cur, op) in vals.iter() {
            let matches = match op {
                Atom::Op(_, x, y) => x == name || y == name,
                Atom::Value(_) => false,
            };
            if matches {
                return &*cur;
            }
        }
        panic!("Can't find parent for {}", name);
    }

    let mut cur = HUMAN;
    cache.remove(cur);
    while cur != "root" {
        let parent = &get_parent(cur, vals);
        cache.remove(cur);
        cur = parent;
    }
}

fn pushdown_monkey(
    name: &str,
    vals: &HashMap<String, Atom>,
    cache: &HashMap<String, i64>,
    target: i64,
) -> i64 {
    match vals.get(name).unwrap() {
        Atom::Op(t, x, y) => {
            let xvo = cache.get(x);
            let yvo = cache.get(y);
            match t {
                /*
                x + y = t
                x = t - y
                y = t - x
                */
                OpType::Add => {
                    if let Some(xv) = xvo {
                        return pushdown_monkey(y, vals, cache, target - xv);
                    } else {
                        return pushdown_monkey(x, vals, cache, target - yvo.unwrap());
                    }
                }
                /*
                x - y = t
                x = t + y
                y = x - t
                */
                OpType::Subtract => {
                    if let Some(xv) = xvo {
                        return pushdown_monkey(y, vals, cache, xv - target);
                    } else {
                        return pushdown_monkey(x, vals, cache, target + yvo.unwrap());
                    }
                }
                /*
                x * y = t
                x = t / y
                y = t / x
                */
                OpType::Multiply => {
                    if let Some(xv) = xvo {
                        return pushdown_monkey(y, vals, cache, target / xv);
                    } else {
                        return pushdown_monkey(x, vals, cache, target / yvo.unwrap());
                    }
                }
                /*
                x / y = t
                x = t * y
                y = x / y
                */
                OpType::Divide => {
                    if let Some(xv) = xvo {
                        return pushdown_monkey(y, vals, cache, xv / target);
                    } else {
                        return pushdown_monkey(x, vals, cache, target * yvo.unwrap());
                    }
                }
            }
        }
        Atom::Value(_) => {
            if name == HUMAN {
                return target;
            } else {
                return *cache.get(name).unwrap();
            }
        }
    };
}

impl BaseDay for Day21 {
    fn parse(&mut self, input: &mut InputReader) {
        fn parse_op(line: &str) -> Atom {
            let mut parsers: Vec<(&Lazy<Regex>, Box<dyn FnMut(Captures) -> Atom>)> = vec![
                (
                    regex!(r#"(-?\d+)"#),
                    Box::new(move |c: Captures| Atom::Value(c[1].parse::<i64>().unwrap())),
                ),
                (
                    regex!(r#"(\S+)\s*\+\s*(\S+)"#),
                    Box::new(move |c: Captures| {
                        Atom::Op(OpType::Add, c[1].to_string(), c[2].to_string())
                    }),
                ),
                (
                    regex!(r#"(\S+)\s*\-\s*(\S+)"#),
                    Box::new(move |c: Captures| {
                        Atom::Op(OpType::Subtract, c[1].to_string(), c[2].to_string())
                    }),
                ),
                (
                    regex!(r#"(\S+)\s*\*\s*(\S+)"#),
                    Box::new(move |c: Captures| {
                        Atom::Op(OpType::Multiply, c[1].to_string(), c[2].to_string())
                    }),
                ),
                (
                    regex!(r#"(\S+)\s*/\s*(\S+)"#),
                    Box::new(move |c: Captures| {
                        Atom::Op(OpType::Divide, c[1].to_string(), c[2].to_string())
                    }),
                ),
            ];
            return parse_regexps(line, &mut parsers);
        }
        parse_lines(input, &mut |line: String| {
            parse_regexp(
                &line,
                regex!(r#"\s*(\S+)\s*:\s*(.*)"#),
                &mut |c: Captures| self.vals.insert(c[1].to_string(), parse_op(&c[2])),
            );
        });
    }

    fn pt1(&mut self) -> String {
        let val = eval_monkey("root", &self.vals);
        return val.to_string();
    }

    fn pt2(&mut self) -> String {
        let val = eval_monkey_for_human("root", &self.vals);
        return val.to_string();
    }
}

fn main() {
    let mut day = Day21 { vals: HashMap::new() };
    run_day(&mut day);
}
