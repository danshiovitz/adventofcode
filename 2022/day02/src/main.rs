extern crate common;
#[macro_use]
extern crate lazy_static;

use std::collections::HashMap;

use common::framework::{parse_lines, run_day, BaseDay, InputReader};

struct Day02 {
    vals: Vec<String>,
}

fn score_1(line: &str) -> i32 {
    lazy_static! {
        static ref HASHMAP: HashMap<&'static str, i32> = {
            let mut m = HashMap::new();
            m.insert("A X", 1 + 3);
            m.insert("A Y", 2 + 6);
            m.insert("A Z", 3 + 0);
            m.insert("B X", 1 + 0);
            m.insert("B Y", 2 + 3);
            m.insert("B Z", 3 + 6);
            m.insert("C X", 1 + 6);
            m.insert("C Y", 2 + 0);
            m.insert("C Z", 3 + 3);
            m
        };
    }
    return *HASHMAP.get(line).unwrap();
}

fn score_2(line: &str) -> i32 {
    lazy_static! {
        static ref HASHMAP: HashMap<&'static str, i32> = {
            let mut m = HashMap::new();
            m.insert("A X", 3 + 0);
            m.insert("A Y", 1 + 3);
            m.insert("A Z", 2 + 6);
            m.insert("B X", 1 + 0);
            m.insert("B Y", 2 + 3);
            m.insert("B Z", 3 + 6);
            m.insert("C X", 2 + 0);
            m.insert("C Y", 3 + 3);
            m.insert("C Z", 1 + 6);
            m
        };
    }
    return *HASHMAP.get(line).unwrap();
}

impl BaseDay for Day02 {
    fn parse(&mut self, input: &mut InputReader) {
        self.vals
            .extend(parse_lines(input, &mut |line: String| line))
    }

    fn pt1(&mut self) -> String {
        let tot = self.vals.iter().map(|v| score_1(v)).sum::<i32>();
        return tot.to_string();
    }

    fn pt2(&mut self) -> String {
        let tot = self.vals.iter().map(|v| score_2(v)).sum::<i32>();
        return tot.to_string();
    }
}

fn main() {
    let mut day = Day02 { vals: Vec::new() };
    run_day(&mut day);
}
