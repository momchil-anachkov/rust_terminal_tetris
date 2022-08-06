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
const KEY_REPEAT_INTERVAL: u128 = 100000;

fn main() -> Result<(), ()> {
    let mut last_frame_start_time: u128 = 0;
    let mut now: u128;
    let mut delta_time: u128;
    let mut time_since_last_tick: u128 = 0;
    let mut time_since_last_key_repeat: u128 = 0;
    let mut time_since_key_down: u128 = 0;
    let mut key_is_pressed: bool = false;
    let mut state_changed: bool;
    let device_state = DeviceState::new();
    let mut input_system = InputSystem::new();

    let mut game: Game = Game::new();

    crossterm::terminal::enable_raw_mode().unwrap();

    game.print_board();

    let start = time::Instant::now();
    loop {
        state_changed = false;
        now = start.elapsed().as_micros();
        delta_time = now - last_frame_start_time;
        last_frame_start_time = now;

        time_since_last_tick += delta_time;

        let keys: Vec<Keycode> = device_state.get_keys();

        // TODO:
        // Build a command system that reads the inputs + delta_time and decides what commands to run
        // Process the command with the Game object and map it to a method
        // Make a renderer that takes the game state, and renders it to the terminal

        if keys.is_empty() {
            key_is_pressed = false;
            time_since_last_key_repeat = 0;
            time_since_key_down = 0;
        }

        let command = input_system.process_input(keys, delta_time);

        if command == Command::Exit {
            return Ok(());
        }

        match command {
            Command::MakeGameMove(game_move) => {
                match game_move {
                    GameMove::Tick => game.move_down_and_stick(),
                    GameMove::MoveLeft => game.move_left(),
                    GameMove::MoveRight => game.move_right(),
                    GameMove::MoveDown => game.move_down(),
                    GameMove::RotateClockwise => game.rotate_clockwise(),
                    GameMove::RotateCounterClockwise => game.rotate_counterclockwise(),
                }
                game.print_board();
            }
            Command::NoOp => {}
            Command::Exit => {}
        }

        // if keys.contains(&Keycode::LControl) && keys.contains(&Keycode::C) {
        //     crossterm::terminal::disable_raw_mode().unwrap();
        //     return Ok(());
        // }
        //
        // for key in keys.iter() {
        //     if key.eq(&Keycode::Escape) {
        //         crossterm::terminal::disable_raw_mode().unwrap();
        //         return Ok(());
        //     }
        //
        //     if !key_is_pressed {
        //         key_is_pressed = true;
        //         process_keypress(key, &mut game);
        //         state_changed = true;
        //     } else {
        //         time_since_key_down += delta_time;
        //     }
        //
        //     if time_since_key_down > KEY_REPEAT_DELAY {
        //         time_since_last_key_repeat += delta_time;
        //     }
        //
        //     if time_since_last_key_repeat > KEY_REPEAT_INTERVAL {
        //         time_since_last_key_repeat -= KEY_REPEAT_INTERVAL;
        //         process_keypress(key, &mut game);
        //         state_changed = true;
        //     }
        // }
        //
        // if time_since_last_tick > TICK_INTERVAL_TIME {
        //     time_since_last_tick -= TICK_INTERVAL_TIME;
        //     game.move_down_and_stick();
        //     state_changed = true;
        // }

        if state_changed {
            game.print_board();
        }
    }
}

#[derive(PartialEq)]
enum GameMove {
    Tick,
    MoveLeft,
    MoveRight,
    MoveDown,
    RotateClockwise,
    RotateCounterClockwise,
}

#[derive(PartialEq)]
enum Command {
    MakeGameMove(GameMove),
    NoOp,
    Exit,
}

struct InputSystem {
    last_frame_keys: Vec<Keycode>,
    current_frame_keys: Vec<Keycode>,
    time_since_last_tick: u128,
    time_since_last_left: u128,
    time_since_last_right: u128,
    time_since_last_down: u128,
}

impl InputSystem {
    fn new() -> InputSystem {
        return InputSystem {
            last_frame_keys: Vec::new(),
            current_frame_keys: Vec::new(),
            time_since_last_tick: 0,
            time_since_last_left: 0,
            time_since_last_right: 0,
            time_since_last_down: 0,
        }
    }

    fn process_input(&mut self, keys: Vec<Keycode>, delta_time: u128) -> Command {
        self.current_frame_keys = keys.clone(); // TODO: Figure out how to do this with references
        self.time_since_last_tick += delta_time;

        if self.time_since_last_tick > TICK_INTERVAL_TIME {
            self.time_since_last_tick -= TICK_INTERVAL_TIME;
            return self.return_command(Command::MakeGameMove(GameMove::Tick));
        }

        if keys.contains(&Keycode::Left) && keys.contains(&Keycode::Right) {
            return self.return_command(Command::NoOp);
        }

        if keys.contains(&Keycode::Escape) || keys.contains(&Keycode::LControl) && keys.contains(&Keycode::C) {
            return self.return_command(Command::Exit);
        }

        if keys.contains(&Keycode::Left) {
            if self.last_frame_keys.contains(&Keycode::Left) {
                self.time_since_last_left += delta_time;
                if self.time_since_last_left > KEY_REPEAT_INTERVAL {
                    self.time_since_last_left -= KEY_REPEAT_INTERVAL;
                    return self.return_command(Command::MakeGameMove(GameMove::MoveLeft));
                }
            } else {
                return self.return_command(Command::MakeGameMove(GameMove::MoveLeft));
            }
        } else {
            self.time_since_last_left = 0;
        }

        if keys.contains(&Keycode::Right) {
            if self.last_frame_keys.contains(&Keycode::Right) {
                self.time_since_last_right += delta_time;
                if self.time_since_last_right > KEY_REPEAT_INTERVAL {
                    self.time_since_last_right -= KEY_REPEAT_INTERVAL;
                    return self.return_command(Command::MakeGameMove(GameMove::MoveRight));
                }
            } else {
                return self.return_command(Command::MakeGameMove(GameMove::MoveRight));
            }
        } else {
            self.time_since_last_right = 0;
        }

        if keys.contains(&Keycode::Down) {
            if self.last_frame_keys.contains(&Keycode::Down) {
                self.time_since_last_down += delta_time;
                if self.time_since_last_down > KEY_REPEAT_INTERVAL {
                    self.time_since_last_down -= KEY_REPEAT_INTERVAL;
                    return self.return_command(Command::MakeGameMove(GameMove::MoveDown));
                }
            } else {
                return self.return_command(Command::MakeGameMove(GameMove::MoveDown));
            }
        } else {
            self.time_since_last_down = 0;
        }

        if keys.contains(&Keycode::Z) || keys.contains(&Keycode::Up) {
            return self.return_command(Command::MakeGameMove(GameMove::RotateClockwise));
        }

        if keys.contains(&Keycode::X) {
            return self.return_command(Command::MakeGameMove(GameMove::RotateCounterClockwise));
        }

        return self.return_command(Command::NoOp);
    }

    fn return_command(&mut self, command: Command) -> Command {
        self.last_frame_keys = self.current_frame_keys.clone();
        return command;
    }
}

fn process_input(keys: &Vec<Keycode>) -> Command {
    if keys.contains(&Keycode::Left) && keys.contains(&Keycode::Right) {
        return Command::NoOp;
    }

    if keys.contains(&Keycode::Escape) || keys.contains(&Keycode::LControl) && keys.contains(&Keycode::C) {
        return Command::Exit;
    }

    if keys.contains(&Keycode::Left) {
        return Command::MakeGameMove(GameMove::MoveLeft);
    }

    if keys.contains(&Keycode::Right) {
        return Command::MakeGameMove(GameMove::MoveRight);
    }

    if keys.contains(&Keycode::Down) {
        return Command::MakeGameMove(GameMove::MoveDown);
    }

    if keys.contains(&Keycode::Z) || keys.contains(&Keycode::Up) {
        return Command::MakeGameMove(GameMove::RotateClockwise);
    }

    if keys.contains(&Keycode::X) {
        return Command::MakeGameMove(GameMove::RotateCounterClockwise);
    }

    return Command::NoOp;
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