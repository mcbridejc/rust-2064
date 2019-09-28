use super::gameplay::*;

extern crate flame;
extern crate rand;
use rand::seq::SliceRandom;
use std::cmp::max;



pub fn random(_player: &mut GamePlayer, board: &Board) -> MoveDir {
    // Choose any of the four moves at random
    let options: Vec<MoveDir> = vec![MoveDir::Up, MoveDir::Down, MoveDir::Left, MoveDir::Right];
    let valid_options: Vec<MoveDir> = options.into_iter()
        .filter(|&dir| board.is_valid_move(dir))
        .collect();
    
    let mut rng = rand::thread_rng();
    *valid_options.choose(&mut rng).unwrap()
}

pub fn random_3dir(_player: &mut GamePlayer, board: &Board) -> MoveDir {
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

pub fn max_free_space_3dir(player: &mut GamePlayer, board: &Board) -> MoveDir {
    // Choose from a set of three moves, choosing the direction which results
    // in the greatest number of empty squares on the next turn (i.e. the move 
    // which results in the greatest number of merged tiles)
    let options = vec![MoveDir::Up, MoveDir::Left, MoveDir::Right];

    let mut selected = None;
    let mut best_score = -1;

    for dir in options.iter() {
        if let Ok(b) = player.play(&board, *dir) {
            let score = score_free_space(&b);
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

pub fn max_free_space(player: &mut GamePlayer, board: &Board) -> MoveDir {
    // Choose the direction which results in the greatest number of empty 
    // squares on the next turn (i.e. the move which results in the greatest
    // number of merged tiles)
    let options = vec![MoveDir::Up, MoveDir::Down, MoveDir::Left, MoveDir::Right];

    let mut selected = MoveDir::Up;
    let mut best_score = -1;

    for dir in options.iter() {
        if let Ok(new_board) = player.play(&board, *dir) {
            let score = score_free_space(&new_board);
            if score > best_score {
                best_score = score;
                selected = *dir;
            }
        }
    }
    selected
}


#[derive(Clone)]
struct EvaluationNode {
    dir: Option<MoveDir>,
    board: Board,
    rank: i32,
}

fn expand_scenarios(player: &mut GamePlayer, input_set: &Vec<EvaluationNode>, score_fn: fn(&Board) -> i32)  -> Vec<EvaluationNode> {
    //let _guard = flame::start_guard("expand_scenarios");
    let mut out: Vec<EvaluationNode> = Vec::new();

    let options = vec![MoveDir::Up, MoveDir::Down, MoveDir::Left, MoveDir::Right];

    for start in input_set {
        for dir in &options {
            if let Ok(new_board) = player.play(&start.board, *dir) {
                let rank = score_fn(&new_board);
                out.push(EvaluationNode{ dir: Some(start.dir.unwrap_or(*dir)), board: new_board, rank});
            }
        }
    }
    out
}

pub enum ScoreFunction {
    FreeSpace,
    FreeSpaceWithSortedness
}

pub fn naive_lookahead(mut player: &mut GamePlayer, board: &Board, moves: i32, score_fn: ScoreFunction) -> MoveDir {
    // "Naive" because it would be better, probably, to do a full minimax with all
    // of the possible random new tiles at each turn. 

    let score_fn = match score_fn {
        ScoreFunction::FreeSpace => score_free_space,
        ScoreFunction::FreeSpaceWithSortedness => score_free_space_sortedness,
    };

    let mut nodes = vec![EvaluationNode{dir: None, board: board.clone(), rank: 0}];
    for _ in 0..moves {
        let new_nodes = expand_scenarios(&mut player, &nodes, score_fn);
        if new_nodes.len() == 0 {
            break;
        }
        nodes = new_nodes;
    }

    if nodes.len() > 0 {
        let best_node = nodes.iter().max_by_key(|x| x.rank);
        best_node.unwrap().dir.unwrap()
    } else {
        MoveDir::Down // TODO: We should probably return a Result with error if there is no available move
    }
}


fn score_free_space(board: &Board) -> i32 {
    let mut count = 0;
    for v in &board.values {
        if *v == 0 {
            count += 1;
        }
    }
    count
}

fn score_free_space_sortedness(board: &Board) -> i32 {
    //let _guard = flame::start_guard("score_fss");

    // We want a function that rewards having more bigger blocks on one edge of 
    // the board, and also having htem sorted by size along that edge, basically.
    let mut empty_count = 0;
    let mut row_score = 0;
    let mut col_score = 0;
    let mut row_scoren = 0;
    let mut col_scoren = 0;
    for i in 0..4 {
        for j in 0..4 {
            if board.values[i + j * 4] == 0 {
                empty_count += 1;
            }
        }
        for j in 0..3 {
            if board.values[i + j*4] >= board.values[i + (j+1)*4] {
                row_score += 1;
            } 
            if board.values[i + j*4] <= board.values[i + (j+1)*4] {
                row_scoren += 1;
            }

            if board.values[i*4 + j] >= board.values[i*4 + (j+1)] {
                col_score += 1;
            } 
            if board.values[i*4 + j] <= board.values[i*4 + (j+1)] {
                col_scoren += 1;
            }
        }
    }

    let sorted_score = max(row_score, row_scoren) +  max(col_score, col_scoren);
    return empty_count*20 + sorted_score*20 + board.score;
}
