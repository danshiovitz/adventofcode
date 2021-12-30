use lazy_regex::regex;
use std::collections::HashMap;

extern crate common;

use common::framework::{parse_numbers, parse_records, run_day, BaseDay, InputReader};

struct Board {
    idx: i32,
    win_turn: i32,
    score: i32,
}

struct Day04 {
    vals: Vec<Board>,
}

fn parse_solve_board(lines: &Vec<String>, numbers: &HashMap<i32, i32>) -> Board {
    #[derive(Copy, Clone)]
    struct P {
        value: i32,
        turn: i32,
    }

    let sep = regex!(r#"\s+"#);
    let num_iter = |line: &str| {
        sep.split(&line.trim())
            .map(|n| n.parse::<i32>().unwrap())
            .map(|n| P { value: n, turn: *numbers.get(&n).unwrap() })
            .collect::<Vec<P>>()
    };
    let vals: Vec<Vec<P>> = lines.iter().map(|line| num_iter(line)).collect();

    // Find the maximum turn of each row and each column, and then take the minimum of those
    // to determine the winning turn
    let rows = vals.iter();
    let cols = (0..vals[0].len())
        .map(|c| vals.iter().map(|row| row[c]).collect::<Vec<P>>())
        .collect::<Vec<_>>();
    let win_p: P = *rows
        .chain(cols.iter())
        .map(|vs| vs.iter().max_by(|x, y| x.turn.cmp(&y.turn)).unwrap())
        .min_by(|x, y| x.turn.cmp(&y.turn))
        .unwrap();

    // calc score
    let unmarked: i32 = vals
        .iter()
        .map(|row| {
            row.iter()
                .filter(|p| p.turn > win_p.turn)
                .map(|p| p.value)
                .sum::<i32>()
        })
        .sum();
    let score = unmarked * win_p.value;
    return Board { idx: 0, win_turn: win_p.turn, score: score };
}

impl BaseDay for Day04 {
    fn parse(&mut self, input: &mut InputReader) {
        let numbers: HashMap<i32, i32> = parse_numbers(input)
            .iter()
            .enumerate()
            .map(|(idx, val)| (*val, idx as i32))
            .collect();

        self.vals = parse_records(input, &mut |lines: &Vec<String>| {
            parse_solve_board(lines, &numbers)
        });
        for idx in 0..self.vals.len() {
            self.vals[idx].idx = idx as i32;
        }
        self.vals
            .sort_by(|a, b| a.win_turn.partial_cmp(&b.win_turn).unwrap());
    }

    fn pt1(&mut self) -> String {
        return self.vals[0].score.to_string();
    }

    fn pt2(&mut self) -> String {
        return self.vals[self.vals.len() - 1].score.to_string();
    }
}

fn main() {
    let mut day = Day04 { vals: Vec::new() };
    run_day(&mut day);
}
