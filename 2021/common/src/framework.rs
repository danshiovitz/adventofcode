use std::boxed::Box;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Read};

use lazy_regex::regex;

use crate::grid::{Coord, Grid};

pub type InputReader = BufReader<Box<dyn Read>>;

pub trait BaseDay {
    fn parse(&mut self, input: &mut InputReader);
    fn pt1(&mut self) -> String;
    fn pt2(&mut self) -> String;
}

pub fn parse_lines<F, T>(input: &mut InputReader, parse_line: &mut F) -> Vec<T>
where
    F: FnMut(String) -> T,
{
    let mut vals = Vec::new();
    for line in input.lines() {
        let line = line.unwrap();
        if line.len() == 0 {
            break;
        }
        vals.push(parse_line(line));
    }
    return vals;
}

// Potentially reads multiple lines, but accumulates into a single
// vec of numbers. Stops if it sees a blank line (which it consumes).
pub fn parse_numbers(input: &mut InputReader) -> Vec<i32> {
    let sep = regex!(r#"\s*,\s*"#);

    let mut vals = Vec::new();
    for line in input.lines() {
        let line = line.unwrap();
        if line.is_empty() {
            return vals;
        }
        let ws = sep.split(line.trim());
        vals.extend(ws.map(|n| n.parse::<i32>().unwrap()));
    }
    return vals;
}

pub fn parse_records<F, T>(input: &mut InputReader, parse_record: &mut F) -> Vec<T>
where
    F: FnMut(&Vec<String>) -> T,
{
    let mut vals = Vec::new();
    let mut cur = Vec::new();
    for line in input.lines() {
        let line = line.unwrap();
        if line.is_empty() {
            // TODO: alternate record separators?
            vals.push(parse_record(&cur));
            cur.clear();
        } else {
            cur.push(line);
        }
    }
    if cur.len() > 0 {
        vals.push(parse_record(&cur));
    }
    return vals;
}

pub fn parse_grid<F, T>(input: &mut InputReader, parse_coord: &mut F) -> Grid<T>
where
    F: FnMut(char, &Coord) -> Option<T>,
{
    let mut coords: HashMap<Coord, T> = HashMap::new();
    let mut max_x = 0;
    let mut max_y = 0;
    for (y, line) in input.lines().enumerate() {
        for (x, ch) in line.unwrap().chars().enumerate() {
            let coord = Coord {x: x as i32, y: y as i32};
            match parse_coord(ch, &coord) {
                Some(val) => { coords.insert(coord, val); },
                None => {},
            }
            max_x = x as i32;
        }
        max_y = y as i32;
    }
    return Grid::<T> { coords: coords, min: Coord {x: 0, y: 0}, max: Coord {x: max_x, y: max_y}};
}


fn load_expected(fname: &str) -> Vec<String> {
    let mut expected = Vec::new();
    match File::open(fname.to_owned() + ".expected") {
        Ok(f) => expected.extend(BufReader::new(f).lines().map(|line| line.unwrap())),
        Err(_) => {}
    };
    while expected.len() < 2 {
        expected.push("".to_string());
    }
    return expected;
}

pub fn run_day(day: &mut dyn BaseDay) {
    let args: Vec<String> = std::env::args().collect();

    let mut input: InputReader = match File::open(&args[1]) {
        Ok(f) => BufReader::new(Box::new(f)),
        Err(_) => panic!("Bad file: {}", &args[1]),
    };
    day.parse(&mut input);

    let expected = load_expected(&args[1]);

    let res1 = day.pt1();
    println!("Result 1: {}", res1);
    assert_eq!(res1, expected[0]);
    let res2 = day.pt2();
    println!("Result 2: {}", res2);
    assert_eq!(res2, expected[1]);
}
