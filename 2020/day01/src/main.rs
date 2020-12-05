use failure::Error;
use std::io::{BufReader,BufRead};
use std::fs::File;

fn parse_int(val: &str) -> Result<i64, Error> {
    return Ok(val.parse::<i64>()?);
}

fn read_input(file: &str) -> Result<Vec<i64>, Error> {
    let f = File::open(file)?;
    let br = BufReader::new(f);
    let parsed : Result<Vec<i64>, Error> = br.lines().map(|line| parse_int(&line?)).collect();
    return parsed;
}

fn find_summing(vals: &Vec<i64>) -> (i64, i64) {
    for i in 0..vals.len() {
        for j in i..vals.len() {
            if vals[i] + vals[j] == 2020 {
                return (vals[i], vals[j]);
            }
        }
    }
    panic!("No vals with correct sum found");
}

fn find_summing2(vals: &Vec<i64>) -> (i64, i64, i64) {
    for i in 0..vals.len() {
        for j in i..vals.len() {
            for k in j..vals.len() {
                if vals[i] + vals[j] + vals[k] == 2020 {
                    return (vals[i], vals[j], vals[k]);
                }
            }
        }
    }
    panic!("No vals with correct sum found");
}

fn main() {
    let args : Vec<String> = std::env::args().collect();
    if args[1] == "1" {
        println!("Doing part 1");
        let vals = read_input(&args[2]).unwrap();
        let (v1, v2) = find_summing(&vals);
        println!("Found {} and {}: {}", v1, v2, v1*v2);
    } else {
        println!("Doing part 2");
        let vals = read_input(&args[2]).unwrap();
        let (v1, v2, v3) = find_summing2(&vals);
        println!("Found {} and {} and {}: {}", v1, v2, v3, v1*v2*v3);
    }
}
