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
                ret.edges[2].insert(0, c);
            }
            if x == 0 {
                ret.edges[3].insert(0, c);
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
            ret.insert((x, y), (tid, 0));
        }
    }
    return ret;
}

fn rotate_pieces(image_data: &ImageData, pairing_map: &PairingMap, assembly_map: &mut AssemblyMap) -> () {
    let mut top_priors = HashMap::new();
    let mut left_priors = HashMap::new();
    let second_id = assembly_map.get(&(1, 0)).unwrap().0;
    for y in 0..image_data.tile_count {
        for x in 0..image_data.tile_count {
            let val = assembly_map.get_mut(&(x, y)).unwrap();
            let tid = val.0;
            let top_id = top_priors.remove(&(x, y));
            let left_id = left_priors.remove(&(x, y));
            let unrot = compute_unrotation(tid, second_id, top_id, left_id, image_data, pairing_map);
            val.1 = unrot;

            top_priors.insert((x, y+1), (tid, unrot >= 4));
            left_priors.insert((x+1, y), (tid, unrot >= 4));
        }
    }
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

fn compute_unrotation(tid: i64, second_id: i64, top_id: Option<(i64, bool)>, left_id: Option<(i64, bool)>, image_data: &ImageData, pairing_map: &PairingMap) -> i64 {
    let nghs = pairing_map.get(&tid).unwrap();
    let top_rotations = compute_unrotation_one(top_id, false, &nghs);
    let left_rotations = compute_unrotation_one(left_id, true, &nghs);
    let mut isect : HashSet<i64> = top_rotations.intersection(&left_rotations).map(|x| *x).collect();
    // println!("Top rotations for {}: {:?}", tid, top_rotations);
    // println!("Left rotations for {}: {:?}", tid, left_rotations);
    if isect.len() > 1 {
        println!("Found multiple rotations for {}: {:?}", tid, isect);
        assert!(top_id.is_none());
        assert!(left_id.is_none());
        // of these ambiguous rotations, we have to pick the one that matches our right tile
        // (if we were placing and then rotating one at a time, we wouldn't have to, but
        // we've already determined placement so we have to use the one that matches the next
        // tile)
        let unrot_idx = |rotation: i64| {
            // math here seems hokey to the extreme
            if rotation <= 1 {
                return 1 - rotation;
            } else if rotation <= 3 {
                return 5 - rotation;
            } else {
                return 7 - rotation;
            }
        };
        isect.retain(|rot: &i64| {
            let idx = unrot_idx(*rot) as usize;
            if let Some(val) = nghs[idx] {
                return val.0 == second_id;
            } else {
                return false;
            }
        });
    }
    return isect.into_iter().nth(0).unwrap();
}

fn compute_unrotation_one(other_id: Option<(i64, bool)>, is_left: bool, nghs: &Vec<Option<(i64, bool)>>) -> HashSet<i64> {
    let pick_rot = |idx: usize, reverse: bool, is_left: bool| -> i64 {
        if is_left {
            let mut rot = 3 - (idx as i64);
            if reverse {
                rot = (rot + 2) % 4 + 4;
            }
            return rot;
        } else {
            return (4 - (idx as i64)) % 4 + (if reverse { 4 } else { 0 });
        }
    };

    if other_id.is_none() {
        // for external edges, we don't know if they should be flipped or not, so try both
        return (0..4_usize).filter(|i| nghs[*i].is_none())
            .flat_map(|en| vec![pick_rot(en, false, is_left), pick_rot(en, true, is_left)])
            .collect();
    }

    let other_id = other_id.unwrap();
    return (0..4_usize).filter_map(|i| {
        if let Some(ngh) = nghs[i] {
            if ngh.0 == other_id.0 {
                // reverse flag is true if we had to flip edges to match up
                // and their tile isn't flipped, or we didn't flip but they're
                // flipped. except, sigh, top/bottom and left/right actually need
                // to be reversed to match up normally, so invert that.
                let use_reverse = ngh.1 == other_id.1;
                Some(pick_rot(i, use_reverse, is_left))
            } else {
                None
            }
        } else {
            None
        }
    }).collect();
}

fn make_image(image_data: &ImageData, assembly_map: &AssemblyMap) -> Image {
    let info : HashMap<i64, (i64, (i64, i64))> =
        assembly_map.iter().map(|(k, (t, r))| (*t, (*r, *k))).collect();

    let tile_len = image_data.tile_len;
    let tile_count = image_data.tile_count;
    let mut ret = Image { data: HashSet::new(), image_len: tile_count * (tile_len - 2)};
    for tile in &image_data.tiles {
        let cur_info = info.get(&tile.id).unwrap();
        let rotation = cur_info.0;
        let unrotated = rotate_data(&tile.data, tile_len, rotation);
        let converted = unrotated.iter()
            .filter(|(x, y)| *x != 0 && *y != 0 && *x != tile_len - 1 && *y != tile_len - 1)
            .map(|(x, y)| {
                // moved down because we chopped off the borders:
                let mut i = x - 1;
                let mut j = y - 1;
                i += cur_info.1.0 * (tile_len - 2); // -2 because of borders
                j += cur_info.1.1 * (tile_len - 2);
                (i, j)
            });
        ret.data.extend(converted);
    }
    return ret;
}

fn rotate_data(data: &HashSet<(i64, i64)>, side_len: i64, rotation: i64) -> HashSet<(i64, i64)> {
    let mut ret = HashSet::new();
    for i in 0..side_len {
        for j in 0..side_len {
            let (mut x, mut y) = (j, i);
            if rotation >= 4 {
                x = side_len - x - 1;
            }
            for _ in 0..(rotation % 4) {
                let tmp_x = x;
                x = y;
                y = side_len - tmp_x - 1;
            }
            if data.contains(&(x, y)) {
                ret.insert((j, i));
            }
        }
    }
    return ret;
}

fn unrotate_data(data: &HashSet<(i64, i64)>, side_len: i64, rotation: i64) -> HashSet<(i64, i64)> {
    let new_rotation = if rotation < 4 {
        (4 - rotation) % 4
    } else {
        rotation
    };
    return rotate_data(data, side_len, new_rotation);
}

fn print_data(data: &HashSet<(i64, i64)>, side_len: i64) -> () {
    for y in 0..side_len {
        for x in 0..side_len {
            let ch = if data.contains(&(x, y)) { '#' } else { '.' };
            print!("{}", ch);
        }
        println!();
    }
}

fn find_sea_monsters(data: &HashSet<(i64, i64)>, side_len: i64) -> HashSet<(i64, i64)> {
    let offsets : Vec<(i64, i64)> = vec![
        (18, 0),
        (0, 1), (5, 1), (6, 1), (11, 1), (12, 1), (17, 1), (18, 1), (19, 1),
        (1, 2), (4, 2), (7, 2), (10, 2), (13, 2), (16, 2),
    ];
    let max_x = side_len - offsets.iter().map(|o| o.0).max().unwrap() + 1;
    let max_y = side_len - offsets.iter().map(|o| o.1).max().unwrap() + 1;
    let mut ret = HashSet::new();
    for y in 0..max_y {
        for x in 0..max_x {
            let monster : Vec<(i64, i64)> = offsets.iter().map(|o| (x + o.0, y + o.1)).collect();
            if monster.iter().all(|m| data.contains(m)) {
                ret.extend(monster);
            }
        }
    }
    return ret;
}

fn print_with_sea_monsters(data: &HashSet<(i64, i64)>, sea_monsters: &HashSet<(i64, i64)>, side_len: i64) -> () {
    for y in 0..side_len {
        for x in 0..side_len {
            let ch =
                if data.contains(&(x, y)) {
                    if sea_monsters.contains(&(x, y)) {
                        'O'
                    } else {
                        '#'
                    }
                } else {
                    '.'
                };
            print!("{}", ch);
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
    } else if args[1] == "2" {
        println!("Doing part 2");
        let image_data = read_input(&args[2]).unwrap();
        let verbose = args.len() > 3 && args[3].chars().nth(0).unwrap() == 't';
        let pairing_map = make_pairing_map(&image_data);
        let corners = find_corners(&pairing_map);
        assert_eq!(corners.len(), 4);
        let mut assembly_map = assemble(&image_data, &pairing_map, corners[0]);
        rotate_pieces(&image_data, &pairing_map, &mut assembly_map);
        let image = make_image(&image_data, &assembly_map);

        // for y in 0..image_data.tile_count {
        //     for x in 0..image_data.tile_count {
        //         let tile = assembly_map.get(&(x, y)).unwrap();
        //         println!("At {},{}: {} {}", x, y, tile.0, tile.1);
        //     }
        // }

        // print_data(&image.data, image.image_len);
        for rot in 0..8 {
            let rotated = rotate_data(&image.data, image.image_len, rot);
            let monsters = find_sea_monsters(&rotated, image.image_len);
            if monsters.len() > 0 {
                println!("Rotation: {}", rot);
                print_with_sea_monsters(&rotated, &monsters, image.image_len);
                println!();
                println!("Unoccupied water: {}", rotated.difference(&monsters).count());
            }
        }
    } else if args[1] == "test_rotate_unrotate" {
        println!("Doing test");
        let data : HashSet<(i64, i64)> = vec![
            (1, 1), (2, 1), (3, 1), (4, 1), (1, 2),
        ].into_iter().collect();
        for rot in 0..8 {
            let rotated = rotate_data(&data, 8, rot);
            println!("Rotation {}", rot);
            print_data(&rotated, 8);
            let unrotated = unrotate_data(&rotated, 8, rot);
            if unrotated != data {
                println!("Rotation mismatch:");
                print_data(&unrotated, 8);
            } else {
                println!("Unrotated ok");
            }
            println!();
        }
    } else if args[1] == "test_rotate_pieces" {
        println!("Doing test");
        let image_data = read_input(&args[2]).unwrap();
        let pairing_map = make_pairing_map(&image_data);
        let mut assembly_map : AssemblyMap = vec![
            ((0, 0), (1951, 0)),
            ((1, 0), (2311, 0)),
            ((2, 0), (3079, 0)),
            ((0, 1), (2729, 0)),
            ((1, 1), (1427, 0)),
            ((2, 1), (2473, 0)),
            ((0, 2), (2971, 0)),
            ((1, 2), (1489, 0)),
            ((2, 2), (1171, 0)),
        ].into_iter().collect();
        rotate_pieces(&image_data, &pairing_map, &mut assembly_map);
        let image = make_image(&image_data, &assembly_map);
        print_data(&image.data, image.image_len);

        // for y in 0..image_data.tile_count {
        //     for x in 0..image_data.tile_count {
        //         let tile = assembly_map.get(&(x, y)).unwrap();
        //         println!("At {},{}: {} {}", x, y, tile.0, tile.1);
        //     }
        // }
    } else if args[1] == "test_unrotate_only" {
        println!("Doing test");
        let image_data = read_input(&args[2]).unwrap();
        let tid : i64 = args[3].parse().unwrap();
        let rotation : i64 = args[4].parse().unwrap();
        let tile = image_data.tiles.iter().filter(|t| t.id == tid).nth(0).unwrap();
        println!("Raw:");
        print_data(&tile.data, image_data.tile_len);
        println!("");
        for edge in &tile.edges {
            println!("Edge: {}", edge);
        }
        println!("");
        let unrotated = unrotate_data(&tile.data, image_data.tile_len, rotation);
        println!("Unrotated:");
        print_data(&unrotated, image_data.tile_len);
        println!();
    }
}
