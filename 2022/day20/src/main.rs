extern crate common;

use common::framework::{parse_lines, run_day, BaseDay, InputReader};

struct Day20 {
    vals: Vec<i64>,
}

fn move_by_dumb(i: usize, shift: i64, idxs: &mut Vec<usize>) {
    let mut cur = i;
    for _ in 0..shift.abs() {
        let nxt = (cur + (if shift > 0 { 1 } else { idxs.len() - 1 })) % idxs.len();
        let old = idxs[cur];
        idxs[cur] = idxs[nxt];
        idxs[nxt] = old;
        cur = nxt;
    }
}

fn move_by(i: usize, shift: i64, idxs: &mut Vec<usize>) {
    let mut shift = shift % (idxs.len() - 1) as i64;
    if shift < 0 {
        shift = idxs.len() as i64 - 1 + shift;
    }

    let idx = idxs[i];
    let new_i = i + shift as usize;
    if new_i < idxs.len() {
        idxs.insert(new_i + 1, idx);
        idxs.remove(i);
    } else {
        idxs.remove(i);
        idxs.insert(new_i % idxs.len(), idx);
    }
}

fn mix(vals: &Vec<i64>, times: usize) -> Vec<i64> {
    let mut idxs = (0..vals.len()).collect::<Vec<usize>>();
    for _ in 0..times {
        'outer: for i in 0..idxs.len() {
            for p in 0..idxs.len() {
                if idxs[p] == i {
                    move_by(p, vals[i], &mut idxs);
                    continue 'outer;
                }
            }
            panic!("Couldn't find index {}", i);
        }
    }
    return idxs.iter().map(|&i| vals[i]).collect::<Vec<i64>>();
}

fn get_coords(vals: &Vec<i64>) -> i64 {
    for i in 0..vals.len() {
        if vals[i] == 0 {
            return vec![1000, 2000, 3000]
                .into_iter()
                .map(|v| vals[(i + v) % vals.len()])
                .sum();
        }
    }
    panic!("Never found a zero?");
}

impl BaseDay for Day20 {
    fn parse(&mut self, input: &mut InputReader) {
        self.vals.extend(parse_lines(input, &mut |line: String| {
            line.parse::<i64>().unwrap()
        }))
    }

    fn pt1(&mut self) -> String {
        let mixed = mix(&self.vals, 1);
        let coords = get_coords(&mixed);
        return coords.to_string();
    }

    fn pt2(&mut self) -> String {
        let mut applied = self
            .vals
            .iter()
            .map(|v| v * 811589153)
            .collect::<Vec<i64>>();
        let mixed = mix(&applied, 10);
        let coords = get_coords(&mixed);
        return coords.to_string();
    }
}

fn main() {
    let mut day = Day20 { vals: Vec::new() };
    run_day(&mut day);
}
