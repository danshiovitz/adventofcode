use std::collections::{HashMap, HashSet};

use lazy_regex::regex;

extern crate common;

use common::framework::{parse_lines, run_day, BaseDay, InputReader};
use common::solver::{FlagManager, FlagSet, Trid};

struct Day12 {
    cxns: HashMap<String, Vec<String>>
}

#[allow(dead_code)]
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

fn find_paths_dfs(cxns: &HashMap<String, Vec<String>>, start: &str, end: &str, allow_extra: bool) -> Vec<Vec<String>> {
    let seen_mgr = FlagManager::from(cxns.keys().map(|s| s.as_str()));

    let start_id = seen_mgr.translate(start);
    let end_id = seen_mgr.translate(end);

    let mut state = State { step: start_id, seen: seen_mgr.init(), used_extra: false };
    seen_mgr.set(&mut state.seen, start_id);

    let trans_cxns = cxns.iter().map(|(k, v)| (seen_mgr.translate(k), v.iter().map(|s| seen_mgr.translate(s)).collect())).collect();

    let reusable = cxns.keys().filter_map(|k| if !k.chars().next().unwrap().is_ascii_lowercase() { Some(seen_mgr.translate(k)) } else { None }).collect();
    let recurser = Recurser {
        cxns: trans_cxns,
        start: start_id,
        end: end_id,
        reusable: reusable,
        allow_extra: allow_extra,
        seen_mgr: seen_mgr,
    };

    let paths = recurser.execute(state, &mut HashMap::new());
    return paths.into_iter().map(|p| p.into_iter().map(|s| recurser.seen_mgr.translate_back(s)).collect()).collect();
}

#[derive(Clone, Hash, PartialEq, Eq)]
struct State {
    step: Trid,
    seen: FlagSet,
    used_extra: bool,
}

struct Recurser {
    cxns: HashMap<Trid, Vec<Trid>>,
    start: Trid,
    end: Trid,
    reusable: HashSet<Trid>,
    allow_extra: bool,
    seen_mgr: FlagManager,
}

impl Recurser {
    fn execute(&self, state: State, cache: &mut HashMap<State, Vec<Vec<Trid>>>) -> Vec<Vec<Trid>> {
        if state.step == self.end {
            return vec![vec![state.step]];
        }

        if let Some(found) = cache.get(&state) {
            return found.clone();
        }

        let mut ret = Vec::new();

        let others = self.cxns.get(&state.step).unwrap();
        for other in others {
            let other = *other;
            if self.seen_mgr.get(&state.seen, other) {
                if other != self.start && !state.used_extra && self.allow_extra {
                    let new_state = State {
                        step: other,
                        seen: state.seen,
                        used_extra: true,
                    };
                    ret.extend(self.execute(new_state, cache));
                }
                continue;
            }

            let mut new_state = State {
                step: other,
                seen: state.seen,
                used_extra: state.used_extra,
            };

            if !self.reusable.contains(&other) {
                self.seen_mgr.set(&mut new_state.seen, other);
            }
            ret.extend(self.execute(new_state, cache));
        }

        for path in &mut ret {
            path.insert(0, state.step.clone());
        }

        cache.insert(state, ret.clone());

        return ret;
    }
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
        let paths = find_paths_dfs(&self.cxns, "start", "end", false);
        // for p in &paths {
        //     println!("{:?}", p);
        // }
        return paths.len().to_string();
    }

    fn pt2(&mut self) -> String {
        let paths = find_paths_dfs(&self.cxns, "start", "end", true);
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
