use std::collections::{HashMap, HashSet};

use lazy_regex::regex;

extern crate common;

use common::framework::{parse_lines, run_day, BaseDay, InputReader};
use common::solver::{FlagManager, FlagSet, SolverBase, SolverState, Trid, count_all_paths_dfs};

struct Day12 {
    cxns: HashMap<String, Vec<String>>,
}

fn find_paths_dfs(
    cxns: &HashMap<String, Vec<String>>,
    start: &str,
    end: &str,
    allow_extra: bool,
) -> i32 {
    let seen_mgr = FlagManager::from(cxns.keys().map(|s| s.as_str()));

    let start_id = seen_mgr.translate(start);
    let end_id = seen_mgr.translate(end);

    let mut state = State {
        step: start_id,
        seen: seen_mgr.init(),
        used_extra: false,
    };
    seen_mgr.set(&mut state.seen, start_id);

    let trans_cxns = cxns
        .iter()
        .map(|(k, v)| {
            (
                seen_mgr.translate(k),
                v.iter().map(|s| seen_mgr.translate(s)).collect(),
            )
        })
        .collect();

    let reusable = cxns
        .keys()
        .filter_map(|k| {
            if !k.chars().next().unwrap().is_ascii_lowercase() {
                Some(seen_mgr.translate(k))
            } else {
                None
            }
        })
        .collect();
    let recurser = Recurser {
        cxns: trans_cxns,
        start: start_id,
        end: end_id,
        reusable: reusable,
        allow_extra: allow_extra,
        seen_mgr: seen_mgr,
    };

    return count_all_paths_dfs(&recurser, &state);
}


#[derive(Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Debug)]
struct State {
    step: Trid,
    seen: FlagSet,
    used_extra: bool,
}

impl SolverState for State {}

struct Recurser {
    cxns: HashMap<Trid, Vec<Trid>>,
    start: Trid,
    end: Trid,
    reusable: HashSet<Trid>,
    allow_extra: bool,
    seen_mgr: FlagManager,
}

impl SolverBase<State> for Recurser {
    fn is_verbose(&self) -> bool {
        return false;
    }

    fn is_finished(&self, state: &State) -> bool {
        return state.step == self.end;
    }

    fn gen_possible_moves(&self, state: &State) -> Vec<(i32, State)> {
        let mut ret: Vec<(i32, State)> = vec![];

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
                    ret.push((1, new_state));
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
            ret.push((1, new_state));
        }
        return ret;
    }

    fn print_state(&self, state: &State) {
        println!("{:?}", state);
        println!();
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
                self.cxns
                    .insert(pieces[0].to_owned(), vec![pieces[1].to_owned()]);
            }
            if let Some(pps) = self.cxns.get_mut(pieces[1]) {
                pps.push(pieces[0].to_owned());
            } else {
                self.cxns
                    .insert(pieces[1].to_owned(), vec![pieces[0].to_owned()]);
            }
            return true;
        };
        parse_lines(input, &mut parse_line);
    }

    fn pt1(&mut self) -> String {
        let num_paths = find_paths_dfs(&self.cxns, "start", "end", false);
        return num_paths.to_string();
    }

    fn pt2(&mut self) -> String {
        let num_paths = find_paths_dfs(&self.cxns, "start", "end", true);
        return num_paths.to_string();
    }
}

fn main() {
    let mut day = Day12 {
        cxns: HashMap::new(),
    };
    run_day(&mut day);
}
