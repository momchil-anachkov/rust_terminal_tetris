mod tetris;

use std::{time};
use device_query::{DeviceQuery, DeviceState, Keycode};
use crate::tetris::Game;

const TICK_INTERVAL_TIME: u128 = 1000000;
const KEY_REPEAT_INTERVAL: u128 = 100000;

fn main() -> Result<(), ()> {
    let mut last_frame_start_time: u128 = 0;
    let mut now: u128;
    let mut delta_time: u128;
    let device_state = DeviceState::new();
    let mut input_system = InputSystem::new();

    let mut game: Game = Game::new();

    crossterm::terminal::enable_raw_mode().unwrap();

    game.print_board();

    let start = time::Instant::now();
    loop {
        now = start.elapsed().as_micros();
        delta_time = now - last_frame_start_time;
        last_frame_start_time = now;

        let keys: Vec<Keycode> = device_state.get_keys();

        // TODO code:
        // Make a renderer that takes the game state, and renders it to the terminal
        //   instead of having the game print itself and know about stdout
        //   that seems like a poor idea long-term

        // TODO game:
        // Make the tick timer reset when you spawn a piece
        //   right now what can happen is that you spawn a piece and it immediately ticks down
        //   and that's not really intuitive

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
                    GameMove::Slam => game.slam(),
                }
                game.print_board();
            }
            Command::NoOp => {}
            Command::Exit => {}
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
    Slam,
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

        if self.current_frame_keys.contains(&Keycode::Left) && self.current_frame_keys.contains(&Keycode::Right) {
            return self.return_command(Command::NoOp);
        }

        if self.current_frame_keys.contains(&Keycode::Escape) || self.current_frame_keys.contains(&Keycode::LControl) && self.current_frame_keys.contains(&Keycode::C) {
            return self.return_command(Command::Exit);
        }

        if self.current_frame_keys.contains(&Keycode::Left) {
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

        if self.current_frame_keys.contains(&Keycode::Right) {
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

        if self.current_frame_keys.contains(&Keycode::Down) {
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

        if self.current_frame_keys.contains(&Keycode::Z) || self.current_frame_keys.contains(&Keycode::Up) {
            if !self.last_frame_keys.contains(&Keycode::Z) && !self.last_frame_keys.contains(&Keycode::Up) {
                return self.return_command(Command::MakeGameMove(GameMove::RotateClockwise));
            }
        }

        if self.current_frame_keys.contains(&Keycode::X) {
            if !self.last_frame_keys.contains(&Keycode::X) {
                return self.return_command(Command::MakeGameMove(GameMove::RotateCounterClockwise));
            }
        }

        if self.current_frame_keys.contains(&Keycode::Space) {
            if !self.last_frame_keys.contains(&Keycode::Space) {
                return self.return_command(Command::MakeGameMove(GameMove::Slam));
            }
        }

        return self.return_command(Command::NoOp);
    }

    fn return_command(&mut self, command: Command) -> Command {
        self.last_frame_keys = self.current_frame_keys.clone();
        return command;
    }
}
