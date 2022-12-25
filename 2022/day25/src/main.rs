extern crate common;

use common::framework::{parse_lines, run_day, BaseDay, InputReader};

struct Day25 {
    vals: Vec<Vec<char>>,
}

fn to_snafu(val: i64) -> Vec<char> {
    let mut cur = val;
    let mut ret = Vec::new();
    let mut carry = false;
    while cur != 0 {
        let m = cur % 5;
        let mut i = m;
        if carry {
            i += 1;
            carry = false;
            if i == 5 {
                carry = true;
                i = 0;
            }
        }
        if i >= 0 && i <= 2 {
            ret.insert(0, ('0' as u8 + i as u8) as char);
        } else {
            carry = true;
            if i == 3 {
                ret.insert(0, '=');
            } else {
                ret.insert(0, '-');
            }
        }
        cur -= m;
        cur /= 5;
    }
    if carry {
        ret.insert(0, '1');
    }
    return ret;
}

fn from_snafu(val: &Vec<char>) -> i64 {
    let mut ret = 0;
    for &ch in val {
        ret *= 5;
        if ch == '2' {
            ret += 2;
        } else if ch == '1' {
            ret += 1;
        } else if ch == '0' {
            ret += 0;
        } else if ch == '-' {
            ret -= 1;
        } else if ch == '=' {
            ret -= 2;
        } else {
            panic!("Unknown digit: {}", ch);
        }
    }
    return ret;
}

impl BaseDay for Day25 {
    fn parse(&mut self, input: &mut InputReader) {
        self.vals.extend(parse_lines(input, &mut |line: String| {
            line.chars().collect::<Vec<char>>()
        }));
    }

    fn pt1(&mut self) -> String {
        let tot = self.vals.iter().map(|s| from_snafu(s)).sum();
        return to_snafu(tot).into_iter().collect::<String>();
    }

    fn pt2(&mut self) -> String {
        return "".to_string();
    }
}

fn main() {
    let mut day = Day25 { vals: Vec::new() };
    run_day(&mut day);
}
