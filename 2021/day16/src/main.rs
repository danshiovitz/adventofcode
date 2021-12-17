use std::boxed::Box;
use std::collections::HashMap;

extern crate common;

use common::framework::{parse_lines, run_day, BaseDay, InputReader};

#[derive(Debug, Copy, Clone)]
enum OperatorType {
    Sum,
    Product,
    Minimum,
    Maximum,
    GreaterThan,
    LessThan,
    EqualTo,
}

enum Packet {
    Literal(i32, i64),
    Operator(i32, OperatorType, Vec<Box<Packet>>),
}

struct Day16 {
    vals: Vec<Packet>,
}

fn hex_to_bin(c: char) -> &'static str {
    let map = HashMap::from([
        ('0', "0000"),
        ('1', "0001"),
        ('2', "0010"),
        ('3', "0011"),
        ('4', "0100"),
        ('5', "0101"),
        ('6', "0110"),
        ('7', "0111"),
        ('8', "1000"),
        ('9', "1001"),
        ('A', "1010"),
        ('B', "1011"),
        ('C', "1100"),
        ('D', "1101"),
        ('E', "1110"),
        ('F', "1111"),
    ]);
    return map.get(&c).unwrap();
}

fn parse_packet(line: String) -> Packet {
    let mut bits = line
        .chars()
        .map(|c| hex_to_bin(c))
        .collect::<Vec<&str>>()
        .join("");
    return parse_single_packet(&mut bits);
}

fn parse_single_packet(bits: &mut String) -> Packet {
    let mut pull = |sz: usize| -> i32 {
        let ret = isize::from_str_radix(&bits[0..sz], 2).unwrap();
        bits.replace_range(0..sz, "");
        return ret as i32;
    };

    let version = pull(3);
    let optype = pull(3);
    if optype == 4 {
        let mut val: i64 = 0;
        loop {
            let chunk = pull(5);
            val = val * 16 + (chunk as i64 % 16);
            if chunk < 16 {
                break;
            }
        }
        return Packet::Literal(version, val);
    }

    let op_map = HashMap::from([
        (0, OperatorType::Sum),
        (1, OperatorType::Product),
        (2, OperatorType::Minimum),
        (3, OperatorType::Maximum),
        (5, OperatorType::GreaterThan),
        (6, OperatorType::LessThan),
        (7, OperatorType::EqualTo),
    ]);
    let optype = *op_map.get(&optype).unwrap();

    let len_type = pull(1);
    if len_type == 0 {
        let packet_amt = pull(15);
        let done_size = bits.len() - packet_amt as usize;
        let mut packets = Vec::new();
        while bits.len() > done_size {
            packets.push(Box::new(parse_single_packet(bits)));
        }
        return Packet::Operator(version, optype, packets);
    } else {
        let packet_count = pull(11);
        let mut packets = Vec::new();
        for _ in 0..packet_count {
            packets.push(Box::new(parse_single_packet(bits)));
        }
        return Packet::Operator(version, optype, packets);
    }
}

#[allow(dead_code)]
fn print_packet(packet: &Packet, indent: usize) {
    let indent_str = String::from_utf8(vec![b' '; indent]).unwrap();
    match packet {
        Packet::Literal(version, value) => {
            println!("{}A literal (v. {}): {}", indent_str, version, value);
        }
        Packet::Operator(version, optype, packets) => {
            println!(
                "{}An operator (v. {}) of type {:?}:",
                indent_str, version, optype
            );
            for nested in packets {
                print_packet(&nested, indent + 2);
            }
        }
    }
}

fn sum_versions(packet: &Packet) -> i32 {
    return match packet {
        Packet::Literal(version, _value) => *version,
        Packet::Operator(version, _optype, packets) => {
            *version + packets.iter().map(|n| sum_versions(n)).sum::<i32>()
        }
    };
}

fn eval_packet(packet: &Packet) -> i64 {
    return match packet {
        Packet::Literal(_version, value) => *value,
        Packet::Operator(_version, optype, packets) => {
            let nested = packets.iter().map(|n| eval_packet(n)).collect::<Vec<i64>>();
            match optype {
                OperatorType::Sum => nested.into_iter().sum::<i64>(),
                OperatorType::Product => nested.into_iter().product::<i64>(),
                OperatorType::Minimum => nested.into_iter().min().unwrap(),
                OperatorType::Maximum => nested.into_iter().max().unwrap(),
                OperatorType::GreaterThan => {
                    if nested[0] > nested[1] {
                        1
                    } else {
                        0
                    }
                }
                OperatorType::LessThan => {
                    if nested[0] < nested[1] {
                        1
                    } else {
                        0
                    }
                }
                OperatorType::EqualTo => {
                    if nested[0] == nested[1] {
                        1
                    } else {
                        0
                    }
                }
            }
        }
    };
}

impl BaseDay for Day16 {
    fn parse(&mut self, input: &mut InputReader) {
        self.vals = parse_lines(input, &mut |line: String| parse_packet(line));
    }

    fn pt1(&mut self) -> String {
        // for val in &self.vals {
        //     print_packet(&val, 0);
        // }
        return sum_versions(&self.vals[0]).to_string();
    }

    fn pt2(&mut self) -> String {
        return eval_packet(&self.vals[0]).to_string();
    }
}

fn main() {
    let mut day = Day16 { vals: Vec::new() };
    run_day(&mut day);
}
