use std::collections::HashMap;

use lazy_regex::regex;

extern crate common;

use common::framework::{parse_lines, run_day, BaseDay, InputReader};
use common::utils::inc_counter;

struct Rule {
    pair: String,
    insert: String,
}

struct Day14 {
    template: String,
    rules: HashMap<String, String>,
}

fn run_steps(template: &String, rules: &HashMap<String, String>, steps: i32, verbose: bool) -> i64 {
    let mut pairs: HashMap<String, i64> = HashMap::new();
    for idx in 0..template.len() - 1 {
        let seg = template[idx..idx + 2].to_owned();
        inc_counter(&mut pairs, seg, 1);
    }

    for step in 0..steps {
        if verbose {
            println!("Step {}: {:?}", step, pairs);
        }
        let mut next = HashMap::new();
        for (pair, count) in pairs.iter() {
            if let Some(insert) = rules.get(pair) {
                let first = pair[0..1].to_owned() + &insert.to_owned();
                inc_counter(&mut next, first, *count);
                let second = insert.to_owned() + &pair[1..2].to_owned();
                inc_counter(&mut next, second, *count);
            } else {
                next.insert(pair.to_owned(), *count);
            }
        }
        pairs = next;
    }

    let mut quantities = HashMap::new();
    for (pair, count) in pairs.iter() {
        inc_counter(&mut quantities, pair[0..1].to_owned(), *count);
        inc_counter(&mut quantities, pair[1..2].to_owned(), *count);
    }

    // Fix the double-counts (first and last char are not double-counted):
    let first_e = template[0..1].to_owned();
    let last_e = template[template.len() - 1..template.len()].to_owned();
    inc_counter(&mut quantities, first_e.clone(), -1);
    inc_counter(&mut quantities, last_e.clone(), -1);
    for count in quantities.values_mut() {
        *count /= 2;
    }
    inc_counter(&mut quantities, first_e.clone(), 1);
    inc_counter(&mut quantities, last_e.clone(), 1);

    if verbose {
        println!("Quantities: {:?}", quantities);
    }
    let mut sorted: Vec<i64> = quantities.into_values().collect();
    sorted.sort();
    sorted.reverse();
    return sorted[0] - sorted[sorted.len() - 1];
}

impl BaseDay for Day14 {
    fn parse(&mut self, input: &mut InputReader) {
        let mut parse_template = |line: String| -> String { line };
        self.template = parse_lines(input, &mut parse_template)
            .into_iter()
            .next()
            .unwrap();

        let mut parse_rule = |line: String| -> Rule {
            let sep = regex!(r#"\s*->\s*"#);
            let pieces: Vec<&str> = sep.split(&line).collect();
            return Rule {
                pair: pieces[0].to_owned(),
                insert: pieces[1].to_owned(),
            };
        };
        self.rules = parse_lines(input, &mut parse_rule)
            .into_iter()
            .map(|r| (r.pair, r.insert))
            .collect();
    }

    fn pt1(&mut self) -> String {
        let diff = run_steps(&self.template, &self.rules, 10, true);
        return diff.to_string();
    }

    fn pt2(&mut self) -> String {
        let diff = run_steps(&self.template, &self.rules, 40, false);
        return diff.to_string();
    }
}

fn main() {
    let mut day = Day14 {
        template: "".to_string(),
        rules: HashMap::new(),
    };
    run_day(&mut day);
}
