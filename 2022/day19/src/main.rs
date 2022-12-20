use lazy_regex::{regex, Captures};
use std::collections::HashMap;

extern crate common;

use common::framework::{parse_lines, parse_regexp, run_day, BaseDay, InputReader};

const ORE: usize = 0;
const CLAY: usize = 1;
const OBSIDIAN: usize = 2;
const GEODE: usize = 3;
const NOTHING: usize = 4;

struct Blueprint {
    id: i32,
    costs: Vec<Vec<i32>>,
}

struct Day19 {
    vals: Vec<Blueprint>,
}

fn compute_max_geodes(blueprint: &Blueprint, max_turns: i32) -> i32 {
    #[derive(Debug, Clone, Eq, PartialEq, Hash, PartialOrd, Ord)]
    struct State {
        stores: Vec<i32>,
        robots: Vec<i32>,
    }

    fn is_better_than(s1: &State, s2: &State) -> bool {
        return (0..4)
            .any(|idx| s1.stores[idx] > s2.stores[idx] || s1.robots[idx] > s2.robots[idx])
            && (0..4)
                .all(|idx| s1.stores[idx] >= s2.stores[idx] && s1.robots[idx] >= s2.robots[idx]);
    }

    fn recur(
        s: State,
        blueprint: &Blueprint,
        turns: i32,
        cache: &mut HashMap<State, i32>,
        better: &mut Vec<Vec<State>>,
    ) -> (i32, Vec<usize>, i32) {
        if turns == 0 {
            // println!("Turn zero for {:?}", s);
            return (s.stores[GEODE], Vec::new(), 1);
        }
        if turns == 1 {
            return (s.stores[GEODE] + s.robots[GEODE], vec![NOTHING], 1);
        }

        if let Some(cache_turns) = cache.get(&s) {
            if *cache_turns >= turns {
                // println!("State {:?} is cached", s);
                return (-1, Vec::new(), 1);
            }
        }

        cache.insert(s.clone(), turns);
/*
        for t in 0..(better.len() - turns as usize + 1) {
            for u in 0..better[t].len() {
                if is_better_than(&better[t][u], &s) {
                    // println!("State {:?} is worse than existing state (turn {}) {:?}", s, better.len() - t, better[t][u]);
                    return (-1, Vec::new(), 1);
                }
            }
        }

        // if we didn't peace out, see if we're better than anything in the current turn
        let b_idx = better.len() - turns as usize;
        better[b_idx].retain(|b| !is_better_than(&s, b));
        better[b_idx].push(s.clone());
*/

        // println!("Exporing {:?} - {}", s, turns);
        let mut best_geodes = 0;
        let mut best_path = vec![];
        let mut explored = 1;

        let mut recur_state = s.clone();

        for rt in vec![GEODE, OBSIDIAN, CLAY, ORE, NOTHING].into_iter() {
            if rt < 4 {
                if turns == 1 || turns == 2 && rt != GEODE || turns == 3 && rt == CLAY {
                    // no point in building anything on the last turn?
                    continue;
                }

                if !(0..4).all(|i| recur_state.stores[i] >= blueprint.costs[rt][i]) {
                    continue;
                }
                if rt == OBSIDIAN || rt == CLAY {
                    let max_needed = blueprint.costs.iter().map(|tc| tc[rt]).max().unwrap();
                    if recur_state.stores[rt] > max_needed * 2 {
                        continue;
                    }
                }
                if rt == ORE {
                    let max_needed = blueprint.costs.iter().map(|tc| tc[rt]).max().unwrap();
                    let eog_needed = (max_needed - recur_state.robots[rt]) * turns;
                    if recur_state.stores[rt] > eog_needed {
                        continue;
                    }
                }
                for i in 0..4 {
                    recur_state.stores[i] -= blueprint.costs[rt][i];
                    recur_state.stores[i] += recur_state.robots[i];
                }
                recur_state.robots[rt] += 1;
            } else {
                for i in 0..4 {
                    recur_state.stores[i] += recur_state.robots[i];
                }
            }

            let (recur_geodes, recur_path, recur_explored) = recur(
                recur_state.clone(),
                blueprint,
                turns - 1,
                cache,
                better,
            );
            if recur_geodes > best_geodes {
                best_geodes = recur_geodes;
                best_path = recur_path;
                best_path.insert(0, rt);
            }
            explored += recur_explored;

            // unwind changes
            if rt < 4 {
                recur_state.robots[rt] -= 1;
                for i in 0..4 {
                    recur_state.stores[i] -= recur_state.robots[i];
                    recur_state.stores[i] += blueprint.costs[rt][i];
                }
            } else {
                for i in 0..4 {
                    recur_state.stores[i] -= recur_state.robots[i];
                }
            }
        }

        return (best_geodes, best_path, explored);
    }

    let mut cache = HashMap::new();
    let mut better = (0..max_turns)
        .map(|_| Vec::new())
        .collect::<Vec<Vec<State>>>();
    let (geode_count, best_path, explored) = recur(
        State { stores: vec![0, 0, 0, 0], robots: vec![1, 0, 0, 0] },
        &blueprint,
        max_turns,
        &mut cache,
        &mut better,
    );
    let best_path = best_path.into_iter().map(|rt|
        match rt {
            ORE => "build ore",
            CLAY => "build clay",
            OBSIDIAN => "build obsidian",
            GEODE => "build geode",
            NOTHING => "build nothing",
            _ => "build banana",
        }).collect::<Vec<&str>>();
    println!(
        "Final best path for {} (explored {}): {} {:?}",
        blueprint.id, explored, geode_count, best_path
    );
    return geode_count;
}

impl BaseDay for Day19 {
    fn parse(&mut self, input: &mut InputReader) {
        fn parse_line(line: String) -> Blueprint {
            let sep = regex!(r#"\s*[:.]\s*"#);
            let ws = sep.split(line.trim()).collect::<Vec<&str>>();

            Blueprint {
                id: parse_regexp(
                    ws[0],
                    regex!(r#"\s*Blueprint (\d+)"#),
                    &mut |c: Captures| c[1].parse::<i32>().unwrap(),
                ),
                costs: vec![
                    // we make some assumptions about the costs, but we're hard-coding
                    // them so our parsing will break if our assumptions are wrong
                    parse_regexp(
                        ws[1],
                        regex!(r#"\s*Each ore robot costs (\d+) ore"#),
                        &mut |c: Captures| vec![c[1].parse::<i32>().unwrap(), 0, 0, 0],
                    ),
                    parse_regexp(
                        ws[2],
                        regex!(r#"\s*Each clay robot costs (\d+) ore"#),
                        &mut |c: Captures| vec![c[1].parse::<i32>().unwrap(), 0, 0, 0],
                    ),
                    parse_regexp(
                        ws[3],
                        regex!(r#"\s*Each obsidian robot costs (\d+) ore and (\d+) clay"#),
                        &mut |c: Captures| {
                            vec![
                                c[1].parse::<i32>().unwrap(),
                                c[2].parse::<i32>().unwrap(),
                                0,
                                0,
                            ]
                        },
                    ),
                    parse_regexp(
                        ws[4],
                        regex!(r#"\s*Each geode robot costs (\d+) ore and (\d+) obsidian"#),
                        &mut |c: Captures| {
                            vec![
                                c[1].parse::<i32>().unwrap(),
                                0,
                                c[2].parse::<i32>().unwrap(),
                                0,
                            ]
                        },
                    ),
                ],
            }
        }

        self.vals = parse_lines(input, &mut parse_line);
    }

    fn pt1(&mut self) -> String {
        let quality: i32 = self
            .vals
            .iter()
            .map(|b| b.id * compute_max_geodes(&b, 24))
            .sum();
        return quality.to_string();
    }

    fn pt2(&mut self) -> String {
        let quality: i32 = self
            .vals
            .iter()
            .take(3)
            .map(|b| compute_max_geodes(&b, 32))
            .fold(1, |tot, v| tot * v as i32);
        return quality.to_string();
    }
}

fn main() {
    let mut day = Day19 { vals: Vec::new() };
    run_day(&mut day);
}
