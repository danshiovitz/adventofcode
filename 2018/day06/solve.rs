//#[macro_use] extern crate lazy_static;

use lazy_static::lazy_static;
use regex::Regex;
use failure::Error;
use std::io::{BufReader,BufRead};
use std::fs::File;

struct Point {
    x: i32,
    y: i32,
}

fn parse_point(line: &str) -> Result<Point, Error> {
    lazy_static! {
        static ref RE: Regex = Regex::new("([0-9]+)[ ,\t]+([0-9]+)").unwrap();
    }
    let cap = RE.captures(line).ok_or(std::io::Error::new(std::io::ErrorKind::Other, "Mismatch error"))?;
    let xs = cap.get(1).map_or("", |xm| xm.as_str());
    let ys = cap.get(2).map_or("", |ym| ym.as_str());
    return Ok(Point { x: xs.parse()?, y: ys.parse()? });
}

fn read_input(file: &str) -> Result<Vec<Point>, Error> {
    let f = File::open(file)?;
    let br = BufReader::new(f);
    return br.lines().map(|line| parse_point(&line?)).collect();
}

fn dist_to(point: &Point, x: i32, y: i32) -> i32 {
    return (point.x - x).abs() + (point.y - y).abs();
}

fn calc_exclusive_area(points: &[Point]) -> (usize, i32) {
    let mut areas = Vec::new();
    areas.resize(points.len(), 0);
    let mut is_inf = Vec::new();
    is_inf.resize(points.len(), false);

    let min_x : i32 = points.iter().map(|p| p.x).min().unwrap();
    let max_x : i32 = points.iter().map(|p| p.x).max().unwrap();
    let min_y : i32 = points.iter().map(|p| p.y).min().unwrap();
    let max_y : i32 = points.iter().map(|p| p.y).max().unwrap();

    let unowned : usize = points.len() + 10;
    let multi_owned : usize = points.len() + 11;

    for x in min_x .. max_x+1 {
        for y in min_y .. max_y+1 {
            let mut best_dist = max_x * max_y + 1;
            let mut best_owner = unowned;
            for (i, p) in points.iter().enumerate() {
                let d = dist_to(&p, x, y);
                if d == best_dist {
                    if best_owner == unowned {
                        best_owner = i;
                    } else {
                        best_owner = multi_owned;
                    }
                } else if d < best_dist {
                    best_dist = d;
                    best_owner = i;
                }
            }
            if best_owner != unowned && best_owner != multi_owned {
                areas[best_owner] += 1;
                if x == min_x || x == max_x || y == min_y || y == max_y {
                    is_inf[best_owner] = true;
                }
            }
        }
    }

    let best_idx = (0..points.len()).max_by_key(|i: &usize| if is_inf[*i] { 0 } else { areas[*i] }).unwrap();
    return (best_idx, areas[best_idx]);
}

fn calc_near_area(points: &[Point], max_total: i32) -> i32 {
    let min_x : i32 = points.iter().map(|p| p.x).min().unwrap();
    let max_x : i32 = points.iter().map(|p| p.x).max().unwrap();
    let min_y : i32 = points.iter().map(|p| p.y).min().unwrap();
    let max_y : i32 = points.iter().map(|p| p.y).max().unwrap();

    let slop = (max_total as f32 / points.len() as f32).ceil() as i32;
    let mut within_count = 0;

    for x in (min_x - slop) .. (max_x+slop+1) {
        for y in (min_y - slop) .. (max_y+slop+1) {
            let mut tot = 0;
            for (i, p) in points.iter().enumerate() {
                tot += dist_to(&p, x, y);
                if tot >= max_total {
                    break;
                }
            }
            if tot < max_total {
                within_count += 1;
            }
        }
    }

    return within_count;
}

fn main() {
    let args : Vec<String> = std::env::args().collect();
    let points = read_input(&args[1]).unwrap();
    let max_total = if args[1] == "input.txt" { 10000 } else { 32 };
    let (e_pid, e_max_area) = calc_exclusive_area(&points);
    println!("Max area exclusively owned by {} = {}", e_pid, e_max_area);
    let n_max_area = calc_near_area(&points, max_total);
    println!("Max area within {} of everything = {}", max_total, n_max_area);
}
