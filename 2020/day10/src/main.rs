use failure::Error;
use std::collections::HashMap;
use std::io::{BufReader,BufRead};
use std::fs::File;

fn parse_int(val: &str) -> Result<i64, Error> {
    return Ok(val.parse::<i64>()?);
}

fn read_input(file: &str) -> Result<Vec<i64>, Error> {
    let f = File::open(file)?;
    let br = BufReader::new(f);
    return br.lines().map(|line| parse_int(&line?)).collect::<Result<Vec<i64>, Error>>();
}

fn find_diffs(values: &Vec<i64>) -> (i64, i64) {
    let mut diff1s = 0;
    let mut diff3s = 0;
    let mut cpy : Vec<i64> = values.iter().cloned().collect();
    cpy.sort();
    cpy.insert(0, 0);
    cpy.insert(cpy.len(), cpy[cpy.len() - 1] + 3);
    for i in 0..cpy.len() - 1 {
        let diff = cpy[i+1] - cpy[i];
        if diff == 1 {
            diff1s += 1;
        } else if diff == 3 {
            diff3s += 1;
        }
    }
    return (diff1s, diff3s);
}

fn find_combos(values: &Vec<i64>) -> i64 {
    let mut cpy : Vec<i64> = values.iter().cloned().collect();
    cpy.sort();
    cpy.insert(0, 0);
    cpy.insert(cpy.len(), cpy[cpy.len() - 1] + 3);

    let mut muls = HashMap::new();
    muls.insert(1, 1);
    muls.insert(2, 1);
    muls.insert(3, 2);
    muls.insert(4, 4);
    muls.insert(5, 7);
    let muls = muls;

    let mut combos = 1;
    let mut idx = 0;
    while idx < cpy.len() {
        let mut j = idx + 1;
        // this probably won't give the right answer for i, i+2, i+4, ...
        while j < cpy.len() - 1 && cpy[j-1] + 3 > cpy[j] {
            j += 1;
        }
        if j - idx > 1 {
            println!("Seq from {} to {}: {}", cpy[idx], cpy[j-1], j - idx);
        }
        combos *= muls.get(&((j - idx) as usize)).unwrap();
        idx = j;
    }
    return combos;
}

fn find_combos2(values: &Vec<i64>) -> i64 {
    let mut cpy : Vec<i64> = values.iter().cloned().collect();
    cpy.sort();
    cpy.insert(0, 0);
    cpy.insert(cpy.len(), cpy[cpy.len() - 1] + 3);

    return find_combos2_recur(&cpy, 0, &mut HashMap::new());
}

fn find_combos2_recur(values: &Vec<i64>, idx: usize, cache: &mut HashMap<usize, i64>) -> i64 {
    if let Some(val) = cache.get(&idx) {
        return *val;
    }

    if idx >= values.len() - 1 {
        cache.insert(idx, 1);
        return 1;
    }

    let mut combos = 0;
    for i in 1..4 {
        if idx+i < values.len() && values[idx] + 3 >= values[idx+i] {
            let recur = find_combos2_recur(values, idx + i, cache);
            // println!("Considering {}, {} -> {}", values[idx], values[idx+i], recur);
            combos += recur;
        }
    }
    cache.insert(idx, combos);
    return combos;
}

fn main() {
    let args : Vec<String> = std::env::args().collect();
    if args[1] == "1" {
        println!("Doing part 1");
        let values = read_input(&args[2]).unwrap();
        let (diff1s, diff3s) = find_diffs(&values);
        println!("Diffs is {} * {} = {}", diff1s, diff3s, diff1s * diff3s);
    } else {
        println!("Doing part 2");
        let values = read_input(&args[2]).unwrap();
        let combos = find_combos2(&values);
        println!("Combos is {}", combos);
    }
}
