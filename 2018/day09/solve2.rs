// Prev solution was too slow, I'm assuming because of all the copies on inserting in the middle
// of the vec, so let's try a double linked list instead

#[derive(Debug)]
struct Marble {
    value: i32,
    cw: usize,
    ccw: usize,
}

// Insert to cw of the current node, then make the newly-inserted one current
fn circular_insert(current: usize, value: i32, marbles: &mut Vec<Marble>) -> usize {
    marbles.push(Marble {value: value, cw: marbles[current].cw, ccw: current});
    let cw = marbles[current].cw;
    marbles[current].cw = marbles.len() - 1;
    marbles[cw].ccw = marbles.len() - 1;
    return marbles.len() - 1;
}

// Remove current node, then make the cw node of the removed one current
fn circular_remove(current: usize, marbles: &mut Vec<Marble>) -> (i32, usize) {
    // If we cared about saving space we could swap the last marble into the current spot
    // and shrink the vec
    let value = marbles[current].value;
    let cw = marbles[current].cw;
    let ccw = marbles[current].ccw;
    marbles[cw].ccw = ccw;
    marbles[ccw].cw = cw;
    return (value, cw);
}

fn move_n(current: usize, n: i32, marbles: &mut Vec<Marble>) -> usize {
    let mut new_current = current;
    if n >= 0 {
        for _ in 0..n {
            new_current = marbles[new_current].cw;
        }
    } else {
        for _ in 0..-n {
            new_current = marbles[new_current].ccw;
        }
    }
    return new_current;
}

fn marble_str(current: usize, marbles: &Vec<Marble>) -> String {
    let mut idx = 0;
    let mut ret = "".to_string();
    loop {
        let first = ret.len() == 0;
        if !first {
            if idx == 0 {
                return ret;
            }
            ret.push(' ');
        }
        if idx == current {
            ret.push('(');
        }
        ret.push_str(&marbles[idx].value.to_string());
        if idx == current {
            ret.push(')');
        }
        idx = marbles[idx].cw;
    }
}

fn calc_high_score(num_players: usize, last_marble: i32, verbose: bool) -> (usize, i64) {
    let mut circle = vec![Marble {value: 0, cw: 0, ccw: 0}];
    circle.reserve(last_marble as usize);
    let mut next_number = 1;
    let mut cur_marble = 0;
    let mut scores : Vec<i64> = vec![0; num_players];
    let mut cur_player = 0;

    loop {
        if next_number % 23 == 0 {
            cur_marble = move_n(cur_marble, -7, &mut circle);
            let (removed_value, next_marble) = circular_remove(cur_marble, &mut circle);
            let score = next_number + removed_value;
            scores[cur_player] += score as i64;
            cur_marble = next_marble;
            if verbose {
                println!("Player {} scores {} + {} = {}", cur_player + 1, next_number, removed_value, score);
            }
        } else {
            cur_marble = move_n(cur_marble, 1, &mut circle);
            cur_marble = circular_insert(cur_marble, next_number, &mut circle);
        }

        if verbose {
            println!("[{}] {}", cur_player + 1, marble_str(cur_marble, &circle));
        }

        if next_number == last_marble {
            let widx = (0..scores.len()).max_by_key(|p| scores[*p]).unwrap();
            return (widx + 1, scores[widx]);
        }

        next_number += 1;
        cur_player = (cur_player + 1) % scores.len();
    }
}

fn main() {
    let args : Vec<String> = std::env::args().collect();
    let num_players = args[1].parse().unwrap();
    let last_marble = args[2].parse().unwrap();
    let verbose = args.len() > 3;
    let (winning_player, high_score) = calc_high_score(num_players, last_marble, verbose);
    println!("Calculated high score = {}, winning player = {}", high_score, winning_player);
}
