mod tetris;

use std::io::{stdout};
use crossterm::{execute};
use crossterm::cursor::{MoveLeft, MoveUp};
use std::{thread, time};
use std::time::{Duration, Instant};
use crossterm::terminal::{Clear, ClearType};
use device_query::{DeviceQuery, DeviceState, Keycode};
use rand::Rng;
use crate::tetris::Game;

const TICK_INTERVAL_TIME: u128 = 1000000;
const KEY_REPEAT_DELAY: u128 = 200000;
const KEY_REPEAT_INTERVAL: u128 = 50000;

fn main() {
    let mut last_frame_start_time: u128 = 0;
    let mut now: u128;
    let mut delta_time: u128;
    let mut time_since_last_tick: u128 = 0;
    let mut time_since_last_key_repeat: u128 = 0;
    let mut time_since_key_down: u128 = 0;
    let mut key_is_pressed: bool = false;
    let mut state_changed: bool;
    let device_state = DeviceState::new();

    let mut game: Game = Game::new();

    game.print_board();

    let start = time::Instant::now();
    loop {
        state_changed = false;
        now = start.elapsed().as_micros();
        delta_time = now - last_frame_start_time;
        last_frame_start_time = now;

        time_since_last_tick += delta_time;

        let keys: Vec<Keycode> = device_state.get_keys();

        if keys.is_empty() {
            key_is_pressed = false;
            time_since_last_key_repeat = 0;
            time_since_key_down = 0;
        }

        for key in keys.iter() {
            if !key_is_pressed {
                key_is_pressed = true;
                process_keypress(key, &mut game);
                state_changed = true;
            } else {
                time_since_key_down += delta_time;
            }

            if time_since_key_down > KEY_REPEAT_DELAY {
                time_since_last_key_repeat += delta_time;
            }

            if time_since_last_key_repeat > KEY_REPEAT_INTERVAL {
                time_since_last_key_repeat -= KEY_REPEAT_INTERVAL;
                process_keypress(key, &mut game);
                state_changed = true;
            }
        }

        if time_since_last_tick > TICK_INTERVAL_TIME {
            time_since_last_tick -= TICK_INTERVAL_TIME;
            game.move_down_and_stick();
            state_changed = true;
        }

        if state_changed {
            game.print_board();
        }
    }
}

fn process_keypress(key: &Keycode, game: &mut Game) {
    if key.eq(&Keycode::Escape) {
        return;
    }

    if key.eq(&Keycode::Left) {
        game.move_left();
    }

    if key.eq(&Keycode::Right) {
        game.move_right();
    }

    if key.eq(&Keycode::Down) {
        game.move_down();
    }

    if key.eq(&Keycode::Z) || key.eq(&Keycode::Up) {
        game.rotate_counterclockwise();
    }

    if key.eq(&Keycode::X) {
        game.rotate_clockwise();
    }
}