use std::collections::{HashMap, HashSet};

use lazy_regex::regex;

extern crate common;

use common::framework::{parse_lines, run_day, BaseDay, InputReader};

struct Day12 {
    cxns: HashMap<String, Vec<String>>
}

fn find_paths(cxns: &HashMap<String, Vec<String>>, start: &str, end: &str, allow_extra: bool) -> Vec<Vec<String>> {
    struct WorkItem {
        steps: Vec<String>,
        seen: HashSet<String>,
        extra: Option<String>,
    }

    let mut completed = Vec::new();
    let mut working = vec![ WorkItem { steps: vec![start.to_owned()], seen: HashSet::from([start.to_owned()]), extra: None } ];
    while !working.is_empty() {
        let cur = working.remove(0);
        let cur_step = &cur.steps[cur.steps.len() - 1];
        let others = cxns.get(cur_step).unwrap();
        for other in others {
            let mut use_extra = false;
            if cur.seen.contains(other) {
                if other != "start" && cur.extra.is_none() && allow_extra {
                    use_extra = true;
                } else {
                    continue;
                }
            }

            let mut other_item = WorkItem { steps: cur.steps.clone(), seen: cur.seen.clone(), extra: if use_extra { Some(other.clone()) } else { cur.extra.clone() } };
            other_item.steps.push(other.clone());

            if other == end {
                completed.push(other_item.steps.clone());
                continue;
            }

            if other.chars().next().unwrap().is_ascii_lowercase() {
                other_item.seen.insert(other.clone());
            }

            working.push(other_item);
        }
    }
    return completed;
}

impl BaseDay for Day12 {
    fn parse(&mut self, input: &mut InputReader) {
        self.cxns = HashMap::new();
        let mut parse_line = |line: String| -> bool {
            let sep = regex!(r#"\s*-\s*"#);
            let pieces: Vec<&str> = sep.split(&line).collect();
            if let Some(pps) = self.cxns.get_mut(pieces[0]) {
                pps.push(pieces[1].to_owned());
            } else {
                self.cxns.insert(pieces[0].to_owned(), vec![ pieces[1].to_owned() ]);
            }
            if let Some(pps) = self.cxns.get_mut(pieces[1]) {
                pps.push(pieces[0].to_owned());
            } else {
                self.cxns.insert(pieces[1].to_owned(), vec![ pieces[0].to_owned() ]);
            }
            return true;
        };
        parse_lines(input, &mut parse_line);
    }

    fn pt1(&mut self) -> String {
        let paths = find_paths(&self.cxns, "start", "end", false);
        // for p in &paths {
        //     println!("{:?}", p);
        // }
        return paths.len().to_string();
    }

    fn pt2(&mut self) -> String {
        let paths = find_paths(&self.cxns, "start", "end", true);
        // for p in &paths {
        //     println!("{:?}", p);
        // }
        return paths.len().to_string();
    }
}

fn main() {
    let mut day = Day12 { cxns: HashMap::new() };
    run_day(&mut day);
}
