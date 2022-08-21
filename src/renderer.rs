use std::io::stdout;
use std::io::Write;
use crate::GameState;

use crossterm::execute;
use crossterm::terminal::{Clear, ClearType};
use crossterm::cursor::{MoveToColumn, MoveToRow};
use crate::tetris::BlockType;

pub fn setup() {
    crossterm::terminal::enable_raw_mode().unwrap();
}

pub fn teardown() {
    crossterm::terminal::disable_raw_mode().unwrap();
}

pub fn print_board(state: &GameState) {
    let game_board_start_column: u16 = 15;
    let next_pieces_board_start_column: u16 = 38;
    let held_piece_board_start_column: u16 = 0;

    execute!(
        stdout(),
        Clear(ClearType::All),
        MoveToColumn(0),
        MoveToRow(0),
    ).unwrap();

    execute!(
        stdout(),
        MoveToColumn(held_piece_board_start_column),
        MoveToRow(0),
    ).unwrap();

    write!(stdout(), "Hold").unwrap();

    execute!(
        stdout(),
        MoveToColumn(game_board_start_column),
        MoveToRow(0),
    ).unwrap();

    write!(stdout(), "Game").unwrap();

    execute!(
        stdout(),
        MoveToColumn(next_pieces_board_start_column),
        MoveToRow(0),
    ).unwrap();

    write!(stdout(), "Next").unwrap();

    execute!(
        stdout(),
        MoveToColumn(game_board_start_column),
        MoveToRow(1),
    ).unwrap();

    let mut char_board: [[char; 10]; 20] = [[' '; 10]; 20];

    for y in 0..state.board.blocks.len() {
        for x in 0..state.board.blocks[0].len() {
            if
                x == (state.active_piece.position.x + state.active_piece.blocks()[0].x) as usize && y == (state.active_piece.position.y + state.active_piece.blocks()[0].y) as usize ||
                x == (state.active_piece.position.x + state.active_piece.blocks()[1].x) as usize && y == (state.active_piece.position.y + state.active_piece.blocks()[1].y) as usize ||
                x == (state.active_piece.position.x + state.active_piece.blocks()[2].x) as usize && y == (state.active_piece.position.y + state.active_piece.blocks()[2].y) as usize ||
                x == (state.active_piece.position.x + state.active_piece.blocks()[3].x) as usize && y == (state.active_piece.position.y + state.active_piece.blocks()[3].y) as usize
            {
                char_board[y][x] = char_for_block_type(&state.active_piece.block_type);
            } else if
                x == (state.ghost_piece.position.x + state.ghost_piece.blocks()[0].x) as usize && y == (state.ghost_piece.position.y + state.ghost_piece.blocks()[0].y) as usize ||
                x == (state.ghost_piece.position.x + state.ghost_piece.blocks()[1].x) as usize && y == (state.ghost_piece.position.y + state.ghost_piece.blocks()[1].y) as usize ||
                x == (state.ghost_piece.position.x + state.ghost_piece.blocks()[2].x) as usize && y == (state.ghost_piece.position.y + state.ghost_piece.blocks()[2].y) as usize ||
                x == (state.ghost_piece.position.x + state.ghost_piece.blocks()[3].x) as usize && y == (state.ghost_piece.position.y + state.ghost_piece.blocks()[3].y) as usize
            {
                char_board[y][x] = char_for_block_type(&state.ghost_piece.block_type);
            } else
            {
                char_board[y][x] = char_for_block_type(&state.board.blocks[y][x].block_type);
            }
        }
    }
    for line in char_board {
        for c in line {
            write!(stdout(), "{}", c).unwrap();
        }

        crossterm::execute!(
            stdout(),
            crossterm::cursor::MoveDown(1),
            crossterm::cursor::MoveToColumn(game_board_start_column),
        ).unwrap();
    }

    crossterm::execute!(
        stdout(),
        crossterm::cursor::MoveToRow(1),
        crossterm::cursor::MoveToColumn(next_pieces_board_start_column),
    ).unwrap();

    for line in state.next_pieces_board.blocks {
        for block in line {
            write!(stdout(), "{}", char_for_block_type(&block.block_type)).unwrap();
        }

        crossterm::execute!(
            stdout(),
            crossterm::cursor::MoveDown(1),
            crossterm::cursor::MoveToColumn(next_pieces_board_start_column),
        ).unwrap();
    }

    crossterm::execute!(
        stdout(),
        crossterm::cursor::MoveToRow(1),
        crossterm::cursor::MoveToColumn(held_piece_board_start_column),
    ).unwrap();

    for line in state.held_piece_board.blocks {
        for block in line {
            write!(stdout(), "{}", char_for_block_type(&block.block_type)).unwrap();
        }

        crossterm::execute!(
            stdout(),
            crossterm::cursor::MoveDown(1),
            crossterm::cursor::MoveToColumn(held_piece_board_start_column),
        ).unwrap();
    }
}

fn char_for_block_type(block_type: &BlockType) -> char {
   match block_type {
       BlockType::O =>     '🟨',
       BlockType::I =>     '🟪',
       BlockType::L =>     '🟧',
       BlockType::J =>     '🟦',
       BlockType::S =>     '🟩',
       BlockType::Z =>     '🟥',
       BlockType::T =>     '🟫',
       BlockType::Ghost => '🤍',
       BlockType::Empty => '🖤',
   }
}
