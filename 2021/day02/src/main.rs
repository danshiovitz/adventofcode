extern crate common;

use common::framework::{parse_lines, run_day, BaseDay, InputReader};

struct Day02 {
    vals: Vec<Operation>,
}

enum Operation {
    Down(i32), // Up is negative down (this is my band's debut song)
    Forward(i32),
}

impl BaseDay for Day02 {
    fn parse(&mut self, input: &mut InputReader) {
        fn parse_line(line: String) -> Operation {
            let words = line.split(" ").collect::<Vec<&str>>();
            let val = words[1].parse::<i32>().unwrap();
            if words[0] == "forward" {
                return Operation::Forward(val);
            } else if words[0] == "up" {
                return Operation::Down(-val);
            } else if words[0] == "down" {
                return Operation::Down(val);
            } else {
                panic!("Bad op: {}", words[0]);
            }
        }
        self.vals = parse_lines(input, &mut parse_line);
    }

    fn pt1(&mut self) -> String {
        let mut down_pos = 0;
        let mut fwd_pos = 0;
        for val in &self.vals {
            match val {
                Operation::Forward(amt) => fwd_pos += amt,
                Operation::Down(amt) => down_pos += amt,
            }
        }
        println!("Coords is {},{}", fwd_pos, down_pos);
        return (fwd_pos * down_pos).to_string();
    }

    fn pt2(&mut self) -> String {
        let mut aim = 0;
        let mut down_pos = 0;
        let mut fwd_pos = 0;
        for val in &self.vals {
            match val {
                Operation::Forward(amt) => {
                    fwd_pos += amt;
                    down_pos += aim * amt;
                }
                Operation::Down(amt) => aim += amt,
            }
        }
        println!("Coords is {},{}", fwd_pos, down_pos);
        return (fwd_pos * down_pos).to_string();
    }
}

fn main() {
    let mut day = Day02 { vals: Vec::new() };
    run_day(&mut day);
}
