use std::collections::HashSet;

// https://imgur.com/a/LL4sQmq

#[derive(Debug, Eq, PartialEq)]
enum HAT { FLOPPY, DURAG, WIG, TOP, BOATER }

#[derive(Debug, Eq, PartialEq)]
enum NECK { NONE, NECKLACE, BOWTIE, RIBBON, TIE, MONOCLE }

#[derive(Debug, Eq, PartialEq)]
enum BODY { PURPLE, WHITE, SKY, BLUE, ORANGE }

#[derive(Debug, Eq, PartialEq)]
enum HELD { SCYTHE, BASKET, RAKE, GUITAR, BROOM }

impl HAT {
    fn to_char(&self) -> char {
        match self {
            HAT::FLOPPY => 'F',
            HAT::DURAG => 'D',
            HAT::WIG => 'W',
            HAT::TOP => 'T',
            HAT::BOATER => 'B',
        }
    }
}

impl NECK {
    fn to_char(&self) -> char {
        match self {
            NECK::NONE => '.',
            NECK::NECKLACE => 'N',
            NECK::BOWTIE => 'B',
            NECK::RIBBON => 'R',
            NECK::TIE => 'T',
            NECK::MONOCLE => 'M',
        }
    }
}

impl BODY {
    fn to_char(&self) -> char {
        match self {
            BODY::PURPLE => 'P',
            BODY::WHITE => 'W',
            BODY::SKY => 'S',
            BODY::BLUE => 'B',
            BODY::ORANGE => 'O',
        }
    }
}

impl HELD {
    fn to_char(&self) -> char {
        match self {
            HELD::SCYTHE => 'S',
            HELD::BASKET => 'B',
            HELD::RAKE => 'R',
            HELD::GUITAR => 'G',
            HELD::BROOM => 'M',
        }
    }
}

struct Doll {
    hat: HAT,
    neck: NECK,
    body: BODY,
    held: HELD,
}

impl Doll {
    fn to_string(&self) -> String {
        let v = vec![self.hat.to_char(), self.neck.to_char(), self.body.to_char(), self.held.to_char()];
        return v.into_iter().collect::<String>();
    }
}

fn make_slot_neighbors() -> Vec<Vec<usize>> {
    return vec![
        vec![1, 5],
        vec![0, 2, 6],
        vec![1, 3, 7],
        vec![2, 4, 8],
        vec![3, 9],
        vec![0, 6, 10],
        vec![1, 5, 7, 11],
        vec![2, 6, 8, 12],
        vec![3, 7, 9, 13],
        vec![4, 8, 14],
        vec![5, 11],
        vec![6, 10, 12],
        vec![7, 11, 13],
        vec![8, 12, 14],
        vec![9, 13],
    ];
}

fn make_dolls() -> Vec<Doll> {
    return vec![
        Doll { hat: HAT::FLOPPY, neck: NECK::NONE, body: BODY::PURPLE, held: HELD::SCYTHE },
        Doll { hat: HAT::DURAG, neck: NECK::NECKLACE, body: BODY::PURPLE, held: HELD::BASKET },
        Doll { hat: HAT::WIG, neck: NECK::BOWTIE, body: BODY::WHITE, held: HELD::RAKE },
        Doll { hat: HAT::TOP, neck: NECK::NECKLACE, body: BODY::SKY, held: HELD::GUITAR },
        Doll { hat: HAT::FLOPPY, neck: NECK::RIBBON, body: BODY::WHITE, held: HELD::BROOM },

        Doll { hat: HAT::WIG, neck: NECK::MONOCLE, body: BODY::BLUE, held: HELD::BROOM },
        Doll { hat: HAT::WIG, neck: NECK::NECKLACE, body: BODY::ORANGE, held: HELD::RAKE },
        Doll { hat: HAT::BOATER, neck: NECK::BOWTIE, body: BODY::PURPLE, held: HELD::SCYTHE },
        Doll { hat: HAT::BOATER, neck: NECK::BOWTIE, body: BODY::ORANGE, held: HELD::SCYTHE },
        Doll { hat: HAT::TOP, neck: NECK::MONOCLE, body: BODY::WHITE, held: HELD::BASKET },

        Doll { hat: HAT::BOATER, neck: NECK::RIBBON, body: BODY::BLUE, held: HELD::GUITAR },
        Doll { hat: HAT::FLOPPY, neck: NECK::MONOCLE, body: BODY::SKY, held: HELD::RAKE },
        Doll { hat: HAT::DURAG, neck: NECK::RIBBON, body: BODY::ORANGE, held: HELD::BASKET },
        Doll { hat: HAT::DURAG, neck: NECK::TIE, body: BODY::SKY, held: HELD::GUITAR },
        Doll { hat: HAT::TOP, neck: NECK::TIE, body: BODY::BLUE, held: HELD::BROOM },
    ]
}

fn print_assignments(assignments: &Vec<usize>, dolls: &Vec<Doll>) {
    for idx in 0..15 {
        print!("{}", dolls[assignments[idx]].to_string());
        if (idx + 1) % 5 == 0 {
            println!();
        } else {
            print!("  ");
        }
    }
}

fn can_neighbor(x: &Doll, y: &Doll) -> bool {
    return x.hat == y.hat || x.neck == y.neck || x.body == y.body || x.held == y.held;
}

fn compute_assignments(dolls: &Vec<Doll>, neighbors: &Vec<Vec<usize>>) -> Vec<Vec<usize>> {
    let slot_order = vec![6, 11, 10, 5, 0, 1, 2, 7, 12, 13, 8, 3, 4, 9, 14];

    fn recur(slot_assigns: &mut Vec<usize>, doll_assigns: &mut Vec<usize>, slot_order_idx: usize, dolls: &Vec<Doll>, neighbors: &Vec<Vec<usize>>, slot_order: &Vec<usize>, solutions: &mut Vec<Vec<usize>>) {
        if slot_order_idx == slot_order.len() {
            solutions.push(slot_assigns.clone());
            return;
        }

        let slot = slot_order[slot_order_idx];
        'outer: for doll in 0..dolls.len() {
            if doll_assigns[doll] != usize::MAX {
                continue;
            }
            for &ngh_slot in neighbors[slot].iter() {
                if slot_assigns[ngh_slot] != usize::MAX {
                    let ngh_doll = slot_assigns[ngh_slot];
                    if !can_neighbor(&dolls[doll], &dolls[ngh_doll]) {
                        continue 'outer;
                    }
                }
            }

            // legal assignment:
            slot_assigns[slot] = doll;
            doll_assigns[doll] = slot;
            recur(slot_assigns, doll_assigns, slot_order_idx + 1, dolls, neighbors, slot_order, solutions);
            doll_assigns[doll] = usize::MAX;
            slot_assigns[slot] = usize::MAX;
        }
    }

    let mut slot_assigns = vec![usize::MAX; dolls.len()];
    let mut doll_assigns = vec![usize::MAX; dolls.len()];
    let mut solutions = vec![];
    recur(&mut slot_assigns, &mut doll_assigns, 0, dolls, neighbors, &slot_order, &mut solutions);
    println!("Finished with {} solutions.", solutions.len());
    return solutions;
}

fn to_digit(val: &Vec<usize>) -> u64 {
    let mut ret = 0;
    for &v in val.iter() {
        ret *= val.len() as u64;
        ret += v as u64;
    }
    return ret;
}

fn from_digit(val: u64, sz: usize) -> Vec<usize> {
    let sz = sz as u64;
    let mut cur = val;
    let mut ret = vec![];
    while ret.len() < sz as usize {
        ret.insert(0, (cur % sz) as usize);
        cur /= sz;
    }
    if cur != 0 {
        panic!("Didn't hit zero? {}", cur);
    }
    return ret;
}

fn digit_swap(val: u64, i: usize, j: usize, sz: usize) -> u64 {
    let sz = sz as u64;
    let mut ret = val;

    let i_down = val % (sz as u64).pow(i as u32 + 1);
    let i_digit;
    if i > 0 {
        let rest_range = (sz as u64).pow(i as u32);
        let i_rest = val % rest_range;
        let i_only = i_down - i_rest;
        ret -= i_only;
        i_digit = i_rest / rest_range;
    } else {
        ret -= i_down;
        i_digit = i_down;
    }

    let j_down = val % (sz as u64).pow(j as u32 + 1);
    let j_digit;
    if j > 0 {
        let rest_range = (sz as u64).pow(j as u32);
        let j_rest = val % rest_range;
        let j_only = j_down - j_rest;
        ret -= j_only;
        j_digit = j_rest / rest_range;
    } else {
        ret -= j_down;
        j_digit = j_down;
    }

    ret += i_digit * (sz as u64).pow(j as u32);
    ret += j_digit * (sz as u64).pow(i as u32);
    return ret;
}

fn compute_min_dist(start: Vec<usize>, solutions: &Vec<Vec<usize>>, dolls: &Vec<Doll>) {
    let mut ends = HashSet::new();
    for sol in solutions.iter() {
        ends.insert(to_digit(sol));
    }
    let ends = ends;

    let mut working = vec![(to_digit(&start), 0)];
    let mut seen = HashSet::new();

    while working.len() > 0 {
        let cur = working.remove(0);
        if ends.contains(&cur.0) {
            println!();
            println!("Found a match in {} swaps:", cur.1);
            print_assignments(&from_digit(cur.0, start.len()), dolls);
            return;
        }
        if seen.contains(&cur.0) {
            continue;
        }
        if working.len() % 20 == 0 {
            println!("At dist: {}, {} states", cur.1, working.len());
        }
        seen.insert(cur.0);
        for i in 0..(start.len()) {
            for j in (i+1)..(start.len()) {
                let cpy = digit_swap(cur.0, i, j, start.len());
                working.push((cpy, cur.1 + 1));
            }
        }
    }

    println!("Couldn't find anything!");
}

fn count_swaps(from: &Vec<usize>, rev_from: &Vec<usize>, to: &Vec<usize>) -> i32 {
    let mut seen = from.iter().map(|_| false).collect::<Vec<bool>>();
    let mut ret = 0;
    for idx in 0..from.len() {
        if seen[idx] {
            continue;
        }
        seen[idx] = true;

        // Figure out the "ring" that this value belongs to
        // example: from [0 1 2], to [2 0 1]
        // first look at the 0 index, we see 2 and expect 0, so we
        // look in the slot that 2 should be in (which is also 2)
        // and find 1, so we look in the slot that 1 should be in (1)
        // and find 0, so we've reached the beginning/end of the ring
        let expected = from[idx];
        let mut cur = to[idx];
        while cur != expected {
            seen[rev_from[cur]] = true;
            ret += 1;
            cur = to[rev_from[cur]];
        }
    }
    return ret;
}

fn compute_min_dist2(start: &Vec<usize>, solutions: &Vec<Vec<usize>>, _dolls: &Vec<Doll>) {
    let mut best_cnt = i32::MAX;
    let mut best_sol = start.clone();
    let mut rev_start = start.iter().enumerate().map(|(idx, val)| (*val, idx)).collect::<Vec<(usize, usize)>>();
    rev_start.sort();
    let rev_start = rev_start.into_iter().map(|(val, idx)| idx).collect::<Vec<usize>>();

    for sol in solutions.iter() {
        let cnt = count_swaps(start, &rev_start, sol);
        if cnt < best_cnt {
            best_cnt = cnt;
            best_sol = sol.clone();
        }
    }
    println!("Best is {} swaps: {:?}", best_cnt, best_sol);
}

fn main() {
    let dolls = make_dolls();
    let slot_neighbors = make_slot_neighbors();
    // let assignments = (0..15).collect::<Vec<usize>>();
    // print_assignments(&assignments, &dolls);
    let solutions = compute_assignments(&dolls, &slot_neighbors);
    compute_min_dist2(&(0..15).collect::<Vec<usize>>(), &solutions, &dolls);
}
