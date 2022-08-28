mod renderer;
mod input_system;
mod core;

use std::{thread, time};
use std::time::Duration;
use crate::core::Game;
use crate::input_system::{InputSystem};
use crate::renderer::TerminalRenderer;
use crate::core::tetris::Tetris;
use crate::core::tetris::MoveOutcome::{SpawnedNewPiece};
use crate::core::ticker::Ticker;

const TICK_INTERVAL_TIME: u128 = 1000000;
const KEY_REPEAT_INTERVAL: u128 = 100000;

fn main() -> Result<(), ()> {
    let mut last_frame_start_time: u128 = 0;
    let mut now: u128;
    let mut delta_time: u128;

    let mut input_system = InputSystem::new(KEY_REPEAT_INTERVAL);

    TerminalRenderer::setup();

    let mut renderer = TerminalRenderer::new();
    let mut ticker: Ticker = Ticker::new(TICK_INTERVAL_TIME);
    let mut game: Game = Game::new(&mut renderer, &mut ticker);

    let start = time::Instant::now();
    game.render();
    loop {
        now = start.elapsed().as_micros();
        delta_time = now - last_frame_start_time;
        last_frame_start_time = now;
        let keys = input_system.get_keys(&delta_time);

        let should_exit = game.update(&keys, &delta_time);

        if should_exit {
            return exit();
        }

        thread::sleep(Duration::from_millis(10));
    }
}

fn exit() -> Result<(), ()> {
    TerminalRenderer::teardown();
    return Ok(());
}

