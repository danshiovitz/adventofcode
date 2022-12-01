extern crate common;

use common::framework::{parse_numbers, run_day, BaseDay, InputReader};

struct Day01 {
    vals: Vec<Vec<i32>>,
}

impl BaseDay for Day01 {
    fn parse(&mut self, input: &mut InputReader) {
        loop {
            let parsed = parse_numbers(input);
            if parsed.is_empty() {
                break;
            }
            self.vals.push(parsed);
        }
    }

    fn pt1(&mut self) -> String {
        let sums = self.vals.iter().map(|v| v.into_iter().sum::<i32>());
        return sums.max().unwrap().to_string();
    }

    fn pt2(&mut self) -> String {
        let mut sums: Vec<i32> = self.vals.iter().map(|v| v.into_iter().sum::<i32>()).collect();
        sums.sort();
        sums.reverse();
        return sums.into_iter().take(3).sum::<i32>().to_string();
    }
}

fn main() {
    let mut day = Day01 { vals: Vec::new() };
    run_day(&mut day);
}
