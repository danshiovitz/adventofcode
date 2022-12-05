use lazy_regex::regex;

extern crate common;

use common::framework::{parse_lines, run_day, BaseDay, InputReader};

struct Move {
    from: i32,
    to: i32,
    amount: i32,
}

struct Day05 {
    stacks: Vec<Vec<char>>,
    moves: Vec<Move>,
}

fn run_moves(stacks_orig: &Vec<Vec<char>>, moves: &Vec<Move>, single: bool) -> Vec<Vec<char>> {
    let mut stacks = stacks_orig
        .iter()
        .map(|s| s.clone())
        .collect::<Vec<Vec<char>>>();

    for mv in moves {
        if single {
            for _ in 0..mv.amount {
                let c = stacks[mv.from as usize].pop().unwrap();
                stacks[mv.to as usize].push(c);
            }
        } else {
            let mut temp = Vec::new();
            for _ in 0..mv.amount {
                let c = stacks[mv.from as usize].pop().unwrap();
                temp.push(c);
            }
            temp.reverse();
            stacks[mv.to as usize].extend(temp);
        }
    }

    return stacks;
}

impl BaseDay for Day05 {
    fn parse(&mut self, input: &mut InputReader) {
        fn parse_stackline(line: String, stacks: &mut Vec<Vec<char>>) -> bool {
            for (idx, ch) in line.chars().skip(1).step_by(4).enumerate() {
                if stacks.len() <= idx {
                    stacks.push(Vec::new());
                }
                if ch != ' ' {
                    stacks[idx].insert(0, ch);
                }
            }
            return true;
        }

        fn parse_move(line: String) -> Move {
            let rex = regex!(r#"move\s+(\d+)\s+from\s+(\d+)\s+to\s+(\d+)\s*"#);
            match rex.captures(&line) {
                Some(c) => {
                    return Move {
                        from: c[2].parse::<i32>().unwrap() - 1,
                        to: c[3].parse::<i32>().unwrap() - 1,
                        amount: c[1].parse::<i32>().unwrap(),
                    }
                }
                None => panic!("Bad line: {}", &line),
            };
        }

        parse_lines(input, &mut |line: String| {
            parse_stackline(line, &mut self.stacks)
        });
        self.moves = parse_lines(input, &mut parse_move);
    }

    fn pt1(&mut self) -> String {
        let new_stacks = run_moves(&self.stacks, &self.moves, true);
        return new_stacks
            .into_iter()
            .map(|s| s[s.len() - 1])
            .collect::<String>();
    }

    fn pt2(&mut self) -> String {
        let new_stacks = run_moves(&self.stacks, &self.moves, false);
        return new_stacks
            .into_iter()
            .map(|s| s[s.len() - 1])
            .collect::<String>();
    }
}

fn main() {
    let mut day = Day05 { stacks: Vec::new(), moves: Vec::new() };
    run_day(&mut day);
}
