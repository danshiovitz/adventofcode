#[macro_use] extern crate failure;
#[macro_use] extern crate lazy_static;

use failure::Error;
use gcd::Gcd;
use itertools::Itertools;
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::io::{BufReader,BufRead};
use std::fs::File;

struct Data {
    asteroids: HashSet<(i32, i32)>,
    width: i32,
    height: i32,
}

fn read_input(file: &str) -> Result<Data, Error> {
    let f = File::open(file)?;
    let br = BufReader::new(f);
    let mut y = 0;
    let mut asteroids = HashSet::new();
    let mut width = 0;
    for line in br.lines() {
        let mut x = 0;
        for c in line?.chars() {
            if c == '#' {
                asteroids.insert((x, y));
            }
            x += 1;
        }
        if x > width {
            width = x;
        }
        y += 1;
    }
    return Ok(Data { asteroids: asteroids, height: y, width: width });
}

fn calc_seeable(x: i32, y: i32, data: &Data) -> i32 {
    // println!("Calculating seeables for {}, {}", x, y);
    let mut blockers = HashSet::new();
    for asteroid in &data.asteroids {
         // Never count yourself
         if *asteroid == (x, y) {
             continue;
         }
         let (ax, ay) = *asteroid;
         let xdiff = ax - x;
         let ydiff = ay - y;
         let gcd = (xdiff.abs() as u32).gcd(ydiff.abs() as u32) as i32;
         let rxy = (xdiff/gcd, ydiff/gcd);
         blockers.insert(rxy);
         // println!("- blocker {}, {}", rxy.0, rxy.1);
        }
     return blockers.len() as i32;
}

fn find_best_vantage(data: &Data) -> ((i32, i32), i32) {
    let mut best_xy = (-1, -1);
    let mut best_count = -1;
    for asteroid in &data.asteroids {
        let (x, y) = *asteroid;
        let c = calc_seeable(x, y, data);
        if c > best_count {
            best_count = c;
            best_xy = (x, y);
        }
    }
    return (best_xy, best_count);
}

fn filtered_ratios(
  ratios: &Vec<(i32, i32)>,
  filter: impl Fn((i32, i32)) -> bool) -> Vec<(i32, i32)> {
    let mut ret : Vec<(i32, i32)> = Vec::new();
    for ratio in ratios {
        if filter(*ratio) {
            ret.push(*ratio);
        }
    }
    let keyf = |(cx, cy)| {
        if cx == 0 || cy == 0 {
            return (cx + cy) as f32;
        } else {
            return cy as f32 / cx as f32;
        }
    };
    ret.sort_by(|a, b| { return *&keyf(*a).partial_cmp(&keyf(*b)).unwrap(); });
    // println!("Filter:");
    // for r in &ret {
    //     println!("- {:?}", r);
    // }
    return ret;
}

fn find_vaporization_order(x: i32, y: i32, data: &Data) -> Vec<(i32, i32)> {
    let mut blockers : HashMap<(i32, i32), Vec<(i32, i32)>> = HashMap::new();
    for asteroid in &data.asteroids {
         // Never count yourself
         if *asteroid == (x, y) {
             continue;
         }
         let (ax, ay) = *asteroid;
         let xdiff = ax - x;
         let ydiff = ay - y;
         let gcd = (xdiff.abs() as u32).gcd(ydiff.abs() as u32) as i32;
         let rxy = (xdiff/gcd, ydiff/gcd);
         // println!("{},{} to {},{} -> {},{}", x, y, ax, ay, rxy.0, rxy.1);
         match blockers.get_mut(&rxy) {
             Some(v) => { v.push((ax, ay)); },
             None => { blockers.insert(rxy, vec![(ax, ay)]); }
         }
    }
    let td = |(cx, cy): (i32, i32)| { return (x - cx).abs() + (y - cy).abs(); };
    for v in blockers.values_mut() {
        v.sort_by(|a, b| { return *&td(*a).cmp(&td(*b)); });
    }

    let bk : Vec<(i32, i32)> = blockers.keys().map(|v| *v).collect();
    let mut bks : Vec<(i32, i32)> = Vec::new();
    bks.extend(filtered_ratios(&bk, |(cx, cy)| { return cx == 0 && cy < 0; }));
    bks.extend(filtered_ratios(&bk, |(cx, cy)| { return cx > 0 && cy < 0; }));
    bks.extend(filtered_ratios(&bk, |(cx, cy)| { return cx > 0 && cy == 0; }));
    bks.extend(filtered_ratios(&bk, |(cx, cy)| { return cx > 0 && cy > 0; }));
    bks.extend(filtered_ratios(&bk, |(cx, cy)| { return cx == 0 && cy > 0; }));
    bks.extend(filtered_ratios(&bk, |(cx, cy)| { return cx < 0 && cy > 0; }));
    bks.extend(filtered_ratios(&bk, |(cx, cy)| { return cx < 0 && cy == 0; }));
    bks.extend(filtered_ratios(&bk, |(cx, cy)| { return cx < 0 && cy < 0; }));

    let mut vaporized = Vec::new();
    loop {
        let mut did_any = false;
        for k in &mut bks {
            let mut v = blockers.get_mut(&k).unwrap();
            if v.len() > 0 {
                let nx = v.remove(0);
                // println!("Vaporizing: {:?}", nx);
                vaporized.push(nx);
                did_any = true;
            }
        }
        if !did_any {
            break;
        }
    }
    return vaporized;
}

fn main() {
    let args : Vec<String> = std::env::args().collect();
    if args[1] == "1" {
        println!("Doing part 1");
        let data = read_input(&args[2]).unwrap();
        let (best_vantage, best_count) = find_best_vantage(&data);
        println!("Best vantage is {:?} which can see {}", best_vantage, best_count);
    } else {
        println!("Doing part 2");
        let data = read_input(&args[2]).unwrap();
        let (best_vantage, _best_count) = find_best_vantage(&data);
        let targets = find_vaporization_order(best_vantage.0, best_vantage.1, &data);
        if targets.len() >= 200 {
            println!("The 200th asteroid vaporized from {:?} is {},{} = {}",
                best_vantage, targets[199].0, targets[199].1, ((targets[199].0 * 100) + targets[199].1));
        }
    }
}
