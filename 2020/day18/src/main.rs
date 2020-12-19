#[macro_use] extern crate failure;

use failure::{Error, bail};
use std::io::{BufReader,BufRead};
use std::fs::File;

#[derive(Debug)]
enum Atom {
    Value(Option<i64>),
    Op(char),
}

#[derive(Debug)]
struct Expression {
    atoms: Vec<Atom>,
}

fn parse_line(line: &str) -> Result<Expression, Error> {
    let mut chs : Vec<char> = line.chars().collect();
    let mut atoms = Vec::new();
    while chs.len() > 0 {
        let ch = chs.remove(0);
        if ch.is_digit(10) {
            atoms.push(Atom::Value(Some(ch.to_digit(10).unwrap() as i64)));
        } else if ch == '(' {
            atoms.push(Atom::Value(None));
        } else if ch == '+' || ch == '*' || ch == ')' {
            atoms.push(Atom::Op(ch));
        } else if ch == ' ' {
            continue;
        } else {
            bail!("Unknown char: {}", ch);
        }
    }
    return Ok(Expression { atoms: atoms });
}

fn read_input(file: &str) -> Result<Vec<Expression>, Error> {
    let f = File::open(file)?;
    let br = BufReader::new(f);
    let parsed : Result<Vec<Expression>, Error> = br.lines().map(|line| parse_line(&line?)).collect();
    return parsed;
}

fn eval_expr(expr: &Expression, use_prec: bool, verbose: bool) -> i64 {
    let mut idx = 0;
    return eval_atoms(&expr.atoms, use_prec, verbose, &mut idx);
}

fn eval_atoms(atoms: &Vec<Atom>, use_prec: bool, verbose: bool, idx: &mut usize) -> i64 {
    let mut tot = 0;
    let mut last_op = '+';
    loop {
        match atoms[*idx] {
            Atom::Value(_) => {
                let val = eval_value(atoms, use_prec, verbose, idx);
                if last_op == '+' {
                    tot += val;
                } else if last_op == '*' {
                    tot *= val;
                } else {
                    panic!("Bad op: {}", last_op);
                }
                last_op = ' ';
                if *idx >= atoms.len() {
                    return tot;
                }
                match atoms[*idx] {
                    Atom::Op('+') => {
                        *idx += 1; // consume add
                        last_op = '+';
                        continue;
                    },
                    Atom::Op('*') => {
                        *idx += 1; // consume multiply
                        if use_prec {
                            let rest = eval_atoms(atoms, use_prec, verbose, idx);
                            return tot * rest;
                        } else {
                            last_op = '*';
                            continue;
                        }
                    },
                    Atom::Op(')') => {
                        // don't consume it, just return
                        return tot;
                    },
                    _ => {
                        panic!("Expected op at {}, found {:?}", *idx, atoms[*idx]);
                    }
                }
            },
            _ => {
                panic!("Found unexpected {:?} at {}", atoms[*idx], *idx);
            }
        }
    }
}

fn eval_value(atoms: &Vec<Atom>, use_prec: bool, verbose: bool, idx: &mut usize) -> i64 {
    match atoms[*idx] {
        Atom::Value(None) => {
            *idx += 1; // consume open paren
            let tot = eval_atoms(atoms, use_prec, verbose, idx);
            if *idx >= atoms.len() {
                panic!("Expected ) at end of input");
            }
            match atoms[*idx] {
                Atom::Op(')') => {
                    *idx += 1; // consume close paren
                    return tot;
                },
                _ => {
                    panic!("Expected ) at {}, found {:?}", *idx, atoms[*idx]);
                }
            }
        },
        Atom::Value(Some(num)) => {
            *idx += 1; // consume number
            return num;
        },
        _ => {
            panic!("Expected value at {}, found {:?}", *idx, atoms[*idx]);
        }
    }
}

fn main() {
    let args : Vec<String> = std::env::args().collect();
    if args[1] == "1" {
        println!("Doing part 1");
        let expressions = read_input(&args[2]).unwrap();
        let verbose = args.len() > 3 && args[3].chars().nth(0).unwrap() == 't';
        if verbose {
            for expr in &expressions {
                println!("{:?}", expr);
                println!("Total is {}", eval_expr(expr, false, verbose));
            }
        }
        println!("Sum total is {}", expressions.iter().map(|expr| eval_expr(&expr, false, false)).sum::<i64>());
    } else {
        println!("Doing part 2");
        let expressions = read_input(&args[2]).unwrap();
        let verbose = args.len() > 3 && args[3].chars().nth(0).unwrap() == 't';
        if verbose {
            for expr in &expressions {
                println!("{:?}", expr);
                println!("Total is {}", eval_expr(expr, true, verbose));
            }
        }
        println!("Sum total is {}", expressions.iter().map(|expr| eval_expr(&expr, true, false)).sum::<i64>());
    }
}
