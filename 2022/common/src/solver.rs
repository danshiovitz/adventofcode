use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;

pub type FlagSet = i32;
pub type Trid = usize;

pub struct FlagManager {
    names: HashMap<String, Trid>,
}

impl FlagManager {
    pub fn from<'a, I: Iterator<Item = &'a str>>(names: I) -> FlagManager
    where
        I: IntoIterator<Item = &'a str>,
    {
        let name_map = names
            .enumerate()
            .map(|(idx, n)| (n.to_owned(), idx))
            .collect();
        return FlagManager { names: name_map };
    }

    pub fn init(&self) -> FlagSet {
        let ret: FlagSet = 0;
        return ret;
    }

    pub fn set_name(&self, val: &mut FlagSet, name: &str) {
        let id = *self.names.get(name).unwrap();
        return self.set(val, id);
    }

    pub fn set(&self, val: &mut FlagSet, id: Trid) {
        let bf = 1 << id;
        *val |= bf;
    }

    pub fn get_name(&self, val: &FlagSet, name: &str) -> bool {
        let id = *self.names.get(name).unwrap();
        return self.get(val, id);
    }

    pub fn get(&self, val: &FlagSet, id: Trid) -> bool {
        let bf = 1 << id;
        return *val & bf != 0;
    }

    pub fn translate(&self, name: &str) -> Trid {
        return *self.names.get(name).unwrap();
    }

    pub fn translate_back(&self, idx: Trid) -> String {
        return self
            .names
            .iter()
            .filter_map(|(k, v)| if idx == *v { Some(k.clone()) } else { None })
            .next()
            .unwrap();
    }
}

// return the minimum cost to reach a state for which is_finished is true, where transitions
// from each state are given by gen_possible_moves
pub fn cost_minimizing_dfs<S>(solver: &dyn SolverBase<S>, start_state: &S) -> i32
where
    S: SolverState,
{
    let mut cache = HashMap::new();
    return cost_minimizing_dfs_recur(solver, start_state, 0, solver.max_cost(), &mut cache);
}

fn cost_minimizing_dfs_recur<S>(
    solver: &dyn SolverBase<S>,
    state: &S,
    cur_cost: i32,
    best_cost: i32,
    cache: &mut HashMap<S, (i32, i32)>,
) -> i32
where
    S: SolverState,
{
    if solver.is_finished(state) {
        println!("Found finish state with cost {}", cur_cost);
        return cur_cost;
    }

    if let Some((cost_to, total)) = cache.get(state) {
        if *cost_to <= cur_cost {
            return *total;
        }
    }

    if solver.is_verbose() {
        solver.print_state(state);
    }

    let mut possible_moves = solver.gen_possible_moves(state);
    possible_moves.sort();
    if solver.is_verbose() {
        println!("For {:?}, generated {} moves", state, possible_moves.len());
    }

    if solver.is_verbose() && possible_moves.is_empty() {
        println!("Stuck:");
        solver.print_state(state);
    }

    let mut pbest = best_cost;
    for (pcost, pstate) in &possible_moves {
        let pcur = cur_cost + *pcost;
        if pcur > pbest {
            continue;
        }
        let ptot = cost_minimizing_dfs_recur(solver, pstate, pcur, pbest, cache);
        if ptot <= pbest {
            pbest = ptot;
        }
    }

    cache.insert(state.clone(), (cur_cost, pbest));

    return pbest;
}

pub fn count_all_paths_dfs<S>(solver: &dyn SolverBase<S>, start_state: &S) -> i32
where
    S: SolverState,
{
    let mut cache = HashMap::new();
    return count_all_paths_dfs_recur(solver, start_state, &mut cache);
}

fn count_all_paths_dfs_recur<S>(
    solver: &dyn SolverBase<S>,
    state: &S,
    cache: &mut HashMap<S, i32>,
) -> i32
where
    S: SolverState,
{
    if solver.is_finished(state) {
        return 1;
    }

    if let Some(count) = cache.get(state) {
        return *count;
    }

    if solver.is_verbose() {
        solver.print_state(state);
    }

    let possible_moves = solver.gen_possible_moves(state);
    // don't need to sort here since we're trying everything
    if solver.is_verbose() {
        println!("For {:?}, generated {} moves", state, possible_moves.len());
    }

    if solver.is_verbose() && possible_moves.is_empty() {
        println!("Stuck:");
        solver.print_state(state);
    }

    let mut count = 0;
    for (_pcost, pstate) in &possible_moves {
        count += count_all_paths_dfs_recur(solver, pstate, cache);
    }

    cache.insert(state.clone(), count);

    return count;
}

// TODO: make this a-star
pub fn cost_minimizing_bfs<S>(solver: &dyn SolverBase<S>, start_state: &S) -> i32
where
    S: SolverState,
{
    let mut working = Vec::new();
    working.push((start_state.clone(), 0));

    let mut visited = HashMap::new();

    while !working.is_empty() {
        let (cur, cost) = working.remove(0);
        if solver.is_finished(&cur) {
            return cost;
        }
        if let Some(existing_cost) = visited.get(&cur) {
            if cost >= *existing_cost {
                continue;
            }
        }
        visited.insert(cur.clone(), cost);

        let possible_moves = solver.gen_possible_moves(&cur);
        if solver.is_verbose() {
            println!("For {:?}, generated {} moves", cur, possible_moves.len());
        }
        working.extend(possible_moves.into_iter().map(|(c, s)| (s, c + cost)));
    }

    return solver.cant_solve();
}

pub trait SolverBase<S> {
    fn is_finished(&self, state: &S) -> bool;
    fn print_state(&self, state: &S) -> ();
    // returns a list of (cost, new state) pairs
    fn gen_possible_moves(&self, state: &S) -> Vec<(i32, S)>;
    fn is_verbose(&self) -> bool;
    fn max_cost(&self) -> i32 {
        return i32::MAX - 1;
    }
    fn cant_solve(&self) -> i32 {
        panic!("Couldn't find solution?");
    }
}

pub trait SolverState: Eq + Hash + Ord + PartialOrd + Debug + Clone {}
