extern crate common;

use common::framework::{parse_numbers, run_day, BaseDay, InputReader};

struct Day07 {
    vals: Vec<i32>,
}

fn pick_best<F>(vals: &Vec<i32>, cost_calc: &mut F) -> String
where
    F: FnMut(i32, i32) -> i32,
{
    let min_v: i32 = *vals.iter().min().unwrap();
    let max_v: i32 = *vals.iter().max().unwrap();

    struct C {
        lvl: i32,
        cost: i32,
    }
    let mut cost_for = |lvl: i32| vals.iter().map(|v| cost_calc(*v, lvl)).sum::<i32>();
    let mut costs = (min_v..=max_v)
        .map(|v| C { lvl: v, cost: cost_for(v) })
        .collect::<Vec<C>>();
    costs.sort_by(|a, b| a.cost.partial_cmp(&b.cost).unwrap());
    for c in &costs {
        println!("C: lvl={} cost={}", c.lvl, c.cost);
    }
    return costs[0].cost.to_string();
}

impl BaseDay for Day07 {
    fn parse(&mut self, input: &mut InputReader) {
        self.vals = parse_numbers(input);
    }

    fn pt1(&mut self) -> String {
        let mut cost_calc = |v: i32, lvl: i32| (v - lvl).abs();
        return pick_best(&self.vals, &mut cost_calc);
    }

    fn pt2(&mut self) -> String {
        let mut cost_calc = |v: i32, lvl: i32| (v - lvl).abs() * ((v - lvl).abs() + 1) / 2;
        return pick_best(&self.vals, &mut cost_calc);
    }
}

fn main() {
    let mut day = Day07 { vals: Vec::new() };
    run_day(&mut day);
}
