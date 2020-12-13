#[macro_use] extern crate lazy_static;
#[macro_use] extern crate failure;

use failure::{Error, bail};
use num_bigint::BigInt;
use num_bigint::ToBigInt;
use num_traits::{One,Zero};
use regex::Regex;
use std::io::{BufReader,BufRead};
use std::fs::File;

#[derive(PartialEq, Eq, Clone, Debug, Hash)]
struct Schedule {
    min_time: i64,
    buses: Vec<(i64, i64)>,
}

fn parse_int(val: &str) -> Result<i64, Error> {
    return Ok(val.parse::<i64>()?);
}

fn parse_buses(line: &str) -> Result<Vec<(i64, i64)>, Error> {
    lazy_static! {
        static ref COMMA_RE: Regex = Regex::new(r",\s*").unwrap();
    }
    let mut ret = Vec::new();
    let mut offset = 0;
    for piece in COMMA_RE.split(line) {
        if piece != "x" {
            ret.push((parse_int(piece)?, offset));
        }
        offset += 1;
    }
    return Ok(ret);
}

fn read_input(file: &str) -> Result<Schedule, Error> {
    let f = File::open(file)?;
    let br = BufReader::new(f);
    let vals : Vec<String> = br.lines().map(|line| line.unwrap()).collect();
    if vals.len() != 2 {
        bail!("Expected 2 lines in input, found {}", vals.len());
    }
    let min_time = parse_int(vals[0].as_str())?;
    let buses = parse_buses(vals[1].as_str())?;
    return Ok(Schedule { min_time: min_time, buses: buses });
}

fn next_departing(schedule: &Schedule) -> (i64, i64) {
    let calc_delay = |bus_num| {
        return (bus_num - (schedule.min_time % bus_num)) % bus_num;
    };

    let bus_delays : Vec<(i64, i64)> = schedule.buses.iter().map(|(b, _)| (*b, calc_delay(*b))).collect();
    return *bus_delays.iter().min_by(|a, b| a.1.cmp(&b.1)).unwrap();
}

// gcd stuff per https://math.stackexchange.com/a/3864593
fn combine_phased_rotations(a_period: &BigInt, a_phase: &BigInt, b_period: &BigInt, b_phase: &BigInt) -> (BigInt, BigInt) {
    let (gcd, s, _t) = extended_gcd(a_period, b_period);
    let phase_difference = a_phase - b_phase;
    let pd_mult = &phase_difference / &gcd;
    let pd_remainder = &phase_difference % &gcd;
    if pd_remainder != Zero::zero() {
        panic!("Rotation reference points never synchronize.")
    }

    let combined_period = a_period / gcd * b_period;
    let combined_phase = (a_phase - &s * &pd_mult * a_period) % &combined_period;
    return (combined_period, combined_phase);
}

// Expected:
//   print(arrow_alignment(red_len=9, green_len=15, advantage=3))  # 18
//   print(arrow_alignment(red_len=30, green_len=38, advantage=6))  # 120
//   print(arrow_alignment(red_len=9, green_len=12, advantage=5))  # ValueError
fn _arrow_alignment(red_len: &BigInt, green_len: &BigInt, advantage: &BigInt) -> BigInt {
    let (period, phase) = combine_phased_rotations(
        &red_len, &Zero::zero(), &green_len, &(-advantage % green_len)
    );
    return -phase % period;
}

fn extended_gcd(a: &BigInt, b: &BigInt) -> (BigInt, BigInt, BigInt) {
    let (mut old_r, mut r) : (BigInt, BigInt) = (a.clone(), b.clone());
    let (mut old_s, mut s) : (BigInt, BigInt) = (One::one(), Zero::zero());
    let (mut old_t, mut t) : (BigInt, BigInt) = (Zero::zero(), One::one());
    while r != Zero::zero() {
        let quotient = &old_r / &r;
        let remainder = &old_r % &r;
        old_r = r.clone();
        r = remainder;
        let tmp_s = &old_s - &quotient * &s;
        old_s = s.clone();
        s = tmp_s;
        let tmp_t = &old_t - &quotient * &t;
        old_t = t.clone();
        t = tmp_t;
    }

    return (old_r, old_s, old_t);
}

fn calc_prize_time(schedule: &Schedule) -> BigInt {
    if schedule.buses.len() == 0 {
        return 0.to_bigint().unwrap();
    }
    let (mut r_period, mut r_phase) = (schedule.buses[0].0.to_bigint().unwrap(), schedule.buses[0].1.to_bigint().unwrap());
    // phase is supposed to be negative for whatever reason
    r_phase *= -1;
    for (bus_num, delay) in &schedule.buses[1..] {
        let c_period = bus_num.to_bigint().unwrap();
        let c_phase = (-*delay % *bus_num).to_bigint().unwrap();
        let (j_period, j_phase) = combine_phased_rotations(
            &r_period, &r_phase, &c_period, &c_phase
        );
        r_period = j_period.clone();
        r_phase = &j_phase % &j_period;
    }
    println!("i see {} {}", &r_phase, &r_period);
    if &r_phase < &Zero::zero() {
        r_phase += &r_period;
    }
    return &r_phase % &r_period;
}

fn main() {
    let args : Vec<String> = std::env::args().collect();
    if args[1] == "1" {
        println!("Doing part 1");
        let schedule = read_input(&args[2]).unwrap();
        let (bus_num, delay) = next_departing(&schedule);
        println!("Next departing is {} after {}: {}", bus_num, delay, bus_num * delay);
    } else {
        println!("Doing part 2");
        let schedule = read_input(&args[2]).unwrap();
        let prize_time = calc_prize_time(&schedule);
        println!("Prize time is {}", prize_time);
    }
}
