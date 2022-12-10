use itertools::Itertools;
use lazy_regex::regex;

extern crate common;

use common::framework::{parse_lines, run_day, BaseDay, InputReader};

enum Op {
    Noop(),
    Addx(i32),
}

struct Day10 {
    vals: Vec<Op>,
}

fn run(ops: &Vec<Op>) -> i32 {
    let mut reg = 1;
    let mut turn = 1;
    let mut tot = 0;
    for op in ops {
        let mut pending = Vec::new();
        match op {
            Op::Noop() => {
                pending.push(0);
            }
            Op::Addx(val) => {
                pending.push(0);
                pending.push(*val);
            }
        }

        for val in pending {
            if turn == 20 || (turn > 40 && turn % 40 == 20) {
                println!(
                    "cycle: {}, value: {}, strength: {}",
                    turn,
                    reg,
                    turn as i32 * reg
                );
                tot += turn as i32 * reg;
            }
            reg += val;
            turn += 1;
        }
    }

    return tot;
}

fn draw(ops: &Vec<Op>) -> Vec<String> {
    let mut reg = 1;
    let mut turn = 1;
    let mut line = Vec::new();
    let mut rendered = Vec::new();
    for op in ops {
        let mut pending = Vec::new();
        match op {
            Op::Noop() => {
                pending.push(0);
            }
            Op::Addx(val) => {
                pending.push(0);
                pending.push(*val);
            }
        }

        for val in pending {
            let cursor = (turn - 1) % 40;
            if reg >= cursor - 1 && reg <= cursor + 1 {
                line.push('#');
            } else {
                line.push('.');
            }
            if turn > 0 && turn % 40 == 0 {
                rendered.push(line.iter().collect::<String>());
                line.clear();
            }
            reg += val;
            turn += 1;
        }
    }

    return rendered;
}

impl BaseDay for Day10 {
    fn parse(&mut self, input: &mut InputReader) {
        self.vals = parse_lines(input, &mut |line: String| {
            if line == "noop" {
                return Op::Noop();
            }
            match regex!(r#"addx\s+(-?\d+)"#).captures(&line) {
                Some(c) => {
                    let amt = c[1].parse::<i32>().unwrap();
                    return Op::Addx(amt);
                }
                None => {}
            }
            panic!("Bad line: {}", line);
        });
    }

    fn pt1(&mut self) -> String {
        return run(&self.vals).to_string();
    }

    fn pt2(&mut self) -> String {
        let output = draw(&self.vals);
        println!();
        for ln in &output {
            println!("{}", ln);
        }
        println!();
        return output
            .into_iter()
            .intersperse("/".to_string())
            .collect::<String>();
    }
}

fn main() {
    let mut day = Day10 { vals: Vec::new() };
    run_day(&mut day);
}
