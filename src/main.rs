use std::io::{stdin, stdout};
use crossterm::{execute};
use crossterm::cursor::{MoveLeft, MoveUp, Hide, Show};
use std::{time};
use crossterm::event::KeyCode;
use crossterm::terminal::{Clear, ClearType};
use device_query::{DeviceEvents, DeviceQuery, DeviceState, Keycode};
use device_query::Keycode::Left;

struct Vector2 {
    x: u8,
    y: u8,
}

struct Square {
    position: Vector2,
    blocks: [Vector2; 4],
    pattern: char,
}

struct Board {
    blocks: [[char; 10]; 20],
}

fn main() {
    let start = time::Instant::now();
    let mut last_frame_start_time: u128 = 0;
    let mut now: u128;
    let mut delta_time: u128;
    let mut tick_threshold: u128 = 0;
    let mut key_is_pressed: bool = false;
    let mut state_changed: bool = false;

    let mut board: Board = Board {
        blocks: [[' '; 10]; 20],
    };

    let device_state = DeviceState::new();

    let mut square: Square = Square {
        position: Vector2 { x: 0, y: 0 },
        blocks: [
            Vector2 { x: 0, y: 0 },
            Vector2 { x: 1, y: 0 },
            Vector2 { x: 0, y: 1 },
            Vector2 { x: 1, y: 1 },
        ],
        pattern: 'O',
    };

    // print_board(&board, &square);

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
                    move_left(&mut square);
                    state_changed = true;
                }

                if key.eq(&Keycode::Right) {
                    move_right(&mut square);
                    state_changed = true;
                }

            }
        }

        if tick_threshold > 1000000 {
            tick_threshold -= 1000000;
            square.position.y += 1;
            state_changed = true;
        }

        if state_changed {
            print_board(&board, &square);
        }
    }
}

fn move_left(active_piece: &mut Square) {
    if
        active_piece.position.x + active_piece.blocks[0].x > 0 &&
        active_piece.position.x + active_piece.blocks[1].x > 0 &&
        active_piece.position.x + active_piece.blocks[2].x > 0 &&
        active_piece.position.x + active_piece.blocks[3].x > 0
    {
        active_piece.position.x -= 1;
    }
}

fn move_right(active_piece: &mut Square) {
    if
        active_piece.position.x + active_piece.blocks[0].x < 9 &&
        active_piece.position.x + active_piece.blocks[1].x < 9 &&
        active_piece.position.x + active_piece.blocks[2].x < 9 &&
        active_piece.position.x + active_piece.blocks[3].x < 9
    {
        active_piece.position.x += 1;
    }
}

fn print_board(board: &Board, active_piece: &Square) {
    execute!(
        stdout(),
        Clear(ClearType::All),
        MoveLeft(50),
        MoveUp(20),
    ).unwrap();

    let mut board_copy: Board = Board {
        blocks: [[' '; 10]; 20],
    };

    for y in 0..board.blocks.len() {
        for x in 0..board.blocks[0].len() {
            if
                x == (active_piece.position.x + active_piece.blocks[0].x).into() && y == (active_piece.position.y + active_piece.blocks[0].y).into() ||
                x == (active_piece.position.x + active_piece.blocks[1].x).into() && y == (active_piece.position.y + active_piece.blocks[1].y).into() ||
                x == (active_piece.position.x + active_piece.blocks[2].x).into() && y == (active_piece.position.y + active_piece.blocks[2].y).into() ||
                x == (active_piece.position.x + active_piece.blocks[3].x).into() && y == (active_piece.position.y + active_piece.blocks[3].y).into()
            {
                board_copy.blocks[y][x] = active_piece.pattern;
            } else {
                board_copy.blocks[y][x] = board.blocks[y][x];
            }
        }
    }
    for line in board_copy.blocks {
        println!("{:?}", line);
    }
}
