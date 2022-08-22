use std::io::{Read, stdout};
use std::io::stdin;
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
    crossterm::execute!(
        stdout(),
        MoveToRow(0),
        MoveToColumn(0),
        Clear(ClearType::All),
    ).unwrap();

    // Be a good neighbor and read the accumulated junk from stdin so you can finish on a clear terminal
    let mut junk_input = Vec::new();
    stdin().read(&mut junk_input).unwrap();

    crossterm::terminal::disable_raw_mode().unwrap();

    println!("Thanks for playing!");
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

    for line in state.board.blocks {
        for block in line {
            write!(stdout(), "{}", char_for_block_type(&block.block_type)).unwrap();
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
