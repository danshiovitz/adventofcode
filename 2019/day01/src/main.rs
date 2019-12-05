use failure::Error;
use std::io::{BufReader,BufRead};
use std::fs::File;

fn parse_line(line: &str) -> Result<i32, Error> {
    return Ok(line.parse::<i32>()?);
}

fn read_input(file: &str) -> Result<Vec<i32>, Error> {
    let f = File::open(file)?;
    let br = BufReader::new(f);
    return br.lines().map(|line| parse_line(&line?)).collect();
}

fn fuel_required(mass: i32) -> i32 {
    let fuel : i32 = (mass / 3) - 2;
    if fuel < 0 {
        return 0;
    } else {
        return fuel;
    }
}

fn recursive_fuel_required(mass: i32) -> i32 {
    let mut last : i32 = fuel_required(mass);
    let mut total : i32 = last;
    while last > 0 {
        last = fuel_required(last);
        total += last;
    }
    return total;
}

fn main() {
    let args : Vec<String> = std::env::args().collect();
    if args[1] == "1" {
        println!("Doing part 1");
        let masses = read_input(&args[2]).unwrap();
        let total_fuel : i32 = masses.into_iter().map(|mass| fuel_required(mass)).sum();
        println!("Total fuel: {}", total_fuel);
    } else {
        println!("Doing part 2");
        let masses = read_input(&args[2]).unwrap();
        let total_fuel : i32 = masses.into_iter().map(|mass| recursive_fuel_required(mass)).sum();
        println!("Total fuel: {}", total_fuel);
    }
}
