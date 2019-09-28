

extern crate flame;

extern crate rand;
use rand::{Rng, SeedableRng};
use rand::rngs::SmallRng;

use std::ops::{Index, IndexMut};

#[derive(Copy, Clone)]
pub enum MoveDir {
    Up,
    Down,
    Left,
    Right
}

// Dropping (probably?) in favor of the directional view
// pub struct BoardIterator {
//     curr: i32,
//     step: i32,
//     last: i32,
//     board: &Board,
// }

// pub impl Iterator for BoardIterator {
//     type Item = i32;

//     fun next(&mut self) -> Option<&i32> {
//         if self.curr == self.last {
//             return None;
//         }
//         self.curr += step;
//         return board.values[self.curr as usize]
//     }
// }

pub struct DirectionalView<'a> {
    dir: MoveDir,
    values: &'a mut [i32; 16],
}

pub struct LineView<'a> {
    dir: MoveDir,
    values: &'a mut [i32; 16],
    row: usize,
}

impl Index<usize> for LineView<'_> {
    type Output = i32;

    fn index<'c>(&'c self, index: usize) -> & Self::Output {
        self.cell(self.row, index)
    }
}

impl IndexMut<usize> for LineView<'_> {
    fn index_mut<'c>(&'c mut self, index: usize) -> & mut Self::Output {
        self.cell_mut(self.row, index)
    }
}

impl LineView<'_> {
    // Return a specific element using coordinates in the "move dir" frame
    pub fn cell(&self, row: usize, col: usize) -> &i32 {
        match self.dir {
            MoveDir::Right => &self.values[row * 4 + col],
            MoveDir::Left => &self.values[row * 4 + 3 - col],
            MoveDir::Down => &self.values[row + col * 4],
            MoveDir::Up => &self.values[row + (3 - col) * 4],
        }
    }
    
    // Return a specific element using coordinates in the "move dir" frame
    pub fn cell_mut(&mut self, row: usize, col: usize) -> &mut i32 {
        match self.dir {
            MoveDir::Right => &mut self.values[row * 4 + col],
            MoveDir::Left => &mut self.values[row * 4 + 3 - col],
            MoveDir::Down => &mut self.values[row + col * 4],
            MoveDir::Up => &mut self.values[row + (3 - col) * 4],
        }
    }
}

impl DirectionalView<'_> {
    // Return a specific element using coordinates in the "move dir" frame
    pub fn cell(&mut self, row: usize, col: usize) -> &mut i32 {
        match self.dir {
            MoveDir::Right => &mut self.values[row * 4 + col],
            MoveDir::Left => &mut self.values[row * 4 + 3 - col],
            MoveDir::Down => &mut self.values[row + col * 4],
            MoveDir::Up => &mut self.values[row + (3 - col) * 4],
        }
    }

    pub fn line_view(& mut self, row: usize) -> LineView<> {
        LineView{row, dir: self.dir, values: self.values}
    }
}

#[derive(Clone)]
pub struct Board {
    pub values: [i32; 16],
    pub score: i32,
}

impl Default for Board {
    fn default () -> Board {
        // Create small, cheap to initialize and fast RNG with a random seed.
        // The randomness is supplied by the operating system.
        Board{values: [0; 16], score: 0}
    }
}

impl Board {

    pub fn row(&self, n: usize, reverse: bool) -> [i32; 4] {
        let mut r: [i32; 4] = [0, 0, 0, 0];
        for i in 0..4 {
            let src = if reverse {
                n*4 + 3 - i
            } else {
                n*4 + i
            };
            r[i as usize] = self.values[src as usize];
        }
        r
    }

    pub fn col(&self, n: usize, reverse: bool) -> [i32; 4] {
        let mut r: [i32; 4] = [0, 0, 0, 0];

        for i in 0..4 {
            let src = if reverse {
                n + (3 - i)*4
            } else {
                n + i*4
            };
            r[i as usize] = self.values[src as usize];
        }
        r
    }

    pub fn set_row(&mut self, n: usize, value: [i32; 4], reverse: bool) {
        for i in 0..4 {
            let dst = if reverse {
                n*4 + 3 - i
            } else {
                n*4 + i
            };
            self.values[dst as usize] = value[i as usize];
        }
    }

    pub fn set_col(&mut self, n: usize, value: [i32; 4], reverse: bool) {
        for i in 0..4 {
            let dst = if reverse {
                n + (3 - i) * 4
            } else {
                n + i * 4
            };
            self.values[dst as usize] = value[i as usize];
        }
    }
    
    pub fn blank() -> Board {
        Board::default()
    }

    pub fn init() -> Board {
        // TODO: Generate starting cell randomly
        Board{ values: [2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], ..Board::default()}
    }

    // Faster way to test if a move is valid than to fully execute the move
    pub fn is_valid_move(&self, dir: MoveDir) -> bool {
        // We just need to find one non-zero cell that will merge into a cell,
        // i.e. has a next cell that is 0 or of the same value
        for i in 0..4usize {
            let row = match dir {
                MoveDir::Up => self.col(i, true),
                MoveDir::Down => self.col(i, false),
                MoveDir::Left => self.row(i, true),
                MoveDir::Right => self.row(i, false),
            };
            for j in 0..3 {
                if row[j as usize] != 0 && row[(j + 1) as usize] == 0 || row[j as usize] == row[(j+1) as usize] {
                    return true;
                }
            }
        }
        false
    }

    pub fn stuck(&self) -> bool {
        // Might think about how to do this faster or whatever, but for now 
        // we just try all four moves, but also use the simple rule that if 
        // there are any zeros, we cannot be stuck as I think it will save time. 
        for x in self.values.iter() {
            if *x == 0 {
                return false;
            }
        }
        for dir in &[MoveDir::Up, MoveDir::Down, MoveDir::Left, MoveDir::Right] {
            if self.is_valid_move(*dir) {
                return false;
            }
        }
        true
    }

    // We frequently encounter the problem of wanting to look at the board
    // from a particular frame of reference, e.g. when making a computing a 
    // new board based on up,down,left,right move, or when evaluating the fitness
    // of a board. This will return a copy of a row (for Left, Right), or a 
    // column (up, down) ordered according to direction given. 
    // For example, for directional_iter(0, MoveDir::Up), you will be 
    // returned column 0 of the board, reversed so that row 3 is in the 
    // first location of the returned array. 
    // This returns a copy of the row as an array instead of an iterator
    // because I'm assuming copying the 4 words will be faster, but I haven't
    // tried it.
    pub fn directional_row(&self, n: usize, dir: MoveDir) -> [i32; 4] {
        let (reverse, row) = match dir {
            MoveDir::Up => (true, false),
            MoveDir::Down => (false, false),
            MoveDir::Left => (true, true),
            MoveDir::Right => (false, true)
        };

        if row {
            return self.row(n, reverse);
        } else {
            return self.col(n, reverse);
        }
    }

    // pub fn directional_iter(&self, n: usize, dir: MoveDir) -> BoardIterator {
    //     let (reverse, row) = match dir {
    //         MoveDir::Up => (true, false),
    //         MoveDir::Down => (false, false),
    //         MoveDir::Left => (true, true),
    //         MoveDir::Right => (false, true)
    //     };
    //     if row {
    //         if reverse {
    //             BoardIterator{curr: n * 4 + 4, step: -1, last: n * 4, board: self}
    //         } else {
    //             BoardIterator{curr: n * 4 - 1, step: -1, last: n * 4 + 3, board: self}
    //         }
    //     } else {
    //         if reverse {
    //             BoardIterator{curr: 12 + n + 4, step: -4, last: n, board: self}
    //         } else {
    //             BoardIterator{curr: n - 4, step: 4, last: 12 + n, board: self}
    //         }
    //     }
    // }

    pub fn directional_view(&mut self, dir: MoveDir) -> DirectionalView {
        DirectionalView{dir, values: &mut self.values}
    }
}

// Encapsulate an RNG, because I dont want to initialize a new one every time we play
pub struct GamePlayer {
    pub rng: SmallRng,
}

impl Default for GamePlayer {
    fn default () -> GamePlayer {
        // Create small, cheap to initialize and fast RNG with a random seed.
        // The randomness is supplied by the operating system.
        GamePlayer{rng: SmallRng::from_entropy()}
    }
}

impl GamePlayer {
    pub fn play(&mut self, b: &Board, dir: MoveDir) -> Result<Board, String> {
        let mut new_board = Board{..*b};
        return match self.play_inplace(&mut new_board, dir) {
            Ok(_) => Ok(new_board),
            Err(message) => Err(message),
        } 
    }

    pub fn play_inplace(&mut self, b: &mut Board, dir: MoveDir) -> Result<bool, String> {
        play_inplace(b, dir, &mut self.rng)
    }
}


pub fn play_inplace(b: &mut Board, dir: MoveDir, rng: &mut SmallRng) -> Result<bool, String> {
    // //let _guard = flame::start_guard("play");
    // let mut new = Board{ values: b.values, score: b.score };

    //// Method #1
    // let (reverse, row) = match dir {
    //     MoveDir::Up => (true, false),
    //     MoveDir::Down => (false, false),
    //     MoveDir::Left => (true, true),
    //     MoveDir::Right => (false, true)
    // };
    // if row {
    //     for i in 0..4 {
    //         let (new_row, delta_score) = reduce_row(new.row(i, reverse));
    //         new.set_row(i, new_row, reverse);
    //         new.score += delta_score;
    //     }
    // } else {
    //     for i in 0..4 {
    //         let (new_row, delta_score) = reduce_row(new.col(i, reverse));
    //         new.set_col(i, new_row, reverse);
     
    //         new.score += delta_score;
    //     }
    // }

    // //// Method #2
    // for i in 0..4 {
    //     let add_score = reduce_row_inplace(&mut new.directional_view(dir).line_view(i));
    //     new.score += add_score;
    // }

    //// Method #3
    let mut changed = false;
    for row in 0..4 {
        let (start, step) = match dir {
            MoveDir::Right => (row * 4i32, 1i32),
            MoveDir::Left => (row * 4i32 + 3, -1i32),
            MoveDir::Down => (row, 4),
            MoveDir::Up => (12 + row, -4),
        };

        for slot in (1..4_i32).rev() {
            let cur_pos = slot;
            let mut next_pos = slot-1;
            while next_pos >= 0 {
                let cur_idx = (start + cur_pos * step) as usize;
                let next_idx = (start + next_pos * step) as usize;
                //let mut next = &mut new.values[(start + next_idx * step) as usize];
                if b.values[next_idx] == 0 {
                    next_pos -= 1;
                    continue;
                } else if b.values[next_idx] == b.values[cur_idx] {
                    b.score += b.values[cur_idx] * 2;
                    b.values[cur_idx] *= 2;
                    b.values[next_idx] = 0;
                    changed = true;
                    break;
                } else if b.values[cur_idx] == 0 {
                    b.values[cur_idx] = b.values[next_idx];
                    b.values[next_idx] = 0;
                    changed = true;
                    next_pos = cur_pos - 1; // Restart search with new current position value
                    continue;
                } else {
                    break; // The next row is not combinable
                }
            }
        }
        
    }
    
    if !changed {
        return Err(String::from("Invalid move"))
    }

    // // Collect list of 0's
    // let mut zeros: Vec<i32> = Vec::new();
    // for i in 0..16 {
    //     if new.values[i] == 0 {
    //         zeros.push(i as i32);
    //     }
    // }
    // // Pick a random 0 cell
    // let set_idx = zeros.choose(&mut rng).unwrap();

    let mut zero_count = 0;
    for i in 0..16 {
        if b.values[i] == 0 {
            zero_count += 1;
        }
    }
    zero_count = rng.gen_range(0, zero_count) + 1; 
    let mut set_idx = 0;
    for i in 0..16 {
        if b.values[i] == 0 {
            zero_count -= 1;
        }

        if zero_count == 0 {
            set_idx = i;
            break;
        }
    }

    // 1 in 10 chance of being a 4, otherwise its a 2
    let rval = rng.gen_range(0, 10);
    if rval < 9 {
        b.values[set_idx as usize] = 2;
    } else {
        b.values[set_idx as usize] = 4;
    }
    
    Ok(true)
}

// Defines row/column reduction rules. It assumes movement is "right", i.e. from 
// index 0 towards index 3.
pub fn reduce_row_inplace(row: &mut LineView) -> i32 {
    // First, shift right as needed until there are no empty (value = 0) 
    // cells to the right of non-empty ones
    let mut i = 1;
    let mut add_score = 0;
    while i <= 3 {
        if row[i] != 0 {
            i += 1; 
            continue;
        }
        for j in (1..=i).rev() {
            row[j] = row[j-1];
        }
        row[0] = 0;
        i += 1;
    }

    // Now combine matching neighbors, starting from the right
    i = 3;
    while i > 0 {
        if row[i] == row[i-1] {
            row[i] *= 2;
            add_score += row[i];
            for j in (1..i).rev() {
                row[j] = row[j-1];
            }
            row[0] = 0;
        }
        i -= 1;
    }
    add_score
}

// Defines row/column reduction rules. It assumes movement is "right", i.e. from 
// index 0 towards index 3.
pub fn reduce_row(row: [i32; 4]) -> ([i32; 4], i32) {
    let mut row = row;

    // First, shift right as needed until there are no empty (value = 0) 
    // cells to the right of non-empty ones
    let mut i = 1;
    let mut add_score = 0;
    while i <= 3 {
        if row[i] != 0 {
            i += 1; 
            continue;
        }
        for j in (1..=i).rev() {
            row[j] = row[j-1];
        }
        row[0] = 0;
        i += 1;
    }

    // Now combine matching neighbors, starting from the right
    i = 3;
    while i > 0 {
        if row[i] == row[i-1] {
            row[i] *= 2;
            add_score += row[i];
            for j in (1..i).rev() {
                row[j] = row[j-1];
            }
            row[0] = 0;
        }
        i -= 1;
    }
    (row, add_score)
}

#[cfg(test)]
mod gameplay_tests {
    use super::*;
    #[test]
    fn board_row_access() {
        let b = Board{ values: [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15], ..Board::default() };
        assert_eq!(b.row(0, false), [0, 1, 2, 3]);
        assert_eq!(b.row(2, false), [8, 9, 10, 11]);
        // Reversed
        assert_eq!(b.row(0, true), [3, 2, 1, 0]);
        assert_eq!(b.row(2, true), [11, 10, 9, 8]);
    }

    #[test]
    fn board_col_access() {
        let b = Board{ values: [0, 1, 2, 3, 
                                4, 5, 6, 7,
                                8, 9, 10, 11,
                                12, 13, 14, 15],
                        ..Board::default() };
        assert_eq!(b.col(2, false), [2, 6, 10, 14]);
        assert_eq!(b.col(3, false), [3, 7, 11, 15]);
        // reversed
        assert_eq!(b.col(2, true), [14, 10, 6, 2]);
        assert_eq!(b.col(3, true), [15, 11, 7, 3]);
    }

    #[test]
    fn test_reduce_row() {
        assert_eq!(reduce_row([0, 0, 0, 0]), ([0, 0, 0, 0], 0));
        assert_eq!(reduce_row([0, 2, 2, 0]), ([0, 0, 0, 4], 4));
        assert_eq!(reduce_row([0, 2, 2, 4]), ([0, 0, 4, 4], 4));
        assert_eq!(reduce_row([2, 2, 2, 2]), ([0, 0, 4, 4], 8));
        assert_eq!(reduce_row([2, 4, 8, 16]), ([2, 4, 8, 16], 0));
        assert_eq!(reduce_row([16, 16, 4, 4]), ([0, 0, 32, 8], 40));
        assert_eq!(reduce_row([16, 16, 0, 0]), ([0, 0, 0, 32], 32));
    }

    // Ensure that the board are equal, except that exactly one 0 in b1 has 
    // been changes to a 2 or a 4 in b2
    fn board_compare(b1: Board, b2: Board) {
        let mut change_count = 0;
        for i in 0..16 {
            if b1.values[i] == b2.values[i] {
                continue;
            }
            if b1.values[i] == 0 && change_count == 0 {
                if b2.values[i] == 2 || b2.values[i] == 4 {
                    // This is our only allowed change
                    change_count = 1;
                    continue;
                }
            }
            panic!("Bad match at {}\nb1: {:?}\n b2: {:?}", i, b1.values, b2.values);
        }
        if change_count == 0 {
            panic!("Did not find any zero changes\nb1: {:?}\n b2: {:?}", b1.values, b2.values)
        }
    }

    #[test]
    fn test_play() {
        let b = Board{ values: [0, 8, 0, 2, 
                                4, 8, 2, 2,
                                4, 8, 0, 0,
                                8, 8, 0, 0],
                       ..Board::default()};
        let bdown_expected = Board{ values: [0, 0, 0, 0, 
                                    0, 0, 0, 0,
                                    8, 16, 0, 0,
                                    8, 16, 2, 4],
                                    ..Board::default() };
        let mut player = GamePlayer::default();
        let bdown = player.play(&b, MoveDir::Down);
        match bdown {
            Ok(bdown) => board_compare(bdown_expected, bdown),
            Err(error) => panic!("Error making move: {}", error),
        }

        // Play returns error on invalid move
        let b = Board{ values: [0, 2, 4, 8, 
                                0, 2, 4, 8,
                                0, 2, 4, 8,
                                0, 2, 4, 8],
                        ..Board::default() };
        let bbad = player.play(&b, MoveDir::Right);
        assert!(bbad.is_err(), "Right should be an invalid move");
    }

    #[test]
    fn test_stuck() {
        let b = Board{ 
            values: [0, 8, 0, 2, 
                     4, 8, 2, 2,
                     4, 8, 0, 0,
                     8, 8, 0, 0],
            ..Board::default() };
        assert_eq!(b.stuck(), false);

        let b = Board{ 
            values: [2, 8, 16, 32, 
                     256, 16, 2, 16,
                     4, 8, 4, 8,
                     2, 4, 2, 4],
            ..Board::default() };
        assert_eq!(b.stuck(), true);
    }
}