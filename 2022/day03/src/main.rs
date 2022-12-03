extern crate common;

use std::collections::HashSet;

use common::framework::{parse_lines, run_day, BaseDay, InputReader};

struct Sack {
    first: HashSet<char>,
    second: HashSet<char>,
}

struct Day03 {
    vals: Vec<Sack>,
}

fn priority(ch: char) -> i32 {
    if ch >= 'a' && ch <= 'z' {
        return (ch as u32 - 'a' as u32) as i32 + 1;
    } else if ch >= 'A' && ch <= 'Z' {
        return (ch as u32 - 'A' as u32) as i32 + 27;
    } else {
        panic!("Unexpected character: {}", ch);
    }
}

fn parse_chunk(chunk: &str) -> HashSet<char> {
    chunk.chars().collect::<HashSet<char>>()
}

impl BaseDay for Day03 {
    fn parse(&mut self, input: &mut InputReader) {
        self.vals.extend(parse_lines(input, &mut |line: String| {
            let idx = line.len() / 2;
            Sack {
                first: parse_chunk(&line[0..idx]),
                second: parse_chunk(&line[idx..idx * 2]),
            }
        }))
    }

    fn pt1(&mut self) -> String {
        fn score(sack: &Sack) -> i32 {
            let ik = sack
                .first
                .intersection(&sack.second)
                .map(|c| *c)
                .collect::<Vec<char>>();
            assert_eq!(ik.len(), 1);
            return priority(ik[0]);
        }

        let tot = self.vals.iter().map(|v| score(v)).sum::<i32>();
        return tot.to_string();
    }

    fn pt2(&mut self) -> String {
        fn score(sack1: &Sack, sack2: &Sack, sack3: &Sack) -> i32 {
            let bt1 = sack1
                .first
                .union(&sack1.second)
                .map(|c| *c)
                .collect::<HashSet<char>>();
            let bt2 = sack2
                .first
                .union(&sack2.second)
                .map(|c| *c)
                .collect::<HashSet<char>>();
            let bt3 = sack3
                .first
                .union(&sack3.second)
                .map(|c| *c)
                .collect::<HashSet<char>>();
            let ik1 = bt1
                .intersection(&bt2)
                .map(|c| *c)
                .collect::<HashSet<char>>();
            let ik2 = ik1.intersection(&bt3).map(|c| *c).collect::<Vec<char>>();
            assert_eq!(ik2.len(), 1);
            return priority(ik2[0]);
        }
        let tot = (0..self.vals.len())
            .step_by(3)
            .map(|i| score(&self.vals[i], &self.vals[i + 1], &self.vals[i + 2]))
            .sum::<i32>();
        return tot.to_string();
    }
}

fn main() {
    let mut day = Day03 { vals: Vec::new() };
    run_day(&mut day);
}
