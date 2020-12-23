#[macro_use] extern crate failure;
#[macro_use] extern crate lazy_static;

use failure::{Error, bail};
use itertools::Itertools;
use std::collections::{HashSet, VecDeque};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::io::{BufReader, BufRead};
use std::fs::File;

#[derive(Debug, Clone)]
struct Deck {
    id: String,
    cards: VecDeque<i32>,
}

fn read_input(file: &str) -> Result<Vec<Deck>, Error> {
    let f = File::open(file)?;
    let br = BufReader::new(f);

    let mut decks = Vec::new();

    for line in br.lines() {
        let line = line?;
        if line.len() == 0 {
            continue;
        }
        if line.starts_with("Player") {
            let mut deck = Deck { id: line, cards: VecDeque::new() };
            deck.id.pop();
            decks.push(deck);

        } else {
            let sz = decks.len() - 1;
            decks[sz].cards.push_back(line.parse()?);
        }
    }
    return Ok(decks);
}

fn play_game(orig_decks: &Vec<Deck>, verbose: bool) -> Deck {
    let mut decks : Vec<VecDeque<i32>> = orig_decks.iter().map(|d| d.cards.clone()).collect();
    let mut round = 1;
    loop {
        let winner = play_round(&mut decks);
        if verbose {
            println!("Round {}:", round);
            for deck in &decks {
                println!("{:?}", deck);
            }
            println!();
        }
        if winner.is_some() {
            let deck_id = orig_decks[winner.unwrap()].id.to_string();
            return Deck { id: deck_id, cards: decks.remove(winner.unwrap()) };
        }
        round += 1;
    }
}

fn play_round(decks: &mut Vec<VecDeque<i32>>) -> Option<usize> {
    let mut winner_idx : Option<usize> = None;
    let mut other_nonempty = false;
    for (val, idx) in (0..decks.len()).map(|idx| (decks[idx].pop_front().unwrap(), idx)).sorted().rev() {
        if winner_idx.is_none() {
            winner_idx = Some(idx);
        }
        decks[winner_idx.unwrap()].push_back(val);
        if idx != winner_idx.unwrap() && decks[idx].len() > 0 {
            other_nonempty = true;
        }
    }

    if other_nonempty {
        winner_idx = None;
    }
    return winner_idx;
}

fn play_recur_game(orig_decks: &Vec<Deck>, verbose: bool) -> Deck {
    // the recursive depth should be capped at the number of cards in one deck
    let mut all_decks : Vec<Vec<VecDeque<i32>>> =
        (0..orig_decks[0].cards.len())
        .map(|_| (0..orig_decks.len()).map(|_| VecDeque::new()).collect())
        .collect();
    all_decks[0] = orig_decks.iter().map(|d| d.cards.clone()).collect();
    let winner = play_recur_game_inner(&mut all_decks, 0, verbose);
    let deck_id = orig_decks[winner].id.to_string();
    return Deck { id: deck_id, cards: all_decks[0].remove(winner) };
}

fn play_recur_game_inner(all_decks: &mut Vec<Vec<VecDeque<i32>>>, game: usize, verbose: bool) -> usize {
    let mut seen = HashSet::new();
    let mut round = 1;
    loop {
        let mut hasher = DefaultHasher::new();
        all_decks[game].hash(&mut hasher);
        if !seen.insert(hasher.finish()) {
            if verbose {
                println!("Game {} won by player 1 due to repeated position", game);
            }
            return 0;
        }

        if verbose {
            println!("Round {}, Game {}:", round, game);
            for (idx, deck) in all_decks[game].iter().enumerate() {
                println!("Player {}: {:?}", idx+1, deck);
            }
            println!();
        }

        let winner = play_recur_round(all_decks, game, verbose);
        if winner.is_some() {
            if verbose {
                println!("Game {} won by player {} due to empty deck", game, winner.unwrap() + 1);
            }
            return winner.unwrap();
        }
        round += 1;
    }
}

fn play_recur_round(all_decks: &mut Vec<Vec<VecDeque<i32>>>, game: usize, verbose: bool) -> Option<usize> {
    let cur : Vec<(i32, usize)> =
        (0..all_decks[game].len())
        .map(|idx| (all_decks[game][idx].pop_front().unwrap(), idx))
        .collect();

    let round_winner = if cur.iter().all(|(val, idx)| all_decks[game][*idx].len() as i32 >= *val) {
        if verbose {
            println!("Recursing to new game");
        }
        let next_game = game + 1;
        for (val, idx) in cur.iter() {
            all_decks[next_game][*idx].clear();
            for cn in 0..(*val as usize) {
                let cnth = all_decks[game][*idx][cn];
                all_decks[next_game][*idx].push_back(cnth);
            }
        }
        play_recur_game_inner(all_decks, next_game, verbose)
    } else {
        if verbose {
            println!("Can't recurse, using highest val");
        }
        cur.iter().sorted().rev().nth(0).unwrap().1
    };

    let vals = cur.iter().sorted_by_key(|(_, idx)| if *idx == round_winner { (0, idx) } else { (1, idx ) }).map(|(val, _)| val);
    for val in vals {
        all_decks[game][round_winner].push_back(*val);
    }

    if (0..all_decks[game].len()).filter(|idx| all_decks[game][*idx].len() != 0).count() == 1 {
        return Some(round_winner);
    } else {
        return None;
    }
}

fn score_deck(deck: &Deck) -> i32 {
    return deck.cards.iter().rev().enumerate().map(|(idx, val)| (idx+1) as i32 * val).sum();
}

fn main() {
    let args : Vec<String> = std::env::args().collect();
    if args[1] == "1" {
        println!("Doing part 1");
        let decks = read_input(&args[2]).unwrap();
        let verbose = args.len() > 3 && args[3].chars().nth(0).unwrap() == 't';
        let winner = play_game(&decks, verbose);
        println!("Score for {} is {}", winner.id, score_deck(&winner));
    } else {
        println!("Doing part 2");
        let decks = read_input(&args[2]).unwrap();
        let verbose = args.len() > 3 && args[3].chars().nth(0).unwrap() == 't';
        let winner = play_recur_game(&decks, verbose);
        println!("Score for {} is {}", winner.id, score_deck(&winner));
    }
}
