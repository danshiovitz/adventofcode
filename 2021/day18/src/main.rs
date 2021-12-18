use std::boxed::Box;

#[derive(PartialEq, Eq, Clone, Debug)]
enum SnumberType {
    Literal(i32),
    Pair(Box<Snumber>),
}

#[derive(PartialEq, Eq, Clone, Debug)]
struct Snumber {
    left: SnumberType,
    right: SnumberType,
}

extern crate common;

use common::framework::{parse_lines, run_day, BaseDay, InputReader};

struct Day18 {
    vals: Vec<Snumber>,
}

fn parse_line(line: String) -> Snumber {
    let mut chars = line.chars().filter(|c| *c != ' ').collect();
    let ret = parse_line_recur(&mut chars);
    if chars.len() != 0 {
        panic!("Characters remaining at end {}: {}", chars.len(), line);
    }
    return ret;
}

fn expect(e: char, chs: &mut Vec<char>) {
    if chs.len() > 0 {
        let ch = chs.remove(0);
        if ch != e {
            panic!("Expected {}, found {}", e, ch);
        }
    } else {
        panic!("Nothing left in line?");
    }
}

fn peek_for(e: char, chs: &Vec<char>) -> bool {
    if chs.len() == 0 {
        panic!("Nothing left in line?");
    } else {
        return chs[0] == e;
    }
}

fn parse_line_recur(chs: &mut Vec<char>) -> Snumber {
    expect('[', chs);
    let left: SnumberType;
    if peek_for('[', chs) {
        left = SnumberType::Pair(Box::new(parse_line_recur(chs)));
    } else {
        let c = chs.remove(0);
        left = SnumberType::Literal(c.to_digit(10).unwrap() as i32);
    }
    expect(',', chs);
    let right: SnumberType;
    if peek_for('[', chs) {
        right = SnumberType::Pair(Box::new(parse_line_recur(chs)));
    } else {
        let c = chs.remove(0);
        right = SnumberType::Literal(c.to_digit(10).unwrap() as i32);
    }
    expect(']', chs);
    return Snumber { left: left, right: right };
}

fn add(left: &Snumber, right: &Snumber) -> Snumber {
    let mut cur = Snumber { left: SnumberType::Pair(Box::new(left.clone())), right: SnumberType::Pair(Box::new(right.clone())) };
    loop {
        // print_snumber(&cur, false);
        if let Some((exploded, _left_to, _right_to)) = try_explode(&cur, 4) {
            cur = exploded.unwrap();
            // if _left_to.is_some() {
            //     println!("Silently discarding left {}", _left_to.unwrap());
            // }
            // if _right_to.is_some() {
            //     println!("Silently discarding right {}", _right_to.unwrap());
            // }
            continue;
        } else if let Some(split) = try_split(&cur) {
            cur = split;
            continue;
        } else {
            return cur;
        }
    }
}

fn add_all(vals: &Vec<Snumber>) -> Snumber {
    let mut cur = vals[0].clone();
    for nxt in &vals[1..vals.len()] {
        cur = add(&cur, &nxt);
    }
    return cur;
}

fn try_explode(val: &Snumber, depth: i32) -> Option<(Option<Snumber>, Option<i32>, Option<i32>)> {
    if depth <= 0 {
        let left = match val.left {
            SnumberType::Literal(num) => num,
            _ => panic!("Bad left for explode"),
        };
        let right = match val.right {
            SnumberType::Literal(num) => num,
            _ => panic!("Bad right for explode"),
        };
        return Some((None, Some(left), Some(right)));
    }

    if let Some((new_val, left_to, right_to)) = try_explode_single(&val.left, depth - 1) {
        // our left exploded, therefore we can't handle left_to ourselves,
        // but we might be able to handle right_to
        if right_to.is_some() {
            let new_right = stick_it(right_to.unwrap(), &val.right, true);
            return Some((Some(Snumber { left: new_val, right: new_right }), left_to, None));
        }
        return Some((Some(Snumber { left: new_val, right: val.right.clone() }), left_to, right_to));
    } else if let Some((new_val, left_to, right_to)) = try_explode_single(&val.right, depth - 1) {
        // As above, we can't handle right, maybe can handle left
        if left_to.is_some() {
            let new_left = stick_it(left_to.unwrap(), &val.left, false);
            return Some((Some(Snumber { left: new_left, right: new_val }), None, right_to));
        }
        return Some((Some(Snumber { left: val.left.clone(), right: new_val }), left_to, right_to));
    } else {
        return None;
    }
}

fn stick_it(amt: i32, val: &SnumberType, on_left: bool) -> SnumberType {
    return match val {
        SnumberType::Literal(num) => {
            SnumberType::Literal(num + amt)
        },
        SnumberType::Pair(ptr) => {
            if on_left {
                SnumberType::Pair(Box::new(Snumber { left: stick_it(amt, &ptr.left, true), right: ptr.right.clone() }))
            } else {
                SnumberType::Pair(Box::new(Snumber { left: ptr.left.clone(), right: stick_it(amt, &ptr.right, false) }))
            }
        }
    }
}

fn try_explode_single(val: &SnumberType, depth: i32) -> Option<(SnumberType, Option<i32>, Option<i32>)> {
    match val {
        SnumberType::Literal(_) => None,
        SnumberType::Pair(ptr) => {
            if let Some((new_val, left_to, right_to)) = try_explode(&*ptr, depth) {
                if new_val.is_none() {
                    return Some((SnumberType::Literal(0), left_to, right_to));
                } else {
                    return Some((SnumberType::Pair(Box::new(new_val.unwrap())), left_to, right_to));
                }
            } else {
                return None;
            }
        },
    }
}

fn try_split(val: &Snumber) -> Option<Snumber> {
    let ret = try_split_single(&val.left);
    if ret.is_some() {
        return Some(Snumber { left: ret.unwrap(), right: val.right.clone() });
    }

    let ret = try_split_single(&val.right);
    if ret.is_some() {
        return Some(Snumber { left: val.left.clone(), right: ret.unwrap() });
    }

    return None;
}

fn try_split_single(val: &SnumberType) -> Option<SnumberType> {
    return match val {
        SnumberType::Literal(num) => {
            let num = *num;
            if num >= 10 {
                let new_left = SnumberType::Literal(num / 2);
                let new_right = SnumberType::Literal(num / 2 + num % 2);
                Some(SnumberType::Pair(Box::new(Snumber { left: new_left, right: new_right })))
            } else {
                None
            }
        },
        SnumberType::Pair(ptr) => {
            let ret = try_split(&*ptr);
            if ret.is_some() {
                return Some(SnumberType::Pair(Box::new(ret.unwrap())));
            } else {
                return None;
            }
        }
    };
}

fn print_snumber(val: &Snumber, nested: bool) {
    print!("[");
    match &val.left {
        SnumberType::Literal(num) => print!("{}", num),
        SnumberType::Pair(ptr) => print_snumber(&*ptr, true),
    };
    print!(",");
    match &val.right {
        SnumberType::Literal(num) => print!("{}", num),
        SnumberType::Pair(ptr) => print_snumber(&*ptr, true),
    };
    if nested {
        print!("]");
    } else {
        println!("]");
    }
}

fn magnitude(val: &Snumber) -> i32 {
    let left = match &val.left {
        SnumberType::Literal(num) => *num,
        SnumberType::Pair(ptr) => magnitude(&*ptr),
    };
    let right = match &val.right {
        SnumberType::Literal(num) => *num,
        SnumberType::Pair(ptr) => magnitude(&*ptr),
    };
    return 3 * left + 2 * right;
}

impl BaseDay for Day18 {
    fn parse(&mut self, input: &mut InputReader) {
        self.vals = parse_lines(input, &mut parse_line);
    }

    fn pt1(&mut self) -> String {
        let sum = add_all(&self.vals);
        print_snumber(&sum, false);
        return magnitude(&sum).to_string();
    }

    fn pt2(&mut self) -> String {
        let mut most = 0;
        for i in 0..self.vals.len() {
            for j in 0..self.vals.len() {
                if i == j {
                    continue;
                }
                let sum = magnitude(&add(&self.vals[i], &self.vals[j]));
                if sum > most {
                    most = sum;
                }
            }
        }
        return most.to_string();
    }
}

fn main() {
    let mut day = Day18 { vals: Vec::new() };
    run_day(&mut day);
}
