use failure::Error;
use std::collections::HashSet;
use std::io::{BufReader,BufRead};
use std::fs::File;

fn parse_int(val: &str) -> Result<i64, Error> {
    return Ok(val.parse::<i64>()?);
}

fn read_input(file: &str) -> Result<Vec<i64>, Error> {
    let f = File::open(file)?;
    let br = BufReader::new(f);
    return br.lines().map(|line| parse_int(&line?)).collect::<Result<Vec<i64>, Error>>();
}

fn find_first_invalid(values: &Vec<i64>, preamble: usize) -> i64 {
    for i in preamble..values.len() {
        let mut valid = false;
        'outer: for j in 0..preamble-1 {
            for k in j..preamble {
                let ij = i - preamble + j;
                let ik = i - preamble + k;
                if values[ij] + values[ik] == values[i] {
                    valid = true;
                    break 'outer;
                }
            }
        }
        if !valid {
            return values[i];
        }
    }
    panic!("Everything valid?");
}

fn find_sums_to(values: &Vec<i64>, target: i64) -> (i64, i64) {
    for i in 0..values.len() - 1 {
        let mut tot = 0;
        for j in i..values.len() {
            tot += values[j];
            if tot == target {
                let region = &values[i..j];
                let vmin = region.iter().min().unwrap();
                let vmax = region.iter().max().unwrap();
                return (*vmin, *vmax);
            }
        }
    }
    panic!("Nothing sums");
}

fn main() {
    let args : Vec<String> = std::env::args().collect();
    if args[1] == "1" {
        println!("Doing part 1");
        let values = read_input(&args[2]).unwrap();
        let preamble = if args.len() > 2 { parse_int(&args[3]).unwrap() as usize } else { 25 };
        let invalid = find_first_invalid(&values, preamble);
        println!("Invalid is {}", invalid);
    } else {
        println!("Doing part 2");
        let values = read_input(&args[2]).unwrap();
        let preamble = if args.len() > 2 { parse_int(&args[3]).unwrap() as usize } else { 25 };
        let invalid = find_first_invalid(&values, preamble);
        let (sum_min, sum_max) = find_sums_to(&values, invalid);
        println!("Sums is {} + {} = {}", sum_min, sum_max, sum_min + sum_max);
    }
}
