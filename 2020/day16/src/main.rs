#[macro_use] extern crate failure;
#[macro_use] extern crate lazy_static;

use failure::{Error, bail};
use regex::Regex;
use std::collections::HashSet;
use std::io::{BufReader, BufRead};
use std::fs::File;

#[derive(PartialEq, Eq, Clone, Debug, Hash)]
struct Field {
    name: String,
    ranges: Vec<(i64, i64)>,
}

#[derive(PartialEq, Eq, Clone, Debug, Hash)]
struct Ticket {
    values: Vec<i64>,
}

#[derive(PartialEq, Eq, Clone, Debug, Hash)]
struct Notes {
    fields: Vec<Field>,
    pc_ticket: Ticket,
    other_tickets: Vec<Ticket>,
}

fn parse_field(line: &str) -> Result<Field, Error> {
    lazy_static! {
        static ref KV_RE: Regex = Regex::new(r"^(.*)\s*:\s*(.*)$").unwrap();
        static ref OR_RE: Regex = Regex::new(r"\s*or\s*").unwrap();
        static ref RANGE_RE: Regex = Regex::new(r"(\d+)\s*-\s*(\d+)").unwrap();
    }
    let mut ret = Field { name: "".to_string(), ranges: Vec::new() };
    if let Some(kv_caps) = KV_RE.captures(line) {
        ret.name = kv_caps.get(1).unwrap().as_str().to_string();
        let body = kv_caps.get(2).unwrap().as_str();
        for range in OR_RE.split(body) {
            if let Some(range_caps) = RANGE_RE.captures(range) {
                let min : i64 = range_caps.get(1).unwrap().as_str().parse()?;
                let max : i64 = range_caps.get(2).unwrap().as_str().parse()?;
                ret.ranges.push((min, max));
            } else {
                bail!("Bad range: {} (in {})", body, line);
            }
        }
        return Ok(ret);
    } else {
        bail!("Bad line: {}", line);
    }
}

fn parse_ticket(line: &str) -> Result<Ticket, Error> {
    lazy_static! {
        static ref COMMA_RE: Regex = Regex::new(r"\s*,\s*").unwrap();
    }
    let values = COMMA_RE.split(line).map(|s| s.parse::<i64>().unwrap()).collect();
    return Ok(Ticket { values: values });
}

fn read_input(file: &str) -> Result<Notes, Error> {
    let f = File::open(file)?;
    let br = BufReader::new(f);

    let mut fields = Vec::new();
    let mut pc_ticket : Option<Ticket> = None;
    let mut other_tickets = Vec::new();
    let mut parse_state = 0;

    for line in br.lines() {
        let line = line?;
        if parse_state == 0 {
            if line.len() == 0 {
                parse_state += 1;
            } else {
                fields.push(parse_field(&line)?);
            }
        } else if parse_state == 1 {
            if line == "your ticket:" {
                parse_state += 1;
            } else {
                bail!("Bad line at parse_state 1: {}", line);
            }
        } else if parse_state == 2 {
            if line.len() == 0 {
                parse_state += 1;
            } else if pc_ticket.is_none() {
                pc_ticket = Some(parse_ticket(&line)?);
            } else {
                bail!("Dup pc ticket at parse_state 2: {}", line);
            }
        } else if parse_state == 3 {
            if line == "nearby tickets:" {
                parse_state += 1;
            } else {
                bail!("Bad line at parse_state 3: {}", line);
            }
        } else if parse_state == 4 {
            other_tickets.push(parse_ticket(&line)?);
        }
    }
    if parse_state != 4 {
        bail!("Bad parse at end: {}", parse_state);
    }
    return Ok(Notes { fields: fields, pc_ticket: pc_ticket.unwrap(), other_tickets: other_tickets });
}

fn is_valid(value: i64, field: &Field) -> bool {
    for (min, max) in &field.ranges {
        if value >= *min && value <= *max {
            // println!("Matched {} to field {}", value, field.name);
            return true;
        }
    }
    return false;
}

fn check_nearby(notes: &Notes) -> i64 {
    return notes.other_tickets.iter().flat_map(|tik| tik.values.iter().filter(|v| !&notes.fields.iter().any(|f| is_valid(**v, f)))).sum::<i64>();
}

fn is_ticket_valid(ticket: &Ticket, fields: &Vec<Field>) -> bool {
    return ticket.values.iter().all(|v| fields.iter().any(|f| is_valid(*v, f)));
}

fn reduce_mapping(ticket: &Ticket, fields: &Vec<Field>, mapping: &mut Vec<HashSet<String>>) -> () {
    for idx in 0..ticket.values.len() {
        for field in fields {
            if !is_valid(ticket.values[idx], field) {
                let was_present = mapping[idx].remove(&field.name);
                if was_present {
                    println!("Idx {} can't be field {} due to val {}", idx, field.name, ticket.values[idx]);
                }
            }
        }
    }
}

fn deduce_mapping(notes: &Notes) -> Vec<String> {
    let mut mapping = Vec::new();
    let field_names : HashSet<String> = notes.fields.iter().map(|f| f.name.clone()).collect();
    for _ in 0..notes.fields.len() {
        mapping.push(field_names.clone());
    }

    for tik in &notes.other_tickets {
        if is_ticket_valid(&tik, &notes.fields) {
            reduce_mapping(&tik, &notes.fields, &mut mapping);
        }
    }
    reduce_mapping(&notes.pc_ticket, &notes.fields, &mut mapping);

    // Now do a final scrub to deal with {{"a", "b"}, {"a"}, {"b", "c"}}
    let mut did_any = true;
    let mut scrubbed = HashSet::new();
    while did_any {
        did_any = false;
        for idx in 0..mapping.len() {
            if mapping[idx].len() == 1 && !scrubbed.contains(&idx) {
                scrubbed.insert(idx);
                let name = mapping[idx].iter().nth(0).unwrap().to_string();
                for j in 0..mapping.len() {
                    if j != idx {
                        if mapping[j].remove(&name) {
                            did_any = true;
                        }
                    }
                }
            }
        }
    }
    return mapping.iter().map(|vs| vs.iter().nth(0).unwrap().to_string()).collect();
}

fn main() {
    let args : Vec<String> = std::env::args().collect();
    if args[1] == "1" {
        println!("Doing part 1");
        let notes = read_input(&args[2]).unwrap();
        let err_val = check_nearby(&notes);
        println!("Total error: {}", err_val);
    } else {
        println!("Doing part 2");
        let notes = read_input(&args[2]).unwrap();
        let deduced_mapping = deduce_mapping(&notes);
        println!("Deduced mapping: {:?}", deduced_mapping);
        let departures = deduced_mapping.iter().enumerate().filter(|(_idx, val)| val.starts_with("departure"));
        let departures = departures.collect::<Vec<(usize, &String)>>();
        println!("Departures: {:?}", departures);
        println!("Product: {}", departures.iter().map(|(idx, _val)| notes.pc_ticket.values[*idx]).fold(1, |acc, x| acc * x));
    }
}
