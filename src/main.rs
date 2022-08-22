mod tetris;
mod renderer;
mod input_system;

use std::{thread, time};
use std::time::Duration;
use device_query::{DeviceQuery, DeviceState, Keycode};
use crate::input_system::{Command, GameMove, InputSystem};
use crate::tetris::{Game, GameState};
use crate::tetris::MoveOutcome::{GameOver, SpawnedNewPiece};

const TICK_INTERVAL_TIME: u128 = 1000000;
const KEY_REPEAT_INTERVAL: u128 = 100000;

fn main() -> Result<(), ()> {
    let mut last_frame_start_time: u128 = 0;
    let mut now: u128;
    let mut delta_time: u128;
    let device_state = DeviceState::new();
    let mut input_system = InputSystem::new(TICK_INTERVAL_TIME, KEY_REPEAT_INTERVAL);
    input_system.start();

    let mut game: Game = Game::new();

    renderer::setup();

    renderer::print_board(&game.current_state());

    let start = time::Instant::now();
    loop {
        now = start.elapsed().as_micros();
        delta_time = now - last_frame_start_time;
        last_frame_start_time = now;

        let keys: Vec<Keycode> = device_state.get_keys();

        let command = input_system.process_input(keys, delta_time);

        match command {
            Command::MakeGameMove(game_move) => {
                match game_move {
                    GameMove::MoveLeft => game.try_and_move_left(),
                    GameMove::MoveRight => game.try_and_move_right(),
                    GameMove::MoveDown => game.try_and_move_down(),
                    GameMove::RotateClockwise => game.rotate_clockwise(),
                    GameMove::RotateCounterClockwise => game.rotate_counterclockwise(),
                    GameMove::Hold => game.hold_piece(),
                    GameMove::Tick => {
                        match game.move_down_and_stick() {
                            SpawnedNewPiece => input_system.reset_tick_timer(),
                            GameOver => return exit(),
                            _ => {}
                        }
                    },
                    GameMove::Slam => {
                        match game.slam() {
                            SpawnedNewPiece => input_system.reset_tick_timer(),
                            GameOver => return exit(),
                            _ => {}
                        }
                    },
                }
                renderer::print_board(&game.current_state());
            }
            Command::Exit => return exit(),
            Command::NoOp => {}
        }
        thread::sleep(Duration::from_millis(1));
    }
}

fn exit() -> Result<(), ()> {
    renderer::teardown();
    return Ok(());
}

