#[macro_use] extern crate failure;
#[macro_use] extern crate lazy_static;

use failure::{Error, bail};
use itertools::Itertools;
use regex::Regex;
use std::collections::{HashSet, HashMap};
use std::io::{BufReader, BufRead};
use std::fs::File;

#[derive(Debug)]
struct Tile {
    id: i64,
    data: HashSet<(i64, i64)>,
    edges: Vec<String>,
}

#[derive(Debug)]
struct ImageData {
    tiles: Vec<Tile>,
    tile_len: i64,  // length in chars of a single tile side
    tile_count: i64, // length in tiles of an image side
}

#[derive(Debug)]
struct Image {
    data: HashSet<(i64, i64)>,
    image_len: i64, // length in chars of image side
}

// map of tile_id -> tile_ids+reverse_flag it joins (or Nones if empty)
type PairingMap = HashMap<i64, Vec<Option<(i64, bool)>>>;

// map of x, y tile coordinates -> tile_id, unrotation
type AssemblyMap = HashMap<(i64, i64), (i64, i64)>;

// Our edges are marked clockwise from the top:
//     0
//   3   1
//     2
// The first four rotations are as-is, and then rotated clockwise 1 step each
//     0        3        2        1
//   3   1    2   0    1   3    0   2
//     2        1        0        3
// The next four are the horizontal flips
//     0        3        2        1
//   1   3    0   2    3   1    2   0
//     2        1        0        3

fn parse_tile(buf: &Vec<String>) -> Result<(Tile, i64), Error> {
    lazy_static! {
        static ref TITLE_RE: Regex = Regex::new(r"^Tile (\d+):$").unwrap();
    }
    let mut ret = Tile { id: -1, data: HashSet::new(), edges: Vec::new() };
    if let Some(title_caps) = TITLE_RE.captures(&buf[0]) {
        ret.id = title_caps.get(1).unwrap().as_str().parse()?;
    } else {
        bail!("Bad title line: {}", buf[0]);
    }

    let tile_len = buf[0].len() as i64;
    for _ in 0..4 {
        let mut edge = String::new();
        edge.reserve(tile_len as usize);
        ret.edges.push(edge);
    }

    for iy in 1..buf.len() {
        let y = (iy - 1) as i64;
        for (x, c) in buf[iy].chars().enumerate() {
            let x = x as i64;
            if c == '#' {
                ret.data.insert((x, y));
            } else if c == '.' {
                /* continue */
            } else {
                bail!("Unknown tile char: {}", c);
            }
            if y == 0 {
                ret.edges[0].push(c);
            }
            if x == tile_len - 1 {
                ret.edges[1].push(c);
            }
            if y == tile_len - 1 {
                ret.edges[2].push(c);
            }
            if x == 0 {
                ret.edges[3].push(c);
            }
        }
    }

    return Ok((ret, tile_len));
}

fn read_input(file: &str) -> Result<ImageData, Error> {
    let f = File::open(file)?;
    let br = BufReader::new(f);

    let mut buf = Vec::new();

    let mut ret = ImageData { tiles: Vec::new(), tile_len: -1, tile_count: -1 };

    for line in br.lines() {
        let line = line?;
        if line.len() > 0 {
            buf.push(line);
        } else if buf.len() > 0 {
            let (tile, tl) = parse_tile(&buf)?;
            ret.tile_len = tl;
            ret.tiles.push(tile);
            buf.clear();
        }
    }
    if buf.len() > 0 {
        let (tile, tl) = parse_tile(&buf)?;
        ret.tile_len = tl;
        ret.tiles.push(tile);
        buf.clear();
    }

    ret.tile_count = (ret.tiles.len() as f64).sqrt() as i64;

    return Ok(ret);
}

fn make_pairing_map(image_data: &ImageData) -> PairingMap {
    let mut all_edges = HashMap::new();
    for tile in &image_data.tiles {
        for (idx, edge) in tile.edges.iter().enumerate() {
            let rev = edge.chars().rev().collect::<String>();
            let both = vec![(edge.clone(), idx as i64), (rev, idx as i64 + 4)];
            for (cur, cur_idx) in both {
                let entry = all_edges.entry(cur).or_insert(Vec::new());
                entry.push((tile.id, cur_idx));
            }
        }
    }

    let mut ret = HashMap::new();
    for (edge_str, nghs) in all_edges {
        if nghs.len() == 1 {
            // exterior edge, put in no entry
        } else if nghs.len() == 2 {
            // interior edge. because we put in reversed and non-reversed
            // form, there'll be two entries joining these tiles, one like
            // [(TID_1, 0), (TID_2, 6)] and one like [(TID_1, 4), (TID_2, 2)]
            // for simplicity, we only put in an entry where the key is the
            // unflipped form
            if nghs[0].1 < 4 {
                let entry = ret.entry(nghs[0].0).or_insert(vec![None; 4]);
                entry[nghs[0].1 as usize] = Some((nghs[1].0, nghs[1].1 >= 4));
            }
            if nghs[1].1 < 4 {
                let entry = ret.entry(nghs[1].0).or_insert(vec![None; 4]);
                entry[nghs[1].1 as usize] = Some((nghs[0].0, nghs[0].1 >= 4));
            }
        } else {
            panic!("Uh-oh, ambiguous edge match for {}", edge_str);
        }
    }
    return ret;
}

fn find_corners(pairing_map: &PairingMap) -> Vec<i64> {
    return pairing_map.iter()
        .filter_map(|(tid, nghs)| if nghs.iter().filter(|n| n.is_some()).count() == 2 { Some(*tid) } else { None })
        .sorted()
        .collect();
}

fn assemble(image_data: &ImageData, pairing_map: &PairingMap, starting_corner: i64) -> AssemblyMap {
    let mut ret = HashMap::new();
    let mut placed = HashSet::new();
    for y in 0..image_data.tile_count {
        for x in 0..image_data.tile_count {
            let (tid, top_id, left_id) =
                find_one(x, y, image_data, pairing_map, starting_corner, &placed, &mut ret);
            placed.insert(tid);
            let unrot = compute_unrotation(tid, top_id, left_id, image_data, pairing_map);
            ret.insert((x, y), (tid, unrot));
        }
    }
    return ret;
}

fn find_one(x: i64, y: i64, image_data: &ImageData, pairing_map: &PairingMap, starting_corner: i64, placed: &HashSet<i64>, assembly_map: &mut AssemblyMap) -> (i64, Option<i64>, Option<i64>) {
    let to_edge = |ngh: Option<i64>| -> Option<i64> {
        let is_edge = ngh.map_or(false, |tid| {
            pairing_map.get(&tid).unwrap().iter()
                .filter(|n| n.is_some())
                .count() < 4
        });
        if is_edge { ngh } else { None }
    };

    if x == 0 && y == 0 {
        return (starting_corner, None, None);
    } else if y == 0 {
        let left_id = assembly_map.get(&(x - 1, y)).unwrap().0;
        // find the first (may be 2 if our left one is a corner)
        // neighbor of our left tile that is itself an edge piece
        let tid = pairing_map.get(&left_id).unwrap().iter()
            .filter_map(|ngh| to_edge(ngh.map(|n| n.0)))
            .filter(|t| !placed.contains(t))
            .nth(0).unwrap();
        return (tid, None, Some(left_id));
    } else if x == 0 {
        let top_id = assembly_map.get(&(x, y - 1)).unwrap().0;
        // find the first (may be 2 if our top one is a corner)
        // neighbor of our top tile that is itself an edge piece
        let tid = pairing_map.get(&top_id).unwrap().iter()
            .filter_map(|ngh| to_edge(ngh.map(|n| n.0)))
            .filter(|t| !placed.contains(t))
            .nth(0).unwrap();
        return (tid, Some(top_id), None);
    } else {
        let top_id = assembly_map.get(&(x, y - 1)).unwrap().0;
        let left_id = assembly_map.get(&(x - 1, y)).unwrap().0;
        // find the neighbor in common between left and top tiles
        let top_ns : HashSet<i64> = pairing_map.get(&top_id).unwrap().iter().filter_map(|n| n.map(|t| t.0)).collect();
        let left_ns : HashSet<i64> = pairing_map.get(&left_id).unwrap().iter().filter_map(|n| n.map(|t| t.0)).collect();
        let tid = top_ns.intersection(&left_ns).filter(|t| !placed.contains(t)).nth(0).unwrap();
        return (*tid, Some(top_id), Some(left_id));
    }
}

fn compute_unrotation(tid: i64, top_id: Option<i64>, left_id: Option<i64>, image_data: &ImageData, pairing_map: &PairingMap) -> i64 {
    let nghs = pairing_map.get(&tid).unwrap();
    let top_rotations = compute_unrotation_one(top_id, false, &nghs);
    let left_rotations = compute_unrotation_one(left_id, true, &nghs);
    let isect : HashSet<i64> = top_rotations.intersection(&left_rotations).map(|x| *x).collect();
    println!("Top rotations for {}: {:?}", tid, top_rotations);
    println!("Left rotations for {}: {:?}", tid, left_rotations);
    if isect.len() > 1 {
        println!("Found multiple rotations for {}: {:?}", tid, isect);
    }
    return isect.into_iter().sorted().nth(0).unwrap();
}

fn compute_unrotation_one(other_id: Option<i64>, is_left: bool, nghs: &Vec<Option<(i64, bool)>>) -> HashSet<i64> {
    let pick_rot = |idx: usize, is_left: bool| -> i64 {
        // based on how our rotations and edges are labelled,
        // if X is the edge num we've found on top, the rotation num is 4 - X
        // if it's a left edge, the rotation is 3 - X
        if is_left {
            return 3 - (idx as i64);
        } else {
            // is_top
            return (4 - (idx as i64)) % 4;
        }
    };

    if other_id.is_none() {
        // for external edges, we don't know if they should be flipped or not, so try both
        return (0..4_usize).filter(|i| nghs[*i].is_none())
            .map(|en| pick_rot(en, is_left))
            .flat_map(|en| vec![en, en + 4])
            .collect();
    }

    let other_id = other_id.unwrap();
    return (0..4_usize).filter_map(|i| {
        if let Some(ngh) = nghs[i] {
            if ngh.0 == other_id {
                println!("FOUND AT {}", i);
                Some(pick_rot(i, is_left) + if ngh.1 { 4 } else { 0 })
            } else {
                None
            }
        } else {
            None
        }
    }).collect();
}

// fn make_image(image_data: &ImageData, assembly_map: &AssemblyMap) -> Image {
//
// }

fn print_unrot(data: HashSet<(i64, i64)>, side_len: i64, rotation: i64) {
    for i in 0..side_len {
        for j in 0..side_len {
            let (x, y) =
                if rotation == 0 {
                    (j, i)
                } elif rotation == 1 {

                }
            let ch = if data.contains(&(x, y)) { '#' } else { '.' };
            print(ch);
        }
        println!();
    }
}

fn main() {
    let args : Vec<String> = std::env::args().collect();
    if args[1] == "1" {
        println!("Doing part 1");
        let image_data = read_input(&args[2]).unwrap();
        let verbose = args.len() > 3 && args[3].chars().nth(0).unwrap() == 't';
        let pairing_map = make_pairing_map(&image_data);
        let corners = find_corners(&pairing_map);
        assert_eq!(corners.len(), 4);
        println!("Corners: {}, {}, {}, {} = {}",
            corners[0], corners[1], corners[2], corners[3], corners.iter().fold(1, |acc, x| acc * *x));
    } else {
        println!("Doing part 2");
        let image_data = read_input(&args[2]).unwrap();
        let verbose = args.len() > 3 && args[3].chars().nth(0).unwrap() == 't';
        let pairing_map = make_pairing_map(&image_data);
        let corners = find_corners(&pairing_map);
        assert_eq!(corners.len(), 4);
        let assembly_map = assemble(&image_data, &pairing_map, corners[1]);

        for y in 0..image_data.tile_count {
            for x in 0..image_data.tile_count {
                let tile = assembly_map.get(&(x, y)).unwrap();
                println!("At {},{}: {} {}", x, y, tile.0, tile.1);
            }
        }
    }
}
