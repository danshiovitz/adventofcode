use failure::Error;
use std::io::{BufReader,BufRead};
use std::fs::File;

fn read_input(file: &str) -> Result<Vec<String>, Error> {
    let f = File::open(file)?;
    let br = BufReader::new(f);
    let codes : Vec<String> = br.lines().map(|line| line.unwrap()).collect();
    return Ok(codes);
}

fn binsearch(code: &str, vals: (char, char), range: (i32, i32)) -> i32 {
    let mut cur = range;
    for ch in code.chars() {
        let nr = (cur.0 + cur.1) / 2;
        if ch == vals.0 {
            cur = (cur.0, nr);
        } else if ch == vals.1 {
            cur = (nr + 1, cur.1);
        } else {
            panic!("Unknown char in code: {}", ch);
        }
    }
    if cur.0 != cur.1 {
        panic!("Bad finish: {:?}", cur);
    }
    return cur.0;
}

fn to_position(code: &str) -> (i32, i32) {
    return (binsearch(&code[0..7], ('F', 'B'), (0, 127)),
            binsearch(&code[7..10], ('L', 'R'), (0, 7)));
}

fn to_id(pos: (i32, i32)) -> i32 {
    return pos.0 * 8 + pos.1;
}

fn main() {
    let args : Vec<String> = std::env::args().collect();
    if args[1] == "1" {
        println!("Doing part 1");
        let codes = read_input(&args[2]).unwrap();
        let max_sid = codes.iter().map(|c| to_id(to_position(c))).max().unwrap();
        println!("Max seat id: {}", max_sid);
    } else {
        println!("Doing part 2");
        let codes = read_input(&args[2]).unwrap();
        let mut sids = codes.iter().map(|c| to_id(to_position(c))).collect::<Vec<i32>>();
        sids.sort();
        let sids = sids;
        for i in 0..sids.len() - 1 {
            if sids[i+1] - sids[i] != 1 {
                println!("Prospect: {}, {}", sids[i], sids[i+1]);
            }
        }
    }
}
