use itertools::join;

fn modulus(a: i32, b_u: usize) -> usize {
    let b = b_u as i32;
    return (((a % b) + b) % b) as usize;
}

fn circular_insert(idx: i32, value: usize, buffer: &mut Vec<usize>) -> usize {
    let actual = modulus(idx - 1, buffer.len()) + 1;
    if actual == buffer.len() {
        buffer.push(value);
    } else {
        buffer.insert(actual, value);
    }
    return actual;
}

fn circular_remove(idx: i32, buffer: &mut Vec<usize>) -> (usize, usize) {
    let actual = modulus(idx, buffer.len());
    let value = buffer[actual];
    let nextv = buffer[modulus(idx + 1, buffer.len())];  // dumb belts-and-suspenders check
    let mut next_idx = modulus(idx + 1, buffer.len());
    if next_idx > actual {
        next_idx -= 1;
    }
    buffer.remove(actual);
    assert_eq!(nextv, buffer[next_idx]);
    return (value, next_idx);
}

fn calc_high_score(num_players: usize, last_score: usize, verbose: bool) -> (usize, usize) {
    let mut circle = vec![0];
    let mut next_number = 1;
    let mut cur_marble = 0;
    let mut scores = vec![0; num_players];
    let mut cur_player = 0;

    loop {
        if next_number % 23 == 0 {
            let (removed_value, next_marble) = circular_remove(cur_marble as i32 - 7, &mut circle);
            println!("NN: {}, Old cur: {}, new cur: {}, new size: {}", next_number, cur_marble, next_marble, circle.len());
            let score = next_number + removed_value;
            scores[cur_player] += score;
            cur_marble = next_marble;
            if verbose {
                println!("Player {} scores {} + {} = {}", cur_player + 1, next_number, removed_value, score);
            }
            if score == last_score {
                let widx = (0..scores.len()).max_by_key(|p| scores[*p]).unwrap();
                return (widx + 1, scores[widx]);
            } else if next_number > last_score {
                panic!("Something has gone terribly wrong as we hit {}+{} = {}", next_number, removed_value, score);
            }
        } else {
            cur_marble = circular_insert(cur_marble as i32 + 2, next_number, &mut circle);
        }

        if verbose {
            println!("[{}] {} : ({})", cur_player + 1, join(circle.iter(), " "), circle[cur_marble]);
        }

        next_number += 1;
        cur_player = (cur_player + 1) % scores.len();
    }
}

fn main() {
    let args : Vec<String> = std::env::args().collect();
    let num_players = args[1].parse().unwrap();
    let last_score = args[2].parse().unwrap();
    let verbose = args.len() > 3;
    let (winning_player, high_score) = calc_high_score(num_players, last_score, verbose);
    println!("Calculated high score = {}, winning player = {}", high_score, winning_player);
}
