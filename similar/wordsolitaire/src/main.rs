use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

const WORD_SIZE: usize = 5;

// https://kloonigames.com/wordsolitaire/
struct Trie {
    is_terminal: bool,
    children: HashMap<char, Trie>,
}

fn load_dict(path: &str) -> Trie {
    fn load_one(trie: &mut Trie, word: &str) {
        let mut cur = trie;
        for ch in word.chars() {
            cur = cur.children.entry(ch).or_insert(Trie { is_terminal: false, children: HashMap::new() });
        }
        cur.is_terminal = true;
    }

    let mut root = Trie { is_terminal: false, children: HashMap::new() };
    for line in BufReader::new(File::open(path).unwrap()).lines() {
        let line = line.unwrap();
        let line = line.trim();
        if line.len() != WORD_SIZE {
            continue;
        }
        load_one(&mut root, line);
    }
    return root;
}

fn load_piles(path: &str) -> Vec<Vec<char>> {
    // note this assumes the piles are in the file, one pile per line, no spaces between them,
    // and the lines are written such that the topmost card is on the left and the bottommost
    // on the right
    let mut piles = Vec::new();
    for line in BufReader::new(File::open(path).unwrap()).lines() {
        let line = line.unwrap();
        let line = line.trim();
        piles.push(line.chars().flat_map(|c| c.to_lowercase()).collect());
    }
    return piles;
}

struct State<'a> {
    offsets: Vec<usize>,
    word: Vec<(char, usize)>,
    cur_trie: &'a Trie,
}

fn solve(piles: &Vec<Vec<char>>, trie: &Trie) {
    fn recur<'a>(state: &mut State<'a>, solutions: &mut Vec<Vec<String>>, piles: &Vec<Vec<char>>, root: &'a Trie) {
        if (0..piles.len()).all(|idx| piles[idx].len() == state.offsets[idx]) {
            fn format_word(word: &[(char, usize)]) -> String {
                let alpha = word.iter().map(|c| c.0).collect::<String>();
                let idxs = word.iter().map(|c| c.1.to_string()).collect::<Vec<String>>().join(",");
                return format!("{} [{}]", alpha, idxs);
            }

            solutions.push(state.word.chunks(WORD_SIZE).map(|chunk| format_word(&chunk)).collect());
            panic!("Solutions: {:?}", solutions);
            return;
        }

        for idx in 0..piles.len() {
            let offset = state.offsets[idx];
            if offset >= piles[idx].len() {
                continue;
            }
            let ch = piles[idx][offset];
            if let Some(next_trie) = state.cur_trie.children.get(&ch) {
                state.offsets[idx] = offset + 1;
                state.word.push((ch, idx));
                let old_trie = state.cur_trie;
                if state.word.len() % WORD_SIZE == 0 {
                    if !next_trie.is_terminal {
                        panic!("Reached the end of a word, but it's not terminal?");
                    }
                    // start a new word from the root:
                    state.cur_trie = root;
                } else {
                    state.cur_trie = next_trie;
                }

                recur(state, solutions, piles, root);

                state.cur_trie = old_trie;
                state.word.pop();
                state.offsets[idx] = offset;
            }
        }
    }

    let mut state = State { offsets: vec![0; piles.len()], word: vec![], cur_trie: trie };
    let mut solutions = Vec::new();
    recur(&mut state, &mut solutions, piles, trie);
    println!("Found solutions:");
    for solution in &solutions {
        println!("{}", solution.join(", "));
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let trie = load_dict("/usr/share/dict/words");
    let piles = load_piles(&args[1]);
    solve(&piles, &trie);
}
