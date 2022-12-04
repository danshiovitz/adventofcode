use lazy_regex::regex;

extern crate common;

use common::framework::{parse_lines, run_day, BaseDay, InputReader};

struct Pair {
    first: (i32, i32),
    second: (i32, i32),
}

struct Day04 {
    vals: Vec<Pair>,
}

impl BaseDay for Day04 {
    fn parse(&mut self, input: &mut InputReader) {
        fn parse_line(line: String) -> Pair {
            let rex = regex!(r#"\s*(\d+)\s*-\s*(\d+)\s*,\s*(\d+)\s*-\s*(\d+)\s*"#);
            match rex.captures(&line) {
                Some(c) => {
                    let v1 = c[1].parse::<i32>().unwrap();
                    let v2 = c[2].parse::<i32>().unwrap();
                    let v3 = c[3].parse::<i32>().unwrap();
                    let v4 = c[4].parse::<i32>().unwrap();
                    return Pair { first: (v1, v2), second: (v3, v4) };
                }
                None => panic!("Bad line: {}", &line),
            };
        }
        self.vals = parse_lines(input, &mut parse_line);
    }

    fn pt1(&mut self) -> String {
        fn has_overlap(p: &Pair) -> bool {
            return (p.first.0 <= p.second.0 && p.first.1 >= p.second.1)
                || (p.second.0 <= p.first.0 && p.second.1 >= p.first.1);
        }
        let tot = self.vals.iter().filter(|v| has_overlap(*v)).count();
        return tot.to_string();
    }

    fn pt2(&mut self) -> String {
        fn in_range(v: i32, range: (i32, i32)) -> bool {
            return v >= range.0 && v <= range.1;
        }
        fn has_overlap(p: &Pair) -> bool {
            return in_range(p.first.0, p.second)
                || in_range(p.first.1, p.second)
                || in_range(p.second.0, p.first)
                || in_range(p.second.1, p.first);
        }
        let tot = self.vals.iter().filter(|v| has_overlap(*v)).count();
        return tot.to_string();
    }
}

fn main() {
    let mut day = Day04 { vals: Vec::new() };
    run_day(&mut day);
}
