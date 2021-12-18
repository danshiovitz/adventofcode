use std::rc::Rc;

enum Snumber {
    Literal(i32),
    Pair(Rc<Snumber>, Rc<Snumber>),
}

extern crate common;

use common::framework::{parse_lines, run_day, BaseDay, InputReader};

struct Day18 {
    vals: Vec<Rc<Snumber>>,
}

fn parse_line(line: String) -> Rc<Snumber> {
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

fn parse_line_recur(chs: &mut Vec<char>) -> Rc<Snumber> {
    if peek_for('[', chs) {
        expect('[', chs);
        let left = parse_line_recur(chs);
        expect(',', chs);
        let right = parse_line_recur(chs);
        expect(']', chs);
        return Rc::new(Snumber::Pair(left, right));
    } else {
        let c = chs.remove(0);
        return Rc::new(Snumber::Literal(c.to_digit(10).unwrap() as i32));
    }
}

fn render_snumber(val: &Rc<Snumber>) -> String {
    match &**val {
        Snumber::Literal(num) => return num.to_string(),
        Snumber::Pair(left, right) => {
            let mut ret = "[".to_owned();
            ret.push_str(&render_snumber(&left));
            ret.push(',');
            ret.push_str(&render_snumber(&right));
            ret.push(']');
            return ret;
        }
    }
}

fn print_snumber(val: &Rc<Snumber>) {
    println!("{}", render_snumber(val));
}

fn add_snumber(x: &Rc<Snumber>, y: &Rc<Snumber>, verbose: bool) -> Rc<Snumber> {
    let mut cur = Rc::new(Snumber::Pair(x.clone(), y.clone()));
    loop {
        if verbose {
            print_snumber(&cur);
        }
        if let Some((new_cur, left_spill, right_spill)) = try_explode(&cur, 4) {
            cur = new_cur;
            if verbose {
                if let Some(spill) = left_spill {
                    println!("Discarding left {}", spill);
                }
                if let Some(spill) = right_spill {
                    println!("Discarding right {}", spill);
                }
            }
        } else if let Some(new_cur) = try_split(&cur) {
            cur = new_cur;
        } else {
            break;
        }
    }
    return cur;
}

fn add_all(vals: &Vec<Rc<Snumber>>) -> Rc<Snumber> {
    if vals.len() == 0 {
        return Rc::new(Snumber::Literal(0));
    }
    let mut cur = vals[0].clone();
    for nxt in &vals[1..vals.len()] {
        cur = add_snumber(&cur, &nxt, false);
    }
    return cur;
}

fn try_explode(val: &Rc<Snumber>, depth: i32) -> Option<(Rc<Snumber>, Option<i32>, Option<i32>)> {
    match &**val {
        Snumber::Literal(_) => return None,
        Snumber::Pair(left, right) => {
            if depth <= 0 {
                let left_val = match &**left {
                    Snumber::Literal(num) => *num,
                    _ => panic!("Bad left pair elem to explode"),
                };
                let right_val = match &**right {
                    Snumber::Literal(num) => *num,
                    _ => panic!("Bad right pair elem to explode"),
                };
                return Some((
                    Rc::new(Snumber::Literal(0)),
                    Some(left_val),
                    Some(right_val),
                ));
            }
            if let Some((new_val, left_spill, right_spill)) = try_explode(&left, depth - 1) {
                let new_right = insert_to(right_spill, right, true);
                return Some((Rc::new(Snumber::Pair(new_val, new_right)), left_spill, None));
            } else if let Some((new_val, left_spill, right_spill)) = try_explode(&right, depth - 1)
            {
                let new_left = insert_to(left_spill, left, false);
                return Some((Rc::new(Snumber::Pair(new_left, new_val)), None, right_spill));
            } else {
                return None;
            }
        }
    }
}

fn insert_to(maybe_spill: Option<i32>, val: &Rc<Snumber>, to_left: bool) -> Rc<Snumber> {
    if let Some(spill_value) = maybe_spill {
        match &**val {
            Snumber::Literal(num) => {
                return Rc::new(Snumber::Literal(num + spill_value));
            }
            Snumber::Pair(left, right) => {
                return Rc::new(Snumber::Pair(
                    if to_left {
                        insert_to(maybe_spill, left, true)
                    } else {
                        left.clone()
                    },
                    if to_left {
                        right.clone()
                    } else {
                        insert_to(maybe_spill, right, false)
                    },
                ));
            }
        }
    } else {
        return val.clone();
    }
}

fn try_split(val: &Rc<Snumber>) -> Option<Rc<Snumber>> {
    match &**val {
        Snumber::Literal(num) => {
            let num = *num;
            if num >= 10 {
                let left = Rc::new(Snumber::Literal(num / 2));
                let right = Rc::new(Snumber::Literal(num / 2 + num % 2));
                return Some(Rc::new(Snumber::Pair(left, right)));
            } else {
                return None;
            }
        }
        Snumber::Pair(left, right) => {
            if let Some(new_val) = try_split(&left) {
                return Some(Rc::new(Snumber::Pair(new_val, right.clone())));
            } else if let Some(new_val) = try_split(&right) {
                return Some(Rc::new(Snumber::Pair(left.clone(), new_val)));
            } else {
                return None;
            }
        }
    }
}

fn magnitude(val: &Rc<Snumber>) -> i32 {
    return match &**val {
        Snumber::Literal(num) => *num,
        Snumber::Pair(left, right) => 3 * magnitude(left) + 2 * magnitude(right),
    };
}

impl BaseDay for Day18 {
    fn parse(&mut self, input: &mut InputReader) {
        self.vals = parse_lines(input, &mut parse_line);
    }

    fn pt1(&mut self) -> String {
        let total = add_all(&self.vals);
        return magnitude(&total).to_string();
    }

    fn pt2(&mut self) -> String {
        let mut best = 0;
        for i in 0..self.vals.len() {
            for j in 0..self.vals.len() {
                if i != j {
                    let sum = magnitude(&add_snumber(&self.vals[i], &self.vals[j], false));
                    if sum > best {
                        best = sum;
                    }
                }
            }
        }
        return best.to_string();
    }
}

fn main() {
    let mut day = Day18 { vals: Vec::new() };
    run_day(&mut day);
}
