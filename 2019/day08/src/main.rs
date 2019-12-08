#[macro_use] extern crate failure;
#[macro_use] extern crate lazy_static;

use failure::Error;
use itertools::Itertools;
use regex::Regex;
use std::collections::HashMap;
use std::io::{BufReader,BufRead};
use std::fs::File;

struct Data {
    raw : Vec<char>,
    width: i32,
    height: i32
}

fn num_layers(data: &Data) -> i32 {
    return data.raw.len() as i32 / (data.width * data.height);
}

fn get_layer(data: &Data, num: i32) -> &[char] {
    let amt = (data.width * data.height) as usize;
    let from = num as usize * amt;
    let to = (num + 1) as usize * amt;
    return &data.raw[from .. to];
}

enum Color {
    BLACK,
    WHITE,
}

fn get_pixel(data: &Data, x: i32, y: i32) -> Color {
    let offset = ((data.width * y) + x) as usize;
    for layer in 0..num_layers(&data) {
        match get_layer(&data, layer)[offset] {
            '0' => { return Color::BLACK; },
            '1' => { return Color::WHITE; },
            '2' => { continue; },
            x => { panic!("Unexpected value: {}", x); },
        }
    }
    return Color::WHITE;
}

fn read_input(file: &str, width: i32, height: i32) -> Result<Data, Error> {
    let f = File::open(file)?;
    let br = BufReader::new(f);
    let mut single = Vec::new();
    for line in br.lines() {
        single.extend(line?.chars());
    }
    return Ok(Data { raw: single, width: width, height: height });
}

fn verify_image(data: &Data) -> i32 {
    let mut best_num_zeros = 25 * 6 + 1;
    let mut best_digit_prod = 0;
    for layer in 0..num_layers(&data) {
        let mut counts : HashMap<char, i32> = HashMap::new();
        for ch in &['0', '1', '2'] {
            counts.insert(*ch, 0);
        }
        for ch in get_layer(&data, layer) {
            *counts.get_mut(ch).unwrap() += 1;
        }
        let digit_prod = *counts.get(&'1').unwrap() * *counts.get(&'2').unwrap();
        let zeros = *counts.get(&'0').unwrap();
        if zeros < best_num_zeros {
            best_num_zeros = zeros;
            best_digit_prod = digit_prod;
        }
    }
    return best_digit_prod;
}

fn display_image(data: &Data) {
    for y in 0..data.height {
        let mut line = String::new();
        line.reserve(data.width as usize);
        for x in 0..data.width {
            match get_pixel(&data, x, y) {
                Color::BLACK => line.push(' '),
                Color::WHITE => line.push('#'),
            }
        }
        println!("{}", line);
    }
}

fn main() {
    let args : Vec<String> = std::env::args().collect();
    if args[1] == "1" {
        println!("Doing part 1");
        let data = read_input(&args[2], 25, 6).unwrap();
        let best_digit_prod = verify_image(&data);
        println!("Digit product: {}", best_digit_prod);
    } else {
        println!("Doing part 2");
        let data = read_input(&args[2], 25, 6).unwrap();
        display_image(&data);
    }
}
