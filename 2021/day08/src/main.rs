use std::collections::HashSet;

use lazy_regex::regex;

extern crate common;

use common::framework::{parse_lines, run_day, BaseDay, InputReader};

struct Entry {
    patterns: Vec<HashSet<char>>,
    output: Vec<HashSet<char>>,
}

struct Day08 {
    vals: Vec<Entry>,
}

fn solve_entry(entry: &Entry) -> i32 {
    let find_match = |f: &mut dyn FnMut(&HashSet<char>) -> bool| -> &HashSet<char> {
        let candidates = entry.patterns.iter().filter(|p| f(p)).collect::<Vec<_>>();
        if candidates.len() != 1 {
            panic!("Too many candidates?");
        }
        return candidates[0];
    };

    let one: &HashSet<char> = find_match(&mut |p| p.len() == 2);
    let seven: &HashSet<char> = find_match(&mut |p| p.len() == 3);
    let four: &HashSet<char> = find_match(&mut |p| p.len() == 4);
    let eight: &HashSet<char> = find_match(&mut |p| p.len() == 7);

    let segs_bd: HashSet<char> = four.difference(one).map(|c| *c).collect::<HashSet<char>>();

    let five: &HashSet<char> =
        find_match(&mut |p| p.len() == 5 && p.intersection(&segs_bd).count() == 2);
    let three: &HashSet<char> =
        find_match(&mut |p| p.len() == 5 && p.intersection(&one).count() == 2);
    let two: &HashSet<char> = find_match(&mut |p| p.len() == 5 && p != five && p != three);

    let zero: &HashSet<char> =
        find_match(&mut |p| p.len() == 6 && p.intersection(&five).count() == 4);
    let nine: &HashSet<char> =
        find_match(&mut |p| p.len() == 6 && p != zero && p.intersection(&one).count() == 2);
    let six: &HashSet<char> =
        find_match(&mut |p| p.len() == 6 && p.intersection(&one).count() == 1);

    let digits = vec![zero, one, two, three, four, five, six, seven, eight, nine];
    let to_val = |d| digits.iter().position(|p| *p == d).unwrap() as i32;
    let s = entry
        .output
        .iter()
        .map(|d| to_val(d))
        .fold(0, |tot, val| tot * 10 + val);
    // println!("Val: {}", s);
    return s;
}

impl BaseDay for Day08 {
    fn parse(&mut self, input: &mut InputReader) {
        fn parse_line(line: String) -> Entry {
            let sep = regex!(r#"\s*\|\s*"#);
            let pieces = sep.split(&line).collect::<Vec<&str>>();
            let fix = |s: &str| s.chars().collect::<HashSet<char>>();
            return Entry {
                patterns: pieces[0].split(" ").map(|s| fix(s)).collect(),
                output: pieces[1].split(" ").map(|s| fix(s)).collect(),
            };
        }
        self.vals = parse_lines(input, &mut parse_line);
    }

    fn pt1(&mut self) -> String {
        let tot: i32 = self
            .vals
            .iter()
            .map(|e| {
                e.output
                    .iter()
                    .filter(|p| p.len() <= 4 || p.len() == 7)
                    .count() as i32
            })
            .sum();
        return tot.to_string();
    }

    fn pt2(&mut self) -> String {
        return self
            .vals
            .iter()
            .map(|e| solve_entry(&e))
            .sum::<i32>()
            .to_string();
    }
}

fn main() {
    let mut day = Day08 { vals: Vec::new() };
    run_day(&mut day);
}
