mod tetris;

use std::io::{stdout};
use crossterm::{execute};
use crossterm::cursor::{MoveLeft, MoveUp};
use std::{time};
use crossterm::terminal::{Clear, ClearType};
use device_query::{DeviceQuery, DeviceState, Keycode};
use rand::Rng;
use crate::tetris::Game;

fn main() {
    let start = time::Instant::now();
    let mut last_frame_start_time: u128 = 0;
    let mut now: u128;
    let mut delta_time: u128;
    let mut tick_threshold: u128 = 0;
    let mut key_is_pressed: bool = false;
    let mut state_changed: bool;
    let device_state = DeviceState::new();

    let mut game: Game = Game::new();

    game.print_board();

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
                    game.move_left();
                    state_changed = true;
                }

                if key.eq(&Keycode::Right) {
                    game.move_right();
                    state_changed = true;
                }

                if key.eq(&Keycode::Down) {
                    game.move_down();
                    state_changed = true;
                }

                if key.eq(&Keycode::Z) || key.eq(&Keycode::Up) {
                    game.rotate_counterclockwise();
                    state_changed = true;
                }

                if key.eq(&Keycode::X) {
                    game.rotate_clockwise();
                    state_changed = true;
                }
            }
        }

        if tick_threshold > 1000000 {
            tick_threshold -= 1000000;
            game.move_down_and_stick();
            state_changed = true;
        }

        if state_changed {
            game.print_board();
        }
    }
}
