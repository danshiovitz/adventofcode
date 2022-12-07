use lazy_regex::regex;
use std::collections::HashMap;

extern crate common;

use common::framework::{parse_lines, run_day, BaseDay, InputReader};

#[derive(Debug)]
enum Node {
    File {
        parent: usize,
        size: i32,
    },
    Directory {
        parent: usize,
        nodes: HashMap<String, usize>,
    },
}

struct Day07 {
    nodes: Vec<Node>,
}

fn get_parent(cur: usize, all_nodes: &Vec<Node>) -> usize {
    match &all_nodes[cur] {
        Node::Directory { parent, .. } => *parent,
        Node::File { parent, .. } => *parent,
    }
}

fn add_subdir(cur: usize, name: String, all_nodes: &mut Vec<Node>) -> usize {
    let new_idx = all_nodes.len();
    if let Node::Directory { parent: _, nodes } = &mut all_nodes[cur] {
        if let Some(node_idx) = nodes.get(&name) {
            return *node_idx;
        } else {
            nodes.insert(name, new_idx);
            all_nodes.push(Node::Directory { parent: cur, nodes: HashMap::new() });
            return new_idx;
        }
    }
    panic!("Tried to add subdir to {}, which isn't a dir", cur);
}

fn add_file(cur: usize, name: String, size: i32, all_nodes: &mut Vec<Node>) -> usize {
    let new_idx = all_nodes.len();
    if let Node::Directory { parent: _, nodes } = &mut all_nodes[cur] {
        if let Some(node_idx) = nodes.get(&name) {
            return *node_idx;
        } else {
            nodes.insert(name, new_idx);
            all_nodes.push(Node::File { parent: cur, size: size });
            return new_idx;
        }
    }
    panic!("Tried to add file to {}, which isn't a dir", cur);
}

fn compute_size(cur: usize, all_nodes: &Vec<Node>, sizes: &mut HashMap<usize, i32>) -> i32 {
    if let Some(sz) = sizes.get(&cur) {
        return *sz;
    }
    match &all_nodes[cur] {
        Node::Directory { parent: _, nodes } => {
            let sz = nodes
                .values()
                .map(|ni| compute_size(*ni, all_nodes, sizes))
                .sum::<i32>();
            sizes.insert(cur, sz);
            return sz;
        }
        Node::File { parent: _, size } => {
            return *size;
        }
    }
}

fn print_tree(cur: usize, indent: i32, sizes: &HashMap<usize, i32>, all_nodes: &Vec<Node>) {
    for _ in 0..indent {
        print!(" ");
    }
    println!("{}: {}", cur, sizes.get(&cur).unwrap_or(&0));
    match &all_nodes[cur] {
        Node::Directory { parent: _, nodes } => {
            for n in nodes.values() {
                print_tree(*n, indent + 2, &sizes, &all_nodes);
            }
        }
        Node::File { parent: _, size: _ } => (),
    }
}
// print_tree(0, 0, &sizes, &self.nodes);

impl BaseDay for Day07 {
    fn parse(&mut self, input: &mut InputReader) {
        self.nodes
            .push(Node::Directory { parent: 0, nodes: HashMap::new() });
        let mut cur = 0;

        parse_lines(input, &mut |line: String| {
            let rex = regex!(r#"\$\s+cd\s+(.*)"#);
            match rex.captures(&line) {
                Some(c) => {
                    if &c[1] == ".." {
                        cur = get_parent(cur, &self.nodes);
                    } else if &c[1] == "/" {
                        cur = 0;
                    } else {
                        cur = add_subdir(cur, c[1].to_string(), &mut self.nodes);
                    }
                    return ();
                }
                None => (),
            }
            let rex = regex!(r#"\$\s+(.*)"#);
            match rex.captures(&line) {
                Some(c) => {
                    if &c[1] == "ls" {
                        // Don't actually need to do anything with it
                    } else {
                        panic!("Unknown command: {}", &c[1]);
                    }
                    return ();
                }
                None => (),
            }
            let rex = regex!(r#"dir\s+(.*)"#);
            match rex.captures(&line) {
                Some(c) => {
                    add_subdir(cur, c[1].to_string(), &mut self.nodes);
                    return ();
                }
                None => (),
            }
            let rex = regex!(r#"(\d+)\s+(.*)"#);
            match rex.captures(&line) {
                Some(c) => {
                    add_file(
                        cur,
                        c[2].to_string(),
                        c[1].parse::<i32>().unwrap(),
                        &mut self.nodes,
                    );
                    return ();
                }
                None => (),
            }
            panic!("Unknown line: {}", line);
        });
    }

    fn pt1(&mut self) -> String {
        let mut sizes: HashMap<usize, i32> = HashMap::new();
        compute_size(0, &self.nodes, &mut sizes);
        let tot = sizes
            .values()
            .map(|sz| *sz)
            .filter(|&sz| sz <= 100000)
            .sum::<i32>();
        return tot.to_string();
    }

    fn pt2(&mut self) -> String {
        let mut sizes: HashMap<usize, i32> = HashMap::new();
        compute_size(0, &self.nodes, &mut sizes);

        let root_size = sizes.get(&0).unwrap();
        let needed_size = root_size - 40000000;
        let mut deletable = sizes
            .values()
            .map(|sz| *sz)
            .filter(|&sz| sz >= needed_size)
            .collect::<Vec<i32>>();
        deletable.sort();
        return deletable[0].to_string();
    }
}

fn main() {
    let mut day = Day07 { nodes: Vec::new() };
    run_day(&mut day);
}
