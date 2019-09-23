use super::gameplay::*;

extern crate rand;
use rand::seq::SliceRandom;


pub fn random(board: &Board) -> MoveDir {
    let options = [MoveDir::Up, MoveDir::Down, MoveDir::Left, MoveDir::Right];
    
    let mut rng = rand::thread_rng();
    *options.choose(&mut rng).unwrap()
}

pub fn random_3dir(board: &Board) -> MoveDir {
    // TODO: We need to check if none of the three options are valid, in which case we should
    // fall back to the fourth direction
    let options = [MoveDir::Up, MoveDir::Left, MoveDir::Right];
    
    let mut rng = rand::thread_rng();
    *options.choose(&mut rng).unwrap()
}