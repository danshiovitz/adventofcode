extern crate common;

use common::framework::{parse_lines, run_day, BaseDay, InputReader};

struct Day03 {
    vals: Vec<String>,
}

fn find_common(vals: &Vec<String>, more: bool) -> String {
    let mut cur = vals
        .iter()
        .map(|v| v.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    let mut idx = 0;
    while cur.len() > 1 {
        let cur_cnt = cur.iter().filter(|v| v[idx] == '1').count();
        let half = (cur.len() / 2) + (cur.len() % 2);
        let char_match = if (more && cur_cnt >= half) || (!more && cur_cnt < half) {
            '1'
        } else {
            '0'
        };
        println!(
            "Chose char={} for size={}, idx={}",
            char_match,
            cur.len(),
            idx
        );
        cur = cur.into_iter().filter(|v| v[idx] == char_match).collect();
        idx += 1;
    }
    return cur[0].iter().collect::<String>();
}

impl BaseDay for Day03 {
    fn parse(&mut self, input: &mut InputReader) {
        self.vals = parse_lines(input, &mut |line: String| line);
    }

    fn pt1(&mut self) -> String {
        let each_len = self.vals[0].len();
        let mut cnts = vec![0; each_len];
        for val in &self.vals {
            for (i, c) in val.chars().enumerate() {
                if c == '1' {
                    cnts[i] += 1;
                }
            }
        }
        let half = self.vals.len() / 2;
        let gamma_str = cnts
            .iter()
            .map(|&v| if v >= half { '1' } else { '0' })
            .collect::<String>();
        let epsilon_str = cnts
            .iter()
            .map(|&v| if v < half { '1' } else { '0' })
            .collect::<String>();
        let gamma = isize::from_str_radix(&gamma_str, 2).unwrap();
        let epsilon = isize::from_str_radix(&epsilon_str, 2).unwrap();
        println!(
            "gamma: {} ({}), epsilon: {} ({})",
            gamma_str, gamma, epsilon_str, epsilon
        );
        return (gamma * epsilon).to_string();
    }

    fn pt2(&mut self) -> String {
        let oxygen_str = find_common(&self.vals, true);
        let co2_str = find_common(&self.vals, false);
        let oxygen = isize::from_str_radix(&oxygen_str, 2).unwrap();
        let co2 = isize::from_str_radix(&co2_str, 2).unwrap();
        println!(
            "oxygen: {} ({}), co2: {} ({})",
            oxygen_str, oxygen, co2_str, co2
        );

        return (oxygen * co2).to_string();
    }
}

fn main() {
    let mut day = Day03 { vals: Vec::new() };
    run_day(&mut day);
}
