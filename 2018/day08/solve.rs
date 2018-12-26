//#[macro_use] extern crate lazy_static;

use failure::{Error,err_msg};
use std::boxed::Box;
use std::io::{BufReader,BufRead};
use std::iter::Iterator;
use std::fs::File;

#[derive(Debug)]
struct Node {
    metadata: Vec<i32>,
    children: Vec<Box<Node>>,
}

fn parse_node<'a, I>(vals: &mut I) -> Result<Box<Node>, Error> where
    I: Iterator<Item = i32>,
{
    let c_count = vals.next().ok_or(err_msg("Expected child count"))?;
    let m_count = vals.next().ok_or(err_msg("Expected metadata count"))?;
    let children = (0..c_count).map(|_| parse_node(vals)).collect::<Result<Vec<_>, _>>()?;
    let metadata = (0..m_count).map(|_| vals.next().ok_or(err_msg("Expected metadata"))).collect::<Result<Vec<_>, _>>()?;
    return Ok(Box::new(Node { children, metadata }));
}

fn read_input(file: &str) -> Result<Box<Node>, Error> {
    let f = File::open(file)?;
    let br = BufReader::new(f);
    let lines : Vec<String> = br.lines().collect::<Result<Vec<_>, _>>()?;
    assert_eq!(lines.len(), 1);
    let mut vals = lines[0].split(" ").map(|s| s.parse::<i32>().unwrap());
    let node = parse_node(&mut vals)?;
    assert_eq!(vals.next(), None);
    return Ok(node);
}

fn calc_metadata_sum(root: &Box<Node>) -> i32 {
    let node : &Node = root;
    let local_sum : i32 = node.metadata.iter().sum();
    let child_sum : i32 = node.children.iter().map(|c| calc_metadata_sum(&c)).sum();
    return local_sum + child_sum;
}

fn calc_node_value(root: &Box<Node>) -> i32 {
    let node : &Node = root;
    if node.children.len() == 0 {
        return node.metadata.iter().sum();
    } else {
        return node.metadata.iter().map(|m| m - 1).filter(|m| m >= &0 && m < &(node.children.len() as i32)).
            map(|m| calc_node_value(&node.children[m as usize])).sum();
    }
}

fn main() {
    let args : Vec<String> = std::env::args().collect();
    let nodes = read_input(&args[1]).unwrap();
    let md_sum = calc_metadata_sum(&nodes);
    println!("Calculated metadata sum = {}", md_sum);
    let root_value = calc_node_value(&nodes);
    println!("Calculated root value = {}", root_value);
}
