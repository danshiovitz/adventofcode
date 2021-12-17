use std::collections::HashMap;

extern crate common;

use common::framework::{parse_lines, run_day, BaseDay, InputReader};

struct Day10 {
    vals: Vec<String>,
}

fn check_line(line: &str) -> (Option<char>, Vec<char>) {
    let pairs = vec![('(', ')'), ('[', ']'), ('{', '}'), ('<', '>')];

    let mut stack = Vec::new();
    for c in line.chars() {
        if pairs.iter().any(|p| p.0 == c) {
            stack.push(c);
        } else {
            let o = pairs
                .iter()
                .filter(|p| p.1 == c)
                .map(|p| p.0)
                .next()
                .unwrap();
            if stack.len() > 0 && stack[stack.len() - 1] == o {
                stack.pop();
            } else {
                return (Some(c), stack);
            }
        }
    }
    return (None, stack);
}

fn score_corrupt_lines(line: &str) -> i64 {
    let scores = HashMap::from([(')', 3), (']', 57), ('}', 1197), ('>', 25137)]);

    let (bad_char, _remaining) = check_line(line);
    return match bad_char {
        Some(c) => *scores.get(&c).unwrap(),
        None => 0,
    };
}

fn score_incomplete_lines(line: &str) -> i64 {
    let scores = HashMap::from([('(', 1), ('[', 2), ('{', 3), ('<', 4)]);

    let (bad_char, mut remaining) = check_line(line);
    if bad_char.is_some() {
        return 0;
    }
    remaining.reverse();
    return remaining
        .into_iter()
        .fold(0, |tot, c| 5 * tot + *scores.get(&c).unwrap());
}

impl BaseDay for Day10 {
    fn parse(&mut self, input: &mut InputReader) {
        self.vals = parse_lines(input, &mut |line: String| line);
    }

    fn pt1(&mut self) -> String {
        let tot: i64 = self.vals.iter().map(|ln| score_corrupt_lines(ln)).sum();
        return tot.to_string();
    }

    fn pt2(&mut self) -> String {
        let mut scores: Vec<i64> = self
            .vals
            .iter()
            .map(|ln| score_incomplete_lines(ln))
            .filter(|s| *s != 0)
            .collect();
        println!("scores: {:?}", scores);
        scores.sort();
        let tot = scores[scores.len() / 2];
        return tot.to_string();
    }
}

fn main() {
    let mut day = Day10 { vals: Vec::new() };
    run_day(&mut day);
}
