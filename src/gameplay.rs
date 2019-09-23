

extern crate rand;
use rand::Rng;
use rand::seq::SliceRandom;

pub enum MoveDir {
    Up,
    Down,
    Left,
    Right
}

pub struct Board {
    pub values: [i32; 16],
}

impl Board {
    pub fn row(&self, n: i32, reverse: bool) -> [i32; 4] {
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

    pub fn col(&self, n: i32, reverse: bool) -> [i32; 4] {
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

    pub fn set_row(&mut self, n: i32, value: [i32; 4], reverse: bool) {
        for i in 0..4 {
            let dst = if reverse {
                n*4 + 3 - i
            } else {
                n*4 + i
            };
            self.values[dst as usize] = value[i as usize];
        }
    }

    pub fn set_col(&mut self, n: i32, value: [i32; 4], reverse: bool) {
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
        let b = Board{ values: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0] };
        b
    }

    pub fn init() -> Board {
        // TODO: Generate starting cell randomly
        let b = Board{ values: [2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0] };
        b
    }
}

// Defines row/column reduction rules. It assumes movement is "right", i.e. from 
// index 0 towards index 3.
pub fn reduce_row(row: [i32; 4]) -> [i32; 4] {
    let mut row = row.clone();
    // First, shift right as needed until there are no empty (value = 0) 
    // cells to the right of non-empty ones
    let mut i = 1;
    while i <= 3 {
        if row[i] != 0 {
            i += 1; 
            continue;
        }
        for j in (1..i+1).rev() {
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
            for j in (1..i).rev() {
                row[j] = row[j-1];
            }
            row[0] = 0;
        }
        i -= 1;
    }
    row
}

// Defines the likelihood of drawing a 2. Alternative is drawing a 4. 
const P_DRAW2: f32 = 0.8;

pub fn play(b: &Board, dir: MoveDir) -> Result<Board, String> {
    let mut new = Board{ values: b.values.clone() };

    // Approach #1: define set/get functions
    //
    // The problem here is apparently that closures are always different types and 
    // so I need to do something else her...maybe boxing? 
    // Could come back
    // let (getter, setter) = match dir {
    //     Up => Box::new((|n| { new.col(n, true) }, |n, x| { new.set_col(n, x, true) } )),
    //     Down => Box::new((|n| { new.col(n, false) }, |n, x| { new.set_col(n, x, false) } )),
    //     Left => Box::new((|n| { new.row(n, true) }, |n, x| { new.set_row(n, x, true) } )),
    //     Right => Box::new((|n| { new.row(n, false) }, |n, x| { new.set_row(n, x, false) } ))
    // };
    
    // for i in 0..4 {
    //     setter(i, reduce_row(getter(i)));
    // }

    // Approach #2: fine
    let (reverse, row) = match dir {
        MoveDir::Up => (true, false),
        MoveDir::Down => (false, false),
        MoveDir::Left => (true, true),
        MoveDir::Right => (false, true)
    };
    if row {
        for i in 0..4 {
            new.set_row(i, reduce_row(new.row(i, reverse)), reverse);
        }
    } else {
        for i in 0..4 {
            new.set_col(i, reduce_row(new.col(i, reverse)), reverse);
        }
    }

    if new.values == b.values {
        return Err(String::from("Invalid move"))
    }

    // Collect list of 0's
    let mut zeros: Vec<i32> = Vec::new();
    for i in 0..16 {
        if new.values[i] == 0 {
            zeros.push(i as i32);
        }
    }

    // Pick a random 0 cell
    let mut rng = rand::thread_rng();
    let set_idx = zeros.choose(&mut rng);
    let rval: f32 = rng.gen_range(0.0, 1.0);
    
    match set_idx {
        Some(set_idx) => {
            if rval <= P_DRAW2 {
                new.values[*set_idx as usize] = 2;
            } else {
                new.values[*set_idx as usize] = 4;
            }
        },
        None => panic!("No zeros to replace")
    }
    Ok(new)
}


#[cfg(test)]
mod gameplay_tests {
    use super::*;
    #[test]
    fn board_row_access() {
        let b = Board{ values: [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15] };
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
                                12, 13, 14, 15] };
        assert_eq!(b.col(2, false), [2, 6, 10, 14]);
        assert_eq!(b.col(3, false), [3, 7, 11, 15]);
        // reversed
        assert_eq!(b.col(2, true), [14, 10, 6, 2]);
        assert_eq!(b.col(3, true), [15, 11, 7, 3]);
    }

    #[test]
    fn test_reduce_row() {
        assert_eq!(reduce_row([0, 0, 0, 0]), [0, 0, 0, 0]);
        assert_eq!(reduce_row([0, 2, 2, 0]), [0, 0, 0, 4]);
        assert_eq!(reduce_row([0, 2, 2, 4]), [0, 0, 4, 4]);
        assert_eq!(reduce_row([2, 2, 2, 2]), [0, 0, 4, 4]);
        assert_eq!(reduce_row([2, 4, 8, 16]), [2, 4, 8, 16]);
        assert_eq!(reduce_row([16, 16, 0, 0]), [0, 0, 0, 32]);
        assert_eq!(reduce_row([16, 16, 4, 4]), [0, 0, 32, 8]);
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
                                8, 8, 0, 0] };
        let bdown_expected = Board{ values: [0, 0, 0, 0, 
                                    0, 0, 0, 0,
                                    8, 16, 0, 0,
                                    8, 16, 2, 4] };

        let bdown = play(&b, MoveDir::Down);
        match bdown {
            Ok(bdown) => board_compare(bdown_expected, bdown),
            Err(error) => panic!("Error making move: {}", error),
        }

        // Play returns error on invalid move
        let b = Board{ values: [0, 2, 4, 8, 
                                0, 2, 4, 8,
                                0, 2, 4, 8,
                                0, 2, 4, 8] };
        let bbad = play(&b, MoveDir::Right);
        assert!(bbad.is_err(), "Right should be an invalid move");
    }
}