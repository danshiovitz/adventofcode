use std::collections::HashMap;

use lazy_regex::regex;

extern crate common;

use common::framework::{parse_lines, run_day, BaseDay, InputReader};

struct Day21 {
    vals: Vec<i32>,
}

fn run_deterministic_game(starts: &Vec<i32>) -> i32 {
    let mut die = 0;
    let mut roll = || {
        die += 1;
        if die > 100 {
            die = 1;
        }
        return die;
    };

    let mut curs = starts.clone();
    let mut scores = vec![0; curs.len()];
    let mut num_rolls = 0;
    loop {
        for idx in 0..curs.len() {
            curs[idx] += roll();
            curs[idx] += roll();
            curs[idx] += roll();
            curs[idx] = ((curs[idx] - 1) % 10) + 1;
            num_rolls += 3;
            scores[idx] += curs[idx];
            if scores[idx] >= 1000 {
                let lose_score = scores.iter().min().unwrap();
                return lose_score * num_rolls;
            }
        }
    }
}

fn run_dirac_game(starts: &Vec<i32>) -> i64 {
    let state = State {
        idx: 0,
        curs: starts.clone(),
        scores: vec![0; starts.len()],
    };
    let mut cache = HashMap::new();
    let scores = run_dirac_game_recur(&state, &mut cache);
    println!("Scores: {:?}", scores);
    return *scores.iter().max().unwrap();
}

#[derive(PartialEq, Eq, Clone, Debug, Hash)]
struct State {
    idx: usize,
    curs: Vec<i32>,
    scores: Vec<i32>,
}

fn run_dirac_game_recur(state: &State, cache: &mut HashMap<State, Vec<i64>>) -> Vec<i64> {
    let mut ret = vec![0 as i64; state.curs.len()];

    let paths = vec![(3, 1), (4, 3), (5, 6), (6, 7), (7, 6), (8, 3), (9, 1)];
    for (roll, cnt) in paths.iter() {
        let mut new_state = state.clone();
        new_state.curs[new_state.idx] += *roll;
        new_state.curs[new_state.idx] = ((new_state.curs[new_state.idx] - 1) % 10) + 1;
        new_state.scores[new_state.idx] += new_state.curs[new_state.idx];

        if new_state.scores[new_state.idx] >= 21 {
            ret[new_state.idx] += *cnt;
        } else {
            new_state.idx = (new_state.idx + 1) % new_state.curs.len();
            let recur_scores: Vec<i64> = if let Some(cached) = cache.get(&new_state) {
                cached.clone()
            } else {
                run_dirac_game_recur(&new_state, cache)
            };
            for (idx, score) in recur_scores.iter().enumerate() {
                ret[idx] += score * *cnt;
            }
        }
    }
    // println!("State is {:?}, scores is {:?}", state, ret);
    cache.insert(state.clone(), ret.clone());
    return ret;
}

impl BaseDay for Day21 {
    fn parse(&mut self, input: &mut InputReader) {
        fn parse_line(line: String) -> i32 {
            let rex = regex!(r#"Player \d+ starting position:\s*(\d+)"#);
            return match rex.captures(&line) {
                Some(c) => c[1].parse::<i32>().unwrap(),
                None => panic!("Bad line: {}", &line),
            };
        }
        self.vals = parse_lines(input, &mut parse_line);
    }

    fn pt1(&mut self) -> String {
        return run_deterministic_game(&self.vals).to_string();
    }

    fn pt2(&mut self) -> String {
        return run_dirac_game(&self.vals).to_string();
    }
}

fn main() {
    let mut day = Day21 { vals: Vec::new() };
    run_day(&mut day);
}
