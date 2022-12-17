use lazy_regex::regex;
use std::cmp::{max, min};

extern crate common;

use common::framework::{parse_lines, run_day, BaseDay, InputReader};
use common::grid::{manhattan, Coord};

#[derive(Debug)]
struct Sensor {
    coord: Coord,
    beacon: Coord,
}

struct Day15 {
    vals: Vec<Sensor>,
}

fn add_interval(iv: (i32, i32), intervals: &mut Vec<(i32, i32)>) {
    let mut i = 0;
    while i < intervals.len() {
        if (iv.0 >= intervals[i].0 && iv.0 <= intervals[i].1)
            || (iv.1 >= intervals[i].0 && iv.0 <= intervals[i].1)
            || (iv.0 == intervals[i].1 + 1)
            || (intervals[i].0 == iv.1 + 1)
        {
            let cur = intervals.remove(i);
            let merged = (min(iv.0, cur.0), max(iv.1, cur.1));
            add_interval(merged, intervals);
            return;
        }
        i += 1;
    }
    intervals.push(iv);
}

fn sweep_line(sensors: &Vec<Sensor>, yline: i32) -> Vec<(i32, i32)> {
    let mut intervals: Vec<(i32, i32)> = vec![];
    for sensor in sensors {
        let swept = manhattan(&sensor.coord, &sensor.beacon);
        let remaining = swept - (sensor.coord.y - yline).abs();
        if remaining <= 0 {
            continue;
        }
        let iv = (sensor.coord.x - remaining, sensor.coord.x + remaining);
        add_interval(iv, &mut intervals);
        // println!("After {:?}, {:?}", sensor, intervals);
    }
    return intervals;
}

fn sweep_grid(sensors: &Vec<Sensor>, min: Coord, max: Coord) -> Coord {
    for y in min.y..=max.y {
        let intervals = sweep_line(sensors, y);
        if !intervals.iter().any(|iv| iv.0 <= min.x && iv.1 >= max.x) {
            // not really right, but oh well:
            let x = intervals[0].1 + 1;
            return Coord { x: x, y: y };
        }
    }
    panic!("Couldn't find gap!");
}

impl BaseDay for Day15 {
    fn parse(&mut self, input: &mut InputReader) {
        fn parse_line(line: String) -> Sensor {
            match regex!(
                r#"Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)"#
            )
            .captures(&line)
            {
                Some(c) => Sensor {
                    coord: Coord {
                        x: c[1].parse::<i32>().unwrap(),
                        y: c[2].parse::<i32>().unwrap(),
                    },
                    beacon: Coord {
                        x: c[3].parse::<i32>().unwrap(),
                        y: c[4].parse::<i32>().unwrap(),
                    },
                },
                None => {
                    panic!("Bad line: {}", line);
                }
            }
        }

        self.vals = parse_lines(input, &mut parse_line);
    }

    fn pt1(&mut self) -> String {
        let swept = sweep_line(&self.vals, if self.vals.len() < 20 { 10 } else { 2000000 });
        let swept_size: i32 = swept.into_iter().map(|iv| iv.1 - iv.0).sum();
        return swept_size.to_string();
    }

    fn pt2(&mut self) -> String {
        let mv = if self.vals.len() < 20 { 20 } else { 4000000 };
        let gap = sweep_grid(&self.vals, Coord { x: 0, y: 0 }, Coord { x: mv, y: mv });
        let freq = gap.x as i64 * 4000000 as i64 + gap.y as i64;
        return freq.to_string();
    }
}

fn main() {
    let mut day = Day15 { vals: Vec::new() };
    run_day(&mut day);
}
