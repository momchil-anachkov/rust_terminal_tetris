use std::io;
use std::io::stdout;
use crossterm::{ExecutableCommand, execute};
use crossterm::cursor::{MoveLeft, MoveUp};
use crossterm::style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor};
use std::{thread, time};

fn main() {
    let start = time::Instant::now();
    let mut last_frame_start_time: u128 = 0;
    let mut now: u128 = 0;
    let mut delta_time: u128 = 0;
    let mut tick_threshold: u128 = 0;

    loop {
        now = start.elapsed().as_micros();
        delta_time = now - last_frame_start_time;
        last_frame_start_time = now;

        tick_threshold += delta_time;

        if tick_threshold > 1000000 {
            tick_threshold -= 1000000;
            println!("Tick!")
        }
    }
}
