use super::gameplay::*;

extern crate rand;
use rand::seq::SliceRandom;


pub fn random(board: &Board) -> MoveDir {
    // Choose any of the four moves at random
    let options: Vec<MoveDir> = vec![MoveDir::Up, MoveDir::Down, MoveDir::Left, MoveDir::Right];
    let valid_options: Vec<MoveDir> = options.into_iter()
        .filter(|&dir| board.is_valid_move(dir))
        .collect();
    
    let mut rng = rand::thread_rng();
    *valid_options.choose(&mut rng).unwrap()
}

pub fn random_3dir(board: &Board) -> MoveDir {
    // Pick one of three directions at random (arbitrarily, down is excluded)
    // If none of those three are valid moves, then make the fourth move
    let options: Vec<MoveDir> = vec![MoveDir::Up, MoveDir::Left, MoveDir::Right];
    let mut valid_options: Vec<MoveDir> = options.into_iter()
        .filter(|&dir| board.is_valid_move(dir))
        .collect();

    if valid_options.len() == 0 {
        // Move down as a last resort
        valid_options.push(MoveDir::Down);
    }
    let mut rng = rand::thread_rng();
    *valid_options.choose(&mut rng).unwrap()
}

pub fn max_free_space_3dir(board: &Board) -> MoveDir {
    // Choose from a set of three moves, choosing the direction which results
    // in the greatest number of empty squares on the next turn (i.e. the move 
    // which results in the greatest number of merged tiles)
    let options = vec![MoveDir::Up, MoveDir::Left, MoveDir::Right];

    let mut selected = None;
    let mut best_score = -1;
    fn score(board: &Board) -> i32 {
        let mut count = 0;
        for v in &board.values {
            if *v == 0 {
                count += 1;
            }
        }
        count
    }
    for dir in options.iter() {
        if let Ok(board) = play(&board, *dir) {
            let score = score(&board);
            if score > best_score {
                best_score = score;
                selected = Some(*dir);
            }
        }
    }
    
    match selected {
        Some(dir) => dir,
        None => MoveDir::Down
    }
}

pub fn max_free_space(board: &Board) -> MoveDir {
    // Choose the direction which results in the greatest number of empty 
    // squares on the next turn (i.e. the move which results in the greatest
    // number of merged tiles)
    let options = vec![MoveDir::Up, MoveDir::Down, MoveDir::Left, MoveDir::Right];

    let mut selected = MoveDir::Up;
    let mut best_score = -1;
    fn score(board: &Board) -> i32 {
        let mut count = 0;
        for v in &board.values {
            if *v == 0 {
                count += 1;
            }
        }
        count
    }

    for dir in options.iter() {
        if let Ok(board) = play(&board, *dir) {
            let score = score(&board);
            if score > best_score {
                best_score = score;
                selected = *dir;
            }
        }
    }
    selected
}

