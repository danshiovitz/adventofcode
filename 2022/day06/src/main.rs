use itertools::Itertools;
use std::collections::HashSet;

extern crate common;

use common::framework::{parse_lines, run_day, BaseDay, InputReader};

struct Day06 {
    vals: Vec<Vec<char>>,
}

fn is_marker(i: usize, amt: usize, chs: &Vec<char>) -> bool {
    if i < amt {
        return false;
    }
    let st = &chs[(i - amt + 1)..=i]
        .into_iter()
        .map(|c| *c)
        .collect::<HashSet<char>>();
    return st.len() == amt;
}

impl BaseDay for Day06 {
    fn parse(&mut self, input: &mut InputReader) {
        self.vals = parse_lines(input, &mut |line: String| {
            line.chars().collect::<Vec<char>>()
        });
    }

    fn pt1(&mut self) -> String {
        let amt = 4;
        return self
            .vals
            .iter()
            .map(|v| {
                ((amt - 1)..v.len())
                    .find(|i| is_marker(*i, amt, v))
                    .unwrap_or(9998)
            })
            .map(|i| (i + 1).to_string())
            .intersperse(",".to_string())
            .collect::<String>();
    }

    fn pt2(&mut self) -> String {
        let amt = 14;
        return self
            .vals
            .iter()
            .map(|v| {
                ((amt - 1)..v.len())
                    .find(|i| is_marker(*i, amt, v))
                    .unwrap_or(9998)
            })
            .map(|i| (i + 1).to_string())
            .intersperse(",".to_string())
            .collect::<String>();
    }
}

fn main() {
    let mut day = Day06 { vals: Vec::new() };
    run_day(&mut day);
}
