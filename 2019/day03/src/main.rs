#[macro_use] extern crate failure;
#[macro_use] extern crate lazy_static;

use failure::{Error, err_msg};
use itertools::Itertools;
use regex::Regex;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::io::{BufReader,BufRead};
use std::fs::File;

#[derive(PartialEq, Eq, Copy, Clone, Debug, Hash)]
struct Point {
    x: i32,
    y: i32
}

impl Ord for Point {
    fn cmp(&self, other: &Self) -> Ordering {
        let sd = self.x.abs() + self.y.abs();
        let od = other.x.abs() + other.y.abs();
        if sd == od {
            return self.x.cmp(&other.x);
        } else {
            return sd.cmp(&od);
        }
    }
}

impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn parse_int(val: &str) -> Result<i32, Error> {
    return Ok(val.parse::<i32>()?);
}

fn make_wire(start: Point, segment: &str) -> Result<Vec<Point>, Error> {
    let ln : i32 = parse_int(&segment[1..])?;
    let dir = segment.chars().next().ok_or(err_msg("No dir in segment"))?;
    if dir == 'U' {
        return Ok((1..ln+1).map(|i| Point{x: start.x, y: start.y + i}).collect());
    } else if dir == 'D' {
        return Ok((1..ln+1).map(|i| Point{x: start.x, y: start.y - i}).collect());
    } else if dir == 'R' {
        return Ok((1..ln+1).map(|i| Point{x: start.x + i, y: start.y}).collect());
    } else if dir == 'L' {
        return Ok((1..ln+1).map(|i| Point{x: start.x - i, y: start.y}).collect());
    } else {
        bail!("Unknown dir: {}", dir);
    }
}

fn parse_line(line: &str) -> Result<HashMap<Point, i32>, Error> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r",\s*").unwrap();
    }
    let mut start = Point { x: 0, y: 0 };
    let mut points = HashMap::new();
    let mut d = 1;
    for segment in RE.split(&line) {
        let wire = make_wire(start, &segment)?;
        for point in wire {
            points.insert(point, d);
            d = d + 1;
            start = point;
        }
    }
    return Ok(points);
}

fn read_input(file: &str) -> Result<Vec<HashMap<Point, i32>>, Error> {
    let f = File::open(file)?;
    let br = BufReader::new(f);
    return br.lines().map(|line| parse_line(&line?)).collect();
}

fn find_overlaps(wire1: &HashMap<Point, i32>, wire2: &HashMap<Point, i32>) -> HashMap<Point, i32> {
    let mut ret = HashMap::new();
    for (p1, dist1) in wire1.iter() {
         match wire2.get(&p1) {
             Some(dist2) => { ret.insert(*p1, dist1 + dist2); },
             None => {},
         }
    }
    return ret;
}

fn main() {
    let args : Vec<String> = std::env::args().collect();
    if args[1] == "1" {
        println!("Doing part 1");
        let wires = read_input(&args[2]).unwrap();
        assert!(wires.len() == 2, "Should have exactly two wires");
        let overlaps = find_overlaps(&wires[0], &wires[1]);
        for overlap in overlaps.keys() {
            println!("Found overlap: {:?}", overlap)
        }
        let nearest = overlaps.keys().filter(|&o| *o != Point{x: 0, y: 0}).sorted().next().unwrap();
        println!("Nearest: {:?} = {}", nearest, nearest.x.abs() + nearest.y.abs());
    } else {
        println!("Doing part 2");
        let wires = read_input(&args[2]).unwrap();
        assert!(wires.len() == 2, "Should have exactly two wires");
        let overlaps = find_overlaps(&wires[0], &wires[1]);
        for overlap in overlaps.keys() {
            println!("Found overlap: {:?}", overlap)
        }
        let nearest = overlaps.iter().filter(|&o| *o.0 != Point{x: 0, y: 0}).sorted_by(|a, b| a.1.cmp(b.1)).next().unwrap();
        println!("Nearest: {:?} = {}", nearest.0, nearest.1);
    }
}
