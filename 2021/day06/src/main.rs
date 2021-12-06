extern crate common;

use common::framework::{parse_numbers, run_day, BaseDay, InputReader};

struct Day06 {
    vals: Vec<i32>,
}

fn grow(vals: &Vec<i32>, days: i32, verbose: bool) -> String {
    let mut cnts = vec![0 as i64; 9];
    for v in vals {
        cnts[*v as usize] += 1;
    }
    for d in 0..days {
        if verbose {
            print!("Day {}: ", d);
            for idx in 0..cnts.len() {
                for _ in 0..cnts[idx] {
                    print!("{},", idx);
                }
            }
            println!();
        }
        let zeroes = cnts[0];
        cnts.rotate_left(1);
        cnts[6] += zeroes;
    }
    return cnts.iter().sum::<i64>().to_string();
}

impl BaseDay for Day06 {
    fn parse(&mut self, input: &mut InputReader) {
        self.vals = parse_numbers(input);
    }

    fn pt1(&mut self) -> String {
        return grow(&self.vals, 80, false);
    }

    fn pt2(&mut self) -> String {
        return grow(&self.vals, 256, false);
    }
}

fn main() {
    let mut day = Day06 { vals: Vec::new() };
    run_day(&mut day);
}
