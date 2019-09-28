extern crate easycurses;

use easycurses::*;
use easycurses::constants::acs;

use super::*;
use super::gameplay::{GamePlayer, MoveDir, Board};

const CELL_WIDTH:i32 = 10;
const CELL_HEIGHT:i32 = 6;
const WIDTH:i32 = CELL_WIDTH * 4 + 1;
const HEIGHT:i32 = CELL_HEIGHT * 4 + 1;

fn draw_topline(easy: &mut EasyCurses) {
    easy.print_char(acs::ulcorner());
    for i in 1..(WIDTH-1) {
        if ((i)%CELL_WIDTH) == 0 {
            easy.print_char(acs::ttee());
        } else {
            easy.print_char(acs::hline());
        }
    }
    easy.print_char(acs::urcorner());
}

fn draw_vsep(easy: &mut EasyCurses) {
    easy.print_char(acs::ltee());
    for i in 1..(WIDTH-1) {
        if (i)%CELL_WIDTH == 0 {
            easy.print_char(acs::plus());
        } else {
            easy.print_char(acs::hline());
        }
    }
    easy.print_char(acs::rtee());    
}

fn draw_vblank(easy: &mut EasyCurses) {
    easy.print_char(acs::vline());
        for i in 1..(WIDTH-1) {
        if (i)%CELL_WIDTH == 0 {
            easy.print_char(acs::vline());
        } else {
            easy.print_char(' ');
        }
    }
    easy.print_char(acs::vline());    
}

fn draw_botline(easy: &mut EasyCurses) {
    easy.print_char(acs::llcorner());
    for i in 1..(WIDTH-1) {
        if ((i)%CELL_WIDTH) == 0 {
            easy.print_char(acs::btee());
        } else {
            easy.print_char(acs::hline());
        }
    }
    easy.print_char(acs::lrcorner());
}

pub fn run(algo: fn(&mut GamePlayer, &Board) -> MoveDir) {
    let mut board = gameplay::Board::init();
    let mut message = String::new();
    // Common startup
    let mut easy = EasyCurses::initialize_system().unwrap();
    easy.set_cursor_visibility(CursorVisibility::Invisible);
    easy.set_echo(false);
    easy.set_keypad_enabled(true);

    let mut suggested_move = MoveDir::Down;

    loop {
        easy.clear();

        easy.move_rc(0, 0);
        draw_topline(&mut easy);

        for line in 1..(HEIGHT-1) {
            easy.move_rc(line as i32, 0);
            if (line%CELL_HEIGHT) == 0 {
                draw_vsep(&mut easy);
            } else {
                draw_vblank(&mut easy);
            }
        }

        easy.move_rc(HEIGHT-1, 0);
        
        draw_botline(&mut easy);

        for r in 0..4 {
            let row = board.row(r as usize, false);
            for c in 0..4 {
                easy.move_rc(
                    r * CELL_HEIGHT + CELL_HEIGHT / 2,
                    c * CELL_WIDTH + CELL_WIDTH / 2);
                let value = row[c as usize];
                if value > 0 {
                    easy.print(value.to_string());
                }
            }
        }

        easy.move_rc(HEIGHT+1, 0);
        easy.print(&message);

        easy.refresh();

        

        easy.move_rc(0, 0);
        easy.insert_line();
        easy.insert_line();
        easy.print(format!("Score: {}", &board.score));
        easy.move_rc(1, 0);
        let move_str = match &suggested_move {
            MoveDir::Up => "Up",
            MoveDir::Down => "Down",
            MoveDir::Left => "Left",
            MoveDir::Right => "Right",
        };
        easy.print(format!("Suggested: {}", move_str));


        let mut try_play = |dir: MoveDir| {
            let mut player = GamePlayer::default();
            let result = player.play(&board, dir);
            match result {
                Ok(new_board) => {
                    board = new_board;
                    suggested_move = algo(&mut player, &board);
                },
                Err(error) => message = error,
            }
        };
        
        let input = easy.get_input();
        match input {
            Some(Input::KeyLeft) => try_play(MoveDir::Left),
            Some(Input::KeyRight) => try_play(MoveDir::Right),
            Some(Input::KeyUp) => try_play(MoveDir::Up),
            Some(Input::KeyDown) => try_play(MoveDir::Down),
            other => message = format!("Unknown: {:?}", other),
        }
    }
}