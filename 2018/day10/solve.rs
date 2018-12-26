//#[macro_use] extern crate lazy_static;

use itertools::join;
use lazy_static::lazy_static;
use regex::Regex;
use failure::{Error,err_msg};
use std::collections::HashSet;
use std::io::{BufReader,BufRead};
use std::fs::File;

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
    v_x: i32,
    v_y: i32,
}

fn parse_point(line: &str) -> Result<Point, Error> {
    lazy_static! {
        static ref RE: Regex = Regex::new("position=<\\s*(-?[0-9]+),\\s*(-?[0-9]+)> velocity=<\\s*(-?[0-9]+),\\s*(-?[0-9]+)>").unwrap();
    }
    let cap = RE.captures(line).ok_or(err_msg("Bad line"))?;
    let xs = cap.get(1).map_or("", |xm| xm.as_str());
    let ys = cap.get(2).map_or("", |ym| ym.as_str());
    let xvs = cap.get(3).map_or("", |xm| xm.as_str());
    let yvs = cap.get(4).map_or("", |ym| ym.as_str());
    return Ok(Point { x: xs.parse()?, y: ys.parse()?, v_x: xvs.parse()?, v_y: yvs.parse()? });
}

fn read_input(file: &str) -> Result<Vec<Point>, Error> {
    let f = File::open(file)?;
    let br = BufReader::new(f);
    return br.lines().map(|line| parse_point(&line?)).collect();
}

fn bounds(points: &[Point]) -> (i32, i32, i32, i32) {
    let min_x : i32 = points.iter().map(|p| p.x).min().unwrap();
    let max_x : i32 = points.iter().map(|p| p.x).max().unwrap();
    let min_y : i32 = points.iter().map(|p| p.y).min().unwrap();
    let max_y : i32 = points.iter().map(|p| p.y).max().unwrap();
    return (min_x, max_x, min_y, max_y);
}

fn map_points(min_x: i32, max_x: i32, min_y: i32, max_y: i32, points: &[Point]) -> Vec<String> {
    let mut pset = HashSet::<(i32, i32)>::new();
    for point in points {
        pset.insert((point.x, point.y));
    }
    return (min_y .. max_y + 1).map(|y|
        join((min_x .. max_x + 1).map(|x|
            if pset.contains(&(x, y)) { "#" } else { "."}), "")).
        collect();
}

fn advance_time(points: &mut Vec<Point>) -> () {
    for mut point in points {
        point.x += point.v_x;
        point.y += point.v_y;
    }
}

fn will_expand(min_x: i32, max_x: i32, min_y: i32, max_y: i32, points: &[Point]) -> bool {
    let new_min_x : i32 = points.iter().map(|p| p.x + p.v_x).min().unwrap();
    let new_max_x : i32 = points.iter().map(|p| p.x + p.v_x).max().unwrap();
    let new_min_y : i32 = points.iter().map(|p| p.y + p.v_y).min().unwrap();
    let new_max_y : i32 = points.iter().map(|p| p.y + p.v_y).max().unwrap();
    let x_diff = max_x - min_x;
    let y_diff = max_y - min_y;
    let new_x_diff = new_max_x - new_min_x;
    let new_y_diff = new_max_y - new_min_y;

    if new_x_diff > x_diff && new_y_diff > y_diff {
        return true;
    } else if new_x_diff < x_diff && new_y_diff < y_diff {
        return false;
    } else {
        panic!("Doesn't appear to be converging: x {} vs {}, y {} vs {}",
            x_diff, new_x_diff, y_diff, new_y_diff);
    }
}

fn show_possible_messages(points: &mut Vec<Point>) -> (Vec<String>, i32) {
    let mut sec = 0;
    loop {
        let (min_x, max_x, min_y, max_y) = bounds(points);
        if will_expand(min_x, max_x, min_y, max_y, points) {
            return (map_points(min_x, max_x, min_y, max_y, points), sec);
        }
        advance_time(points);
        sec += 1;
    }
}

fn main() {
    let args : Vec<String> = std::env::args().collect();
    let mut points = read_input(&args[1]).unwrap();
    let (pmap, sec) = show_possible_messages(&mut points);
    println!("Computed at {} sec", sec);
    for line in pmap {
        println!("{}", line);
    }
}
