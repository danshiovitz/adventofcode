extern crate common;

use common::framework::{parse_lines, run_day, BaseDay, InputReader};

struct Day01 {
    vals: Vec<i32>,
}

impl BaseDay for Day01 {
    fn parse(&mut self, input: &mut InputReader) {
        self.vals = parse_lines(input, &mut |line: String| line.parse::<i32>().unwrap());
    }

    fn pt1(&mut self) -> String {
        let cnt = (1 as usize..self.vals.len())
            .into_iter()
            .filter(|i| self.vals[*i] > self.vals[i - 1])
            .count();
        return cnt.to_string();
    }

    fn pt2(&mut self) -> String {
        let cnt = (3 as usize..self.vals.len())
            .into_iter()
            .filter(|i| self.vals[*i] > self.vals[i - 3])
            .count();
        return cnt.to_string();
    }
}

fn main() {
    let mut day = Day01 { vals: Vec::new() };
    run_day(&mut day);
}
