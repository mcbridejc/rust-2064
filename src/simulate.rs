use super::gameplay::*;


use serde::{Serialize, Deserialize};


#[derive(Debug)]
pub struct SingleRunResult {
    pub moves: i32,
    pub score: i32,
    pub largest: i32
}

#[derive(Serialize, Deserialize)]
pub struct BulkRunResult {
    pub avg_moves: i32,
    pub avg_score: i32,
    pub score_cdf_x: Vec<f32>,
    pub score_cdf_y: Vec<f32>,
    pub largest_hist: Vec<i32>,
}

pub fn single(algo: fn(&Board) -> MoveDir) -> SingleRunResult {
    const MAX_INVALID: i32 = 20;
    let mut board = Board::init();
    let mut result = SingleRunResult{moves: 0, score: 0, largest: 0};
    let mut invalid_count = 0;
    while !board.stuck() && invalid_count < MAX_INVALID {
        let move_outcome = play(&board, algo(&board));
        // We could argue that algorithms shouldn't make invalid moves; but I 
        // can make invalid moves with the keyboard. To keep it easy, invalid
        // moves are just ignored, but only a few times so that we don't get 
        // stuck. 
        match move_outcome {
            Ok(new_board) => {
                board = new_board; 
                invalid_count = 0;
            },
            Err(_error) => invalid_count += 1
        }
        result.moves += 1;
    }
    result.score = board.score;
    result.largest = *board.values.iter().max().unwrap();
    result
}


pub fn bulk(algo: fn(&Board) -> MoveDir, n: i32) -> BulkRunResult {
    const CDF_POINTS: i32 = 100;

    let mut results: Vec<SingleRunResult> = Vec::new();
    for _ in 0..n {
        results.push(single(algo));
    }
    let max_score = results.iter().max_by(|a, b| { a.score.cmp(&b.score) }).unwrap().score;
    results.sort_by(|a, b| { a.score.cmp(&b.score)});

    let cdf_step = max_score as f32 / CDF_POINTS as f32;
    let mut x = cdf_step;
    let mut y = 0.0;
    let mut i = 0;
    let mut cdf_x_values: Vec<f32> = Vec::new();
    let mut cdf_y_values: Vec<f32> = Vec::new();
    while x <= max_score as f32{
        while (results[i].score as f32) < x {
            y += 1.0 / results.len() as f32;
            i += 1;
        }
        cdf_x_values.push(x);
        cdf_y_values.push(y);
        x += cdf_step;
    }

    let mut largest_hist: Vec<i32> = Vec::new();
    for r in results {
        // Expect: 2 -> 0, 4 -> 1, 8 -> 2, etc
        let bin = 30 - r.largest.leading_zeros();
        if (bin as usize) >= largest_hist.len() {
            largest_hist.resize_with((bin + 1) as usize, Default::default);
        }
        largest_hist[bin as usize] += 1;
    }



    BulkRunResult{
        avg_moves: 0,
        avg_score: 0,
        score_cdf_x: cdf_x_values,
        score_cdf_y: cdf_y_values,
        largest_hist
    }
}