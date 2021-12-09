extern crate common;

use std::collections::HashSet;

use common::framework::{parse_grid, run_day, BaseDay, InputReader};
use common::grid::{Coord, Grid, four_neighbors};

struct Day09 {
    vals: Grid<i32>,
}

fn is_lower(a: &Coord, b: &Coord, grid: &Grid<i32>) -> bool {
    return grid.coords.get(a).unwrap_or(&10) < grid.coords.get(b).unwrap_or(&10);
}

impl BaseDay for Day09 {
    fn parse(&mut self, input: &mut InputReader) {
        fn parse_coord(c: char, _coord: &Coord) -> i32 {
            return c.to_digit(10).unwrap() as i32;
        }

        self.vals = parse_grid(input, &mut parse_coord);
    }

    fn pt1(&mut self) -> String {
        let mut risk = 0;
        for (coord, val) in self.vals.coords.iter() {
            if four_neighbors(coord).iter().all(|n| is_lower(coord, n, &self.vals)) {
                risk += 1 + val;
            }
        }
        return risk.to_string();
    }

    fn pt2(&mut self) -> String {
        let mut unassigned: HashSet<Coord> = self.vals.coords.iter().filter_map(|(k, v)| if *v < 9 { Some(*k) } else { None }).collect();
        let mut basins: Vec<HashSet<Coord>> = vec![];

        while !unassigned.is_empty() {
            let coord: Coord = *unassigned.iter().next().unwrap();
            //println!("Checking {:?}", coord);
            unassigned.remove(&coord);

            let nghs: Vec<Coord> = four_neighbors(&coord);
            let mut cur_basin: Option<usize> = None;
            for idx in 0..basins.len() {
                if basins[idx].contains(&coord) {
                    cur_basin = Some(idx);
                    break;
                }
            }

            for ngh in nghs {
                if unassigned.contains(&ngh) {
                    // neighbor not assigned a basin yet
                    // are we assigned a basin yet?
                    if cur_basin.is_none() {
                        cur_basin = Some(basins.len());
                        basins.push(HashSet::from([coord, ngh]));
                        //println!("...neighbor {:?} is unassigned, goes to new basin ({}) with us", ngh, cur_basin.unwrap());
                    } else {
                        basins[cur_basin.unwrap()].insert(ngh);
                        //println!("...neighbor {:?} is unassigned, goes to our basin {}", ngh, cur_basin.unwrap());
                    }
                } else {
                    let nv = *self.vals.coords.get(&ngh).unwrap_or(&10);
                    if nv >= 9 {
                        continue;
                    }

                    let mut ngh_basin: Option<usize> = None;
                    for idx in 0..basins.len() {
                        if basins[idx].contains(&ngh) {
                            ngh_basin = Some(idx);
                            break;
                        }
                    }
                    // are we assigned a basin yet?
                    if cur_basin.is_none() {
                        basins[ngh_basin.unwrap()].insert(coord);
                        cur_basin = ngh_basin;
                        //println!("...neighbor {:?} has a basin, we go to it {}", ngh, ngh_basin.unwrap());
                    } else {
                        // don't need to get rid of it, just empty it out
                        // (getting rid of would screw up our indices maybe)
                        // (we copy and drain )
                        let cur_items: HashSet<Coord> = basins[cur_basin.unwrap()].drain().collect();
                        basins[ngh_basin.unwrap()].extend(cur_items);
                        //println!("...neighbor {:?} has a basin, we go from {} to {}", ngh, cur_basin.unwrap(), ngh_basin.unwrap());
                        cur_basin = ngh_basin;
                    }
                }
            }
        }

        for b in &basins {
            if b.len() > 0 {
                println!("Found basin of size {}: {:?}", b.len(), b);
            }
        }
        basins.sort_by(|a, b| b.len().cmp(&a.len()));
        let amt: i32 = basins.iter().take(3).fold(1, |tot, val| tot * val.len() as i32);
        return amt.to_string();
    }
}

fn main() {
    let mut day = Day09 { vals: Grid::new() };
    run_day(&mut day);
}
