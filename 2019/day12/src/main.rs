#[macro_use] extern crate failure;
#[macro_use] extern crate lazy_static;

use failure::Error;
use itertools::Itertools;
use num_integer::lcm;
use regex::Regex;
use std::collections::HashMap;
use std::io::{BufReader,BufRead};
use std::fs::File;

#[derive(Debug)]
struct Point {
    x: i64,
    y: i64,
    z: i64,
}

#[derive(Debug)]
struct Moon {
    position: Point,
    velocity: Point,
}

fn parse_int(val: &str) -> Result<i64, Error> {
    return Ok(val.parse::<i64>()?);
}

fn parse_line(line: &str) -> Result<Moon, Error> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"<x=(-?\d+),\s*y=(-?\d+),\s*z=(-?\d+)>").unwrap();
    }
    match RE.captures(line) {
        Some(caps) => {
            let pos = Point { x: parse_int(caps.get(1).unwrap().as_str())?,
                              y: parse_int(caps.get(2).unwrap().as_str())?,
                              z: parse_int(caps.get(3).unwrap().as_str())? };
            return Ok(Moon { position: pos, velocity: Point { x: 0, y: 0, z: 0 } });
        }
        None => {
            bail!("Bad line: {}", line);
        }
    }
}

fn read_input(file: &str) -> Result<Vec<Moon>, Error> {
    let f = File::open(file)?;
    let br = BufReader::new(f);
    return br.lines().map(|line| parse_line(&line?)).collect();
}

macro_rules! apply_gravity_once {
($p1: expr, $p2: expr, $v1: expr, $v2: expr) => {
    if $p1 < $p2 {
        $v1 += 1;
        $v2 -= 1;
    } else if $p2 < $p1 {
        $v2 += 1;
        $v1 -= 1;
    }
  }
}

fn apply_gravity(moons: &mut Vec<Moon>, im: usize, jm: usize) {
    apply_gravity_once!(moons[im].position.x, moons[jm].position.x, moons[im].velocity.x, moons[jm].velocity.x);
    apply_gravity_once!(moons[im].position.y, moons[jm].position.y, moons[im].velocity.y, moons[jm].velocity.y);
    apply_gravity_once!(moons[im].position.z, moons[jm].position.z, moons[im].velocity.z, moons[jm].velocity.z);
}

fn apply_velocity(m1: &mut Moon) {
    m1.position.x += m1.velocity.x;
    m1.position.y += m1.velocity.y;
    m1.position.z += m1.velocity.z;
}

fn run_simulation(moons: &mut Vec<Moon>, steps: i32) {
    for step in 0..steps {
        if step < 11 || step % 10 == 0 {
            println!("After step {}:", step);
            for moon in &*moons {
                println!("  {:?} pot {}, kin {}", moon, potential_energy(&moon), kinetic_energy(&moon));
            }
            println!();
        }

        for im in 0..moons.len() {
            for jm in im..moons.len() {
                apply_gravity(moons, im, jm);
            }
        }
        for mut moon in &mut *moons {
            apply_velocity(&mut moon);
        }
    }
    println!("Final:");
    for moon in &*moons {
        println!("  {:?}", moon);
    }
    println!();
}

fn potential_energy(moon: &Moon) -> i64 {
    return moon.position.x.abs() + moon.position.y.abs() + moon.position.z.abs();
}

fn kinetic_energy(moon: &Moon) -> i64 {
    return moon.velocity.x.abs() + moon.velocity.y.abs() + moon.velocity.z.abs();
}

fn calc_energy(moons: &Vec<Moon>) -> i64 {
    return moons.into_iter().map(|m| potential_energy(m) * kinetic_energy(m)).sum();
}

fn find_cycle(moons: &mut Vec<Moon>) -> i64 {
    struct Cycle {
        items: HashMap<String, i64>,
        start: Option<i64>,
        end: Option<i64>,
    }

    fn add_to_cycle(moons: &Vec<Moon>, getvals: impl Fn(&Moon) -> Vec<i64>, step: i64, cycle: &mut Cycle) {
        if cycle.start.is_some() {
            return;
        }
        let vs = moons.into_iter().flat_map(|m| getvals(m)).map(|v| v.to_string()).format(",").to_string();
        match cycle.items.get(&vs) {
            Some(v) => {
                cycle.start = Some(*v);
                cycle.end = Some(step);
            },
            None => {
                cycle.items.insert(vs, step);
            }
        }
    }

    let mut step = 0;
    let mut xs = Cycle { items: HashMap::new(), start: None, end: None };
    let mut ys = Cycle { items: HashMap::new(), start: None, end: None };
    let mut zs = Cycle { items: HashMap::new(), start: None, end: None };
    loop {
        add_to_cycle(&moons, |m| { vec![m.position.x, m.velocity.x] }, step, &mut xs);
        add_to_cycle(&moons, |m| { vec![m.position.y, m.velocity.y] }, step, &mut ys);
        add_to_cycle(&moons, |m| { vec![m.position.z, m.velocity.z] }, step, &mut zs);
        if xs.start.is_some() && ys.start.is_some() && zs.start.is_some() || step > 1000000 {
            break;
        }
        for im in 0..moons.len() {
            for jm in im..moons.len() {
                apply_gravity(moons, im, jm);
            }
        }
        for mut moon in &mut *moons {
            apply_velocity(&mut moon);
        }
        step += 1;
    }
    if xs.start.is_some() && ys.start.is_some() && zs.start.is_some() {
        println!("Found cycle:");
        println!(" X: {} to {}", xs.start.unwrap(), xs.end.unwrap());
        println!(" Y: {} to {}", ys.start.unwrap(), ys.end.unwrap());
        println!(" Z: {} to {}", zs.start.unwrap(), zs.end.unwrap());
        if xs.start.unwrap() != 0 || ys.start.unwrap() != 0 || zs.start.unwrap() != 0 {
            panic!("Not sure how to handle non-zero starts yet");
        }
        return lcm(lcm(xs.end.unwrap(), ys.end.unwrap()), zs.end.unwrap());
    } else {
        panic!("No cycle found after {} steps", step)
    }
    return 0;
}

fn main() {
    let args : Vec<String> = std::env::args().collect();
    if args[1] == "1" {
        println!("Doing part 1");
        let mut moons = read_input(&args[2]).unwrap();
        let turns = if args.len() > 3 { parse_int(&args[3]).unwrap() as i32 } else { 100 };
        run_simulation(&mut moons, turns);
        let energy = calc_energy(&moons);
        println!("Total energy: {}", energy);
    } else {
        println!("Doing part 2");
        let mut moons = read_input(&args[2]).unwrap();
        let cycle_at = find_cycle(&mut moons);
        println!("Found cycle at {}", cycle_at);
    }
}
