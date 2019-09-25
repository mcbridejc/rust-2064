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


#[derive(Clone)]
struct EvaluationNode {
    dir: Option<MoveDir>,
    board: Board,
    rank: i32,
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
    let fs_score = score_free_space(&board);

    // We want a function that rewards having more bigger blocks on one edge of 
    // the board, and also having htem sorted by size along that edge, basically.
    // It should be rotation invariant; we don't care which edge we are stacking
    // on. 
    let mut scores: Vec<i32> = Vec::new();
    for dir in &[MoveDir::Up, MoveDir::Down, MoveDir::Left, MoveDir::Right] {
        for i in 0..4 {
            let row = board.directional_row(i, *dir);
            let mut s = 0;
            for j in 0..3 {
                if row[j] == 0 {
                    continue;
                }
                if row[j+1] >= row[j] {
                    s += 1;
                } else {
                    s -= 1;
                }
            }
            scores.push(s);
        }
    }
    return fs_score + scores.iter().max().unwrap();
}

fn expand_scenarios(input_set: &Vec<EvaluationNode>, score_fn: fn(&Board) -> i32)  -> Vec<EvaluationNode> {

    let mut out: Vec<EvaluationNode> = Vec::new();

    let options = vec![MoveDir::Up, MoveDir::Down, MoveDir::Left, MoveDir::Right];

    for start in input_set {
        for dir in &options {
            if let Ok(new_board) = play(&start.board, *dir) {
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

pub fn naive_lookahead(board: &Board, moves: i32, score_fn: ScoreFunction) -> MoveDir {
    // "Naive" because it would be better, probably, to do a full minimax with all
    // of the possible random new tiles at each turn. 

    let score_fn = match score_fn {
        ScoreFunction::FreeSpace => score_free_space,
        ScoreFunction::FreeSpaceWithSortedness => score_free_space_sortedness,
    };

    let mut nodes = vec![EvaluationNode{dir: None, board: board.clone(), rank: 0}];
    for _ in 0..moves {
        let new_nodes = expand_scenarios(&nodes, score_fn);
        if new_nodes.len() == 0 {
            break;
        }
        nodes = new_nodes;
    }

    let best_node = nodes.iter().max_by_key(|x| x.rank);
    best_node.unwrap().dir.unwrap()
}