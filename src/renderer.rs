use std::io::stdout;
use std::io::Write;
use crate::GameState;

use crossterm::execute;
// use crossterm::terminal::{Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen};
use crossterm::cursor::{MoveToColumn, MoveToRow};

pub fn setup() {
    // execute!(stdout(), EnterAlternateScreen).unwrap();
    crossterm::terminal::enable_raw_mode().unwrap();
}

pub fn teardown() {
    crossterm::terminal::disable_raw_mode().unwrap();
    // TODO: Try and clean up after yourself. Currently we're leaving all sorts of junk in the terminal
    // execute!(stdout(), LeaveAlternateScreen, Clear(ClearType::All)).unwrap();
}

pub fn print_board(state: &GameState) {
    execute!(
        stdout(),
        MoveToColumn(0),
        MoveToRow(0),
        Clear(ClearType::FromCursorDown),
    ).unwrap();


    let mut simple_board: [[char; 10]; 20] = [[' '; 10]; 20];

    for y in 0..state.board.blocks.len() {
        for x in 0..state.board.blocks[0].len() {
            if
            x == (state.active_piece.position.x + state.active_piece.blocks()[0].x) as usize && y == (state.active_piece.position.y + state.active_piece.blocks()[0].y) as usize ||
                x == (state.active_piece.position.x + state.active_piece.blocks()[1].x) as usize && y == (state.active_piece.position.y + state.active_piece.blocks()[1].y) as usize ||
                x == (state.active_piece.position.x + state.active_piece.blocks()[2].x) as usize && y == (state.active_piece.position.y + state.active_piece.blocks()[2].y) as usize ||
                x == (state.active_piece.position.x + state.active_piece.blocks()[3].x) as usize && y == (state.active_piece.position.y + state.active_piece.blocks()[3].y) as usize
            {
                simple_board[y][x] = state.active_piece.pattern;
            } else if
            x == (state.ghost_piece.position.x + state.ghost_piece.blocks()[0].x) as usize && y == (state.ghost_piece.position.y + state.ghost_piece.blocks()[0].y) as usize ||
                x == (state.ghost_piece.position.x + state.ghost_piece.blocks()[1].x) as usize && y == (state.ghost_piece.position.y + state.ghost_piece.blocks()[1].y) as usize ||
                x == (state.ghost_piece.position.x + state.ghost_piece.blocks()[2].x) as usize && y == (state.ghost_piece.position.y + state.ghost_piece.blocks()[2].y) as usize ||
                x == (state.ghost_piece.position.x + state.ghost_piece.blocks()[3].x) as usize && y == (state.ghost_piece.position.y + state.ghost_piece.blocks()[3].y) as usize
            {
                simple_board[y][x] = state.ghost_piece.pattern;
            } else
            {
                simple_board[y][x] = state.board.blocks[y][x].pattern;
            }
        }
    }
    for line in simple_board {
        for c in line {
            write!(stdout(), "{}", c).unwrap();
        }

        crossterm::execute!(
            stdout(),
            crossterm::cursor::MoveDown(1),
            crossterm::cursor::MoveToColumn(0),
        ).unwrap();
    }
}
