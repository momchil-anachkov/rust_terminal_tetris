use std::io::{Read, stderr, Stdin, stdout, Stdout};
use std::io::stdin;
use std::io::Write;
use crate::core::tetris::TetrisState;

use crossterm::execute;
use crossterm::terminal::{Clear, ClearType};
use crossterm::cursor::{MoveToColumn, MoveToRow, MoveDown};
use crate::core::{Renderer, RenderState};
use crate::core::tetris::BlockType;

pub struct TerminalRenderer {
    stdout: Stdout,
}

impl Renderer for TerminalRenderer {
    fn render(&mut self, state: &RenderState) {
        match state {
            RenderState::Running(tetris_state) => {
                self.render_tetris_state(&tetris_state);
            }
            RenderState::InMenu(menu) => {
                execute!(
                    self.stdout,
                    Clear(ClearType::All),
                    MoveToColumn(0),
                    MoveToRow(0),
                ).unwrap();

                write!(self.stdout, "{}", menu.title).unwrap();
                execute!(
                    self.stdout,
                    MoveToColumn(0),
                    MoveDown(2),
                ).unwrap();

                for index in 0..menu.items.len() {
                    let format = if index == menu.selected_item {
                        write!(self.stdout, "* {}", menu.items[index].label).unwrap();
                    } else {
                        write!(self.stdout, "  {}", menu.items[index].label).unwrap();
                    };

                    execute!(
                        self.stdout,
                        MoveToColumn(0),
                        MoveDown(1),
                    ).unwrap();
                }
            }
        }
    }
}

impl TerminalRenderer {
    fn render_tetris_state(&mut self, state: &TetrisState) {
        let game_board_start_column: u16 = 15;
        let next_pieces_board_start_column: u16 = 38;
        let held_piece_board_start_column: u16 = 0;

        execute!(
            self.stdout,
            Clear(ClearType::All),
            MoveToColumn(0),
            MoveToRow(0),
        ).unwrap();

        execute!(
            self.stdout,
            MoveToColumn(held_piece_board_start_column),
            MoveToRow(0),
        ).unwrap();

        write!(self.stdout, "Hold").unwrap();

        execute!(
            self.stdout,
            MoveToColumn(game_board_start_column),
            MoveToRow(0),
        ).unwrap();

        write!(self.stdout, "Game").unwrap();

        execute!(
            self.stdout,
            MoveToColumn(next_pieces_board_start_column),
            MoveToRow(0),
        ).unwrap();

        write!(self.stdout, "Next").unwrap();

        execute!(
            self.stdout,
            MoveToColumn(game_board_start_column),
            MoveToRow(1),
        ).unwrap();

        for line in state.board.blocks {
            for block in line {
                write!(self.stdout, "{}", char_for_block_type(&block.block_type)).unwrap();
            }

            crossterm::execute!(
                self.stdout,
                crossterm::cursor::MoveDown(1),
                crossterm::cursor::MoveToColumn(game_board_start_column),
            ).unwrap();
        }

        crossterm::execute!(
            self.stdout,
            crossterm::cursor::MoveToRow(1),
            crossterm::cursor::MoveToColumn(next_pieces_board_start_column),
        ).unwrap();

        for line in state.next_pieces_board.blocks {
            for block in line {
                write!(self.stdout, "{}", char_for_block_type(&block.block_type)).unwrap();
            }

            crossterm::execute!(
                self.stdout,
                crossterm::cursor::MoveDown(1),
                crossterm::cursor::MoveToColumn(next_pieces_board_start_column),
            ).unwrap();
        }

        crossterm::execute!(
            self.stdout,
            crossterm::cursor::MoveToRow(1),
            crossterm::cursor::MoveToColumn(held_piece_board_start_column),
        ).unwrap();

        for line in state.held_piece_board.blocks {
            for block in line {
                write!(self.stdout, "{}", char_for_block_type(&block.block_type)).unwrap();
            }

            crossterm::execute!(
                self.stdout,
                crossterm::cursor::MoveDown(1),
                crossterm::cursor::MoveToColumn(held_piece_board_start_column),
            ).unwrap();
        }
    }

    pub fn new() -> TerminalRenderer {
        return TerminalRenderer {
            stdout: stdout(),
        }
    }

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

        // Be a good neighbor and read any potentially accumulated junk from stdin
        // so you can finish on a clear terminal
        read_until_empty(&mut stdin());

        crossterm::terminal::disable_raw_mode().unwrap();

        println!("Thanks for playing!");
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

fn read_until_empty(std_in: &mut Stdin) {
    let mut junk_input = Vec::new();
    let mut bytes: usize = 0;
    loop {
        bytes = std_in.read(&mut junk_input).unwrap();
        write!(stderr(), "{}\n", bytes).unwrap();
        if bytes == 0 {
            break
        }
    }
}
