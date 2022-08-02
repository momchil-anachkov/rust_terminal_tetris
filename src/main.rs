mod tetris;

use std::io::{stdout};
use crossterm::{execute};
use crossterm::cursor::{MoveLeft, MoveUp};
use std::{time};
use crossterm::terminal::{Clear, ClearType};
use device_query::{DeviceQuery, DeviceState, Keycode};
use rand::Rng;
use crate::tetris::{Block, Board, BOARD_HEIGHT, BOARD_WIDTH, move_down, move_down_and_stick, move_left, move_right, Piece, print_board, rotate_clockwise, rotate_counterclockwise, spawn_next_piece};

fn main() {
    let start = time::Instant::now();
    let mut last_frame_start_time: u128 = 0;
    let mut now: u128;
    let mut delta_time: u128;
    let mut tick_threshold: u128 = 0;
    let mut key_is_pressed: bool = false;
    let mut state_changed: bool;

    let mut board: Board = Board {
        blocks: [ [Block { filled: false, pattern: '🖤' }; BOARD_WIDTH]; BOARD_HEIGHT],
    };

    let device_state = DeviceState::new();

    let mut active_piece: Piece = spawn_next_piece();

    print_board(&board, &active_piece);

    loop {
        state_changed = false;
        now = start.elapsed().as_micros();
        delta_time = now - last_frame_start_time;
        last_frame_start_time = now;

        tick_threshold += delta_time;

        let keys: Vec<Keycode> = device_state.get_keys();

        if keys.is_empty() {
            key_is_pressed = false;
        }

        for key in keys.iter() {
            if !key_is_pressed {
                key_is_pressed = true;

                if key.eq(&Keycode::Left) {
                    move_left(&mut active_piece, &board);
                    state_changed = true;
                }

                if key.eq(&Keycode::Right) {
                    move_right(&mut active_piece, &board);
                    state_changed = true;
                }

                if key.eq(&Keycode::Down) {
                    move_down(&mut active_piece, &mut board);
                    state_changed = true;
                }

                if key.eq(&Keycode::Z) {
                    rotate_counterclockwise(&mut active_piece, &mut board);
                    state_changed = true;
                }

                if key.eq(&Keycode::X) {
                    rotate_clockwise(&mut active_piece, &mut board);
                    state_changed = true;
                }
            }
        }

        if tick_threshold > 1000000 {
            tick_threshold -= 1000000;
            move_down_and_stick(&mut active_piece, &mut board);
            state_changed = true;
        }

        if state_changed {
            print_board(&board, &active_piece);
        }
    }
}
