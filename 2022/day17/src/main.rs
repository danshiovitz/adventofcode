use std::collections::{HashMap, HashSet};

extern crate common;

use common::framework::{parse_lines, run_day, BaseDay, InputReader};
use common::grid::{add_direction, print_grid, Coord, Direction, Grid};

struct Day17 {
    vals: Vec<Direction>,
}

fn print_it(coords: &HashSet<Coord>, falling: &Vec<Coord>) {
    let mut grid: Grid<char> = Grid {
        coords: coords.iter().map(|c| (*c, '#')).collect(),
        min: Coord { x: 0, y: 0 },
        max: Coord { x: 0, y: 0 },
    };
    grid.coords.extend(falling.iter().map(|c| (*c, '@')));
    grid.recompute_minmax();
    grid.min = Coord { x: 0, y: grid.min.y };
    grid.max = Coord { x: 6, y: 0 };
    print_grid(&grid, &mut |_c, ch: Option<&char>| {
        ch.unwrap_or(&'.').to_string()
    });
    println!();
}

fn drop_rocks(
    tetrominoes_list: &Vec<Vec<Direction>>,
    jets_list: &Vec<Direction>,
    width: i32,
    rounds: i64,
) -> i64 {
    let mut grid = HashSet::new();
    let mut max_heights = (0..width).map(|_| 1 as i32).collect::<Vec<i32>>();
    let mut tetrominoes = (0..tetrominoes_list.len()).into_iter().cycle();
    let mut jets = (0..jets_list.len()).into_iter().cycle();

    let drop = Direction { dx: 0, dy: 1 };
    let not_overlap =
        |c: &Coord, grid: &HashSet<Coord>| c.x >= 0 && c.x < width && c.y <= 0 && !grid.contains(c);
    // height profile, tetromino_idx, jet_idx -> round
    let mut cache: HashMap<(Vec<i32>, usize, usize), i64> = HashMap::new();
    // round -> height profile, actual max height
    let mut inv_cache: HashMap<i64, (Vec<i32>, i32)> = HashMap::new();

    for cur_round in 0..rounds {
        let start = Coord { x: 2, y: max_heights.iter().min().unwrap() - 4 };
        let t_idx = tetrominoes.next().unwrap();
        let ts = &tetrominoes_list[t_idx];
        let mut cur = ts
            .into_iter()
            .map(|t| add_direction(&start, &t))
            .collect::<Vec<Coord>>();

        loop {
            let j_idx = jets.next().unwrap();
            let jet = &jets_list[j_idx];
            let pushed = cur
                .iter()
                .map(|c| add_direction(c, jet))
                .filter(|c| not_overlap(c, &grid))
                .collect::<Vec<Coord>>();
            if pushed.len() == cur.len() {
                cur = pushed;
                // println!("Pushing {:?}", jet);
                // print_it(&grid, &cur);
            } else {
                // println!("Not pushed {:?}", jet);
            }
            let dropped = cur
                .iter()
                .map(|c| add_direction(c, &drop))
                .filter(|c| not_overlap(c, &grid))
                .collect::<Vec<Coord>>();
            if dropped.len() == cur.len() {
                cur = dropped;
                // println!("Dropping");
                // print_it(&grid, &cur);
            } else {
                // bonk
                for c in &cur {
                    // despite "max" it's actually negative, right
                    if c.y < max_heights[c.x as usize] {
                        max_heights[c.x as usize] = c.y;
                    }
                }
                grid.extend(cur);
                // print_it(&grid, &vec![]);

                let mh = max_heights.iter().min().unwrap();
                let normalized = max_heights.iter().map(|m| mh - m).collect::<Vec<i32>>();
                let cache_key = (normalized, t_idx, j_idx);
                if let Some(prev_round) = cache.get(&cache_key) {
                    println!("Found cycle between {} and {}", prev_round, cur_round);
                    let (_prev_heights, prev_actual) = inv_cache.get(prev_round).unwrap();
                    let cycle_size = cur_round - prev_round;
                    let cycle_height_gain = mh - prev_actual;
                    let num_cycles = (rounds - 1 - cur_round) / cycle_size;
                    let remaining = (rounds - 1 - cur_round) % cycle_size;
                    let (_rem_heights, rem_actual) =
                        inv_cache.get(&(prev_round + remaining)).unwrap();
                    let rem_height_gain = rem_actual - prev_actual;

                    let neg_height: i64 = *mh as i64
                        + cycle_height_gain as i64 * num_cycles as i64
                        + rem_height_gain as i64;
                    return -neg_height + 1;
                    // if debug {
                    //     println!("Predicted for {}: {} cycles of size {}, {} remaining", rounds, num_cycles, cycle_size, remaining);
                    //     println!("Cycle gain: {}, rem gain: {}", cycle_height_gain, rem_height_gain);
                    //     println!("Height: {}", neg_height);
                    // }
                }
                inv_cache.insert(cur_round, (cache_key.0.clone(), *mh));
                cache.insert(cache_key, cur_round);
                // if debug {
                //     println!("Round {}: {}", cur_round, mh);
                // }
                break;
            }
        }
    }

    return -max_heights.iter().min().unwrap() as i64 + 1;
}

fn default_tetrominoes() -> Vec<Vec<Direction>> {
    return vec![
        vec![
            Direction { dx: 0, dy: 0 },
            Direction { dx: 1, dy: 0 },
            Direction { dx: 2, dy: 0 },
            Direction { dx: 3, dy: 0 },
        ],
        vec![
            Direction { dx: 1, dy: 0 },
            Direction { dx: 0, dy: -1 },
            Direction { dx: 1, dy: -1 },
            Direction { dx: 2, dy: -1 },
            Direction { dx: 1, dy: -2 },
        ],
        vec![
            Direction { dx: 0, dy: 0 },
            Direction { dx: 1, dy: 0 },
            Direction { dx: 2, dy: 0 },
            Direction { dx: 2, dy: -1 },
            Direction { dx: 2, dy: -2 },
        ],
        vec![
            Direction { dx: 0, dy: 0 },
            Direction { dx: 0, dy: -1 },
            Direction { dx: 0, dy: -2 },
            Direction { dx: 0, dy: -3 },
        ],
        vec![
            Direction { dx: 0, dy: 0 },
            Direction { dx: 1, dy: 0 },
            Direction { dx: 0, dy: -1 },
            Direction { dx: 1, dy: -1 },
        ],
    ];
}

impl BaseDay for Day17 {
    fn parse(&mut self, input: &mut InputReader) {
        fn parse_line(line: &String) -> Vec<Direction> {
            return line
                .chars()
                .map(|c| match c {
                    '<' => Direction { dx: -1, dy: 0 },
                    '>' => Direction { dx: 1, dy: 0 },
                    c => {
                        panic!("Bad value: {}", c);
                    }
                })
                .collect::<Vec<Direction>>();
        }

        parse_lines(input, &mut |line: String| {
            self.vals.extend(parse_line(&line));
        });
    }

    fn pt1(&mut self) -> String {
        let max_height = drop_rocks(&default_tetrominoes(), &self.vals, 7, 2022);
        return max_height.to_string();
    }

    fn pt2(&mut self) -> String {
        let max_height = drop_rocks(&default_tetrominoes(), &self.vals, 7, 1000000000000);
        return max_height.to_string();
    }
}

fn main() {
    let mut day = Day17 { vals: Vec::new() };
    run_day(&mut day);
}
