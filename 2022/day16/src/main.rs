use lazy_regex::regex;
use std::collections::HashMap;

extern crate common;

use common::framework::{parse_lines, parse_vals, run_day, BaseDay, InputReader};

struct Node {
    name: String,
    max_flow: i32,
    neighbors: Vec<String>,
}

struct Day16 {
    vals: Vec<Node>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Info {
    name: String,
    max_flow: i32,
    travel_costs: Vec<i32>,
}

fn make_infos(nodes: &Vec<Node>) -> Vec<Info> {
    let mut name_map = HashMap::new();
    name_map.insert("AA", 0);
    let mut graph = HashMap::new();

    for node in nodes {
        if node.max_flow > 0 {
            name_map.insert(node.name.as_str(), name_map.len());
        }
        graph.insert(node.name.as_str(), &node.neighbors);
    }

    let mut travel_costs = HashMap::new();
    for node in nodes {
        if !name_map.contains_key(node.name.as_str()) {
            continue;
        }
        let mut costs = HashMap::new();
        costs.reserve(nodes.len());

        let mut working = vec![(node.name.as_str(), 0)];
        while working.len() > 0 {
            let cur = working.remove(0);
            if costs.contains_key(&cur.0) {
                continue;
            }
            costs.insert(cur.0, cur.1);
            for ngh in graph.get(&cur.0).unwrap().iter() {
                working.push((ngh.as_str(), cur.1 + 1));
            }
        }

        let mut id_costs = costs
            .into_iter()
            .filter_map(|(n, c)| {
                if let Some(id) = name_map.get(n) {
                    Some((*id, c))
                } else {
                    None
                }
            })
            .collect::<Vec<(usize, i32)>>();
        id_costs.sort();
        travel_costs.insert(
            node.name.as_str(),
            id_costs.into_iter().map(|c| c.1).collect::<Vec<i32>>(),
        );
    }

    let mut infos = Vec::new();
    infos.resize(
        name_map.len(),
        Info { name: "".to_string(), max_flow: 0, travel_costs: Vec::new() },
    );
    for node in nodes {
        if let Some(id) = name_map.get(node.name.as_str()) {
            infos[*id] = Info {
                name: node.name.clone(),
                max_flow: node.max_flow,
                travel_costs: travel_costs.get(node.name.as_str()).unwrap().clone(),
            };
        }
    }

    return infos;
}

fn release_max_pressure(nodes: &Vec<Node>, max_cost: i32, num_actors: i32) -> i32 {
    let infos = make_infos(nodes);

    #[derive(Debug, Clone)]
    struct Actor {
        loc: usize,
        delay: i32,
    }
    #[derive(Debug, Clone)]
    struct State {
        actors: Vec<Actor>,
        remaining_cost: i32,
        active_flow: i32,
        visited: Vec<bool>,
    }

    fn recur(s: &mut State, infos: &Vec<Info>) -> (i32, Vec<Vec<String>>) {
        let mut best_flow = 0;
        let mut best_paths = s
            .actors
            .iter()
            .map(|_| Vec::new())
            .collect::<Vec<Vec<String>>>();

        let mut travel_flow = 0;
        let mut turns = 0;
        let old_active_flow = s.active_flow;

        while s.actors.iter().all(|a| a.delay > 0) && s.remaining_cost > 0 {
            travel_flow += s.active_flow;
            s.remaining_cost -= 1;
            turns += 1;

            for actor in s.actors.iter_mut() {
                actor.delay -= 1;
                if actor.delay == 0 {
                    s.active_flow += infos[actor.loc].max_flow;
                }
            }
        }

        for a in 0..s.actors.len() {
            if s.actors[a].delay > 0 {
                continue;
            }

            // Pick a new thing to do
            let mut did_something = false;
            for i in 1..infos.len() {
                if i == s.actors[a].loc || s.visited[i] {
                    continue;
                }

                // costs are one higher because we also pay to turn on the valve
                let travel_cost = infos[s.actors[a].loc].travel_costs[i] + 1;
                if travel_cost > s.remaining_cost {
                    continue;
                }

                did_something = true;

                let old_loc = s.actors[a].loc;
                s.actors[a].loc = i;
                s.actors[a].delay = travel_cost;
                // we mark it visited immediately so other actors won't try to go there
                s.visited[i] = true;

                let (recur_flow, recur_paths) = recur(s, infos);
                let recur_flow = travel_flow + recur_flow;
                if recur_flow > best_flow {
                    best_flow = recur_flow;
                    best_paths = recur_paths;
                    best_paths[a]
                        .insert(0, format!("{} minutes to {}", travel_cost, infos[i].name));
                }

                // now undo the previous moves
                s.visited[i] = false;
                s.actors[a].delay = 0;
                s.actors[a].loc = old_loc;
            }

            if !did_something {
                let old_loc = s.actors[a].loc;
                s.actors[a].loc = 0;
                s.actors[a].delay = s.remaining_cost;

                let (recur_flow, recur_paths);
                if s.remaining_cost > 0 {
                    (recur_flow, recur_paths) = recur(s, infos);
                } else {
                    (recur_flow, recur_paths) = (
                        0,
                        s.actors
                            .iter()
                            .map(|_| Vec::new())
                            .collect::<Vec<Vec<String>>>(),
                    );
                }
                let recur_flow = travel_flow + recur_flow;
                if recur_flow > best_flow {
                    best_flow = recur_flow;
                    best_paths = recur_paths;
                    if s.remaining_cost > 0 {
                        best_paths[a].insert(0, "...".to_string());
                    }
                }

                // now undo the previous moves
                s.actors[a].delay = 0;
                s.actors[a].loc = old_loc;
            }

            // undo travel moves:
            s.remaining_cost += turns;
            for actor in s.actors.iter_mut() {
                actor.delay += turns;
            }
            s.active_flow = old_active_flow;

            return (best_flow, best_paths);
        }

        panic!("Something weird happened!");
    }

    let (best_flow, best_paths) = recur(
        &mut State {
            actors: (0..num_actors)
                .map(|_| Actor { loc: 0, delay: 0 })
                .collect::<Vec<Actor>>(),
            remaining_cost: max_cost,
            active_flow: 0,
            visited: infos.iter().map(|n| n.name == "AA").collect::<Vec<bool>>(),
        },
        &infos,
    );
    println!("Final best paths: {} {:?}", best_flow, best_paths);
    return best_flow;
}

impl BaseDay for Day16 {
    fn parse(&mut self, input: &mut InputReader) {
        fn parse_line(line: String) -> Node {
            match regex!(r#"Valve (\S+) has flow rate=(\d+); tunnels? leads? to valves? (.*)"#)
                .captures(&line)
            {
                Some(c) => Node {
                    name: c[1].to_string(),
                    max_flow: c[2].parse::<i32>().unwrap(),
                    neighbors: parse_vals(&c[3]),
                },
                None => {
                    panic!("Bad line: {}", line);
                }
            }
        }

        self.vals = parse_lines(input, &mut parse_line);
    }

    fn pt1(&mut self) -> String {
        let best_flow = release_max_pressure(&self.vals, 30, 1);
        return best_flow.to_string();
    }

    fn pt2(&mut self) -> String {
        let best_flow = release_max_pressure(&self.vals, 26, 2);
        return best_flow.to_string();
    }
}

fn main() {
    let mut day = Day16 { vals: Vec::new() };
    run_day(&mut day);
}
