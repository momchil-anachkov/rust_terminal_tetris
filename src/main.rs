use std::io::{stdin, stdout};
use crossterm::{execute};
use crossterm::cursor::{MoveLeft, MoveUp, Hide, Show};
use std::{time};
use crossterm::event::KeyCode;
use crossterm::terminal::{Clear, ClearType};
use device_query::{DeviceEvents, DeviceQuery, DeviceState, Keycode};
use device_query::Keycode::Left;

struct Vector2 {
    x: i8,
    y: i8,
}

struct Piece {
    position: Vector2,
    spawn_offset: Vector2,
    blocks: [Vector2; 4],
    pattern: char,
}

impl Piece {
    fn make_square() -> Piece {
        return Piece {
            pattern: '0',
            position: Vector2 { x: 0, y: 0 },
            spawn_offset: Vector2 { x: 0, y: 0 },
            blocks: [
                Vector2 { x: 0, y: 0 },
                Vector2 { x: 1, y: 0 },
                Vector2 { x: 0, y: 1 },
                Vector2 { x: 1, y: 1 },
            ]
        }
    }

    fn make_l() -> Piece {
        return Piece {
            pattern: 'L',
            position: Vector2 { x: 0, y: 0 },
            spawn_offset: Vector2 { x: 0, y: 1 },
            blocks: [
                Vector2 { x: 0, y: -1 },
                Vector2 { x: 0, y: 0 },
                Vector2 { x: 0, y: 1 },
                Vector2 { x: 1, y: 1 },
            ]
        }
    }

    fn make_j() -> Piece {
        return Piece {
            pattern: 'J',
            position: Vector2 { x: 0, y: 0 },
            spawn_offset: Vector2 { x: 1, y: 1 },
            blocks: [
                Vector2 { x: 0, y: -1 },
                Vector2 { x: 0, y: 0 },
                Vector2 { x: 0, y: 1 },
                Vector2 { x: -1, y: 1 },
            ]
        }
    }

    fn make_s() -> Piece {
        return Piece {
            pattern: 'S',
            position: Vector2 { x: 0, y: 0 },
            spawn_offset: Vector2 { x: 1, y: 0 },
            blocks: [
                Vector2 { x: 1, y: 0 },
                Vector2 { x: 0, y: 0 },
                Vector2 { x: 0, y: 1 },
                Vector2 { x: -1, y: 1 },
            ]
        }
    }

    fn make_z() -> Piece {
        return Piece {
            pattern: 'Z',
            position: Vector2 { x: 0, y: 0 },
            spawn_offset: Vector2 { x: 1, y: 0 },
            blocks: [
                Vector2 { x: -1, y: 0 },
                Vector2 { x: 0, y: 0 },
                Vector2 { x: 0, y: 1 },
                Vector2 { x: 1, y: 1 },
            ]
        }
    }

    fn make_i() -> Piece {
        return Piece {
            pattern: 'I',
            position: Vector2 { x: 0, y: 0 },
            spawn_offset: Vector2 { x: 1, y: 0 },
            blocks: [
                Vector2 { x: -1, y: 0 },
                Vector2 { x: 0, y: 0 },
                Vector2 { x: 1, y: 0 },
                Vector2 { x: 2, y: 0 },
            ]
        }
    }

    fn make_t() -> Piece {
        return Piece {
            pattern: 'T',
            position: Vector2 { x: 0, y: 0 },
            spawn_offset: Vector2 { x: 1, y: 0 },
            blocks: [
                Vector2 { x: -1, y: 0 },
                Vector2 { x: 0, y: 0 },
                Vector2 { x: 1, y: 0 },
                Vector2 { x: 0, y: 1 },
            ]
        }
    }
}

struct Board {
    blocks: [[Block; 10]; 20],
}

#[derive(Copy)]
#[derive(Clone)]
struct Block {
    filled: bool,
    pattern: char,
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
        blocks: [ [Block { filled: false, pattern: ' ' }; 10]; 20],
    };

    let device_state = DeviceState::new();

    let mut active_piece: Piece = spawn_next_piece();

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

fn spawn_next_piece() -> Piece {
    let mut piece = Piece::make_t();
    piece.position.y = 0 + piece.spawn_offset.y;
    piece.position.x = 0 + piece.spawn_offset.x;
    return piece;
}

fn move_left(active_piece: &mut Piece, board: &Board) {
    if active_piece.position.x > 0 {
        active_piece.position.x -= 1;
        if collisions_exist(active_piece, board)
        {
            active_piece.position.x += 1;
        }
    }
}

fn move_right(active_piece: &mut Piece, board: &Board) {
    active_piece.position.x += 1;
    if collisions_exist(active_piece, board)
    {
        active_piece.position.x -= 1;
    }
}

fn move_down(active_piece: &mut Piece, board: &mut Board) {
    active_piece.position.y += 1;

    if collisions_exist(active_piece, board)
    {
        active_piece.position.y -= 1;
    }
}

fn move_down_and_stick(active_piece: &mut Piece, board: &mut Board) {
    active_piece.position.y += 1;

    if collisions_exist(active_piece, board)
    {
        active_piece.position.y -= 1;
        board.blocks[(active_piece.position.y + active_piece.blocks[0].y) as usize][(active_piece.position.x + active_piece.blocks[0].x) as usize].filled = true;
        board.blocks[(active_piece.position.y + active_piece.blocks[1].y) as usize][(active_piece.position.x + active_piece.blocks[1].x) as usize].filled = true;
        board.blocks[(active_piece.position.y + active_piece.blocks[2].y) as usize][(active_piece.position.x + active_piece.blocks[2].x) as usize].filled = true;
        board.blocks[(active_piece.position.y + active_piece.blocks[3].y) as usize][(active_piece.position.x + active_piece.blocks[3].x) as usize].filled = true;

        board.blocks[(active_piece.position.y + active_piece.blocks[0].y) as usize][(active_piece.position.x + active_piece.blocks[0].x) as usize].pattern = active_piece.pattern;
        board.blocks[(active_piece.position.y + active_piece.blocks[1].y) as usize][(active_piece.position.x + active_piece.blocks[1].x) as usize].pattern = active_piece.pattern;
        board.blocks[(active_piece.position.y + active_piece.blocks[2].y) as usize][(active_piece.position.x + active_piece.blocks[2].x) as usize].pattern = active_piece.pattern;
        board.blocks[(active_piece.position.y + active_piece.blocks[3].y) as usize][(active_piece.position.x + active_piece.blocks[3].x) as usize].pattern = active_piece.pattern;


        *active_piece = spawn_next_piece();

        active_piece.position.y = 0 + active_piece.spawn_offset.y;
        active_piece.position.x = 0 + active_piece.spawn_offset.x;
    }
}

fn collisions_exist(active_piece: &Piece, board: &Board) -> bool {
    if
        active_piece.position.x + active_piece.blocks[0].x > 9 ||
        active_piece.position.x + active_piece.blocks[1].x > 9 ||
        active_piece.position.x + active_piece.blocks[2].x > 9 ||
        active_piece.position.x + active_piece.blocks[3].x > 9 ||
        active_piece.position.y + active_piece.blocks[0].y > 19 ||
        active_piece.position.y + active_piece.blocks[1].y > 19 ||
        active_piece.position.y + active_piece.blocks[2].y > 19 ||
        active_piece.position.y + active_piece.blocks[3].y > 19 ||
        board.blocks[(active_piece.position.y + active_piece.blocks[0].y) as usize][(active_piece.position.x + active_piece.blocks[0].x) as usize].filled ||
        board.blocks[(active_piece.position.y + active_piece.blocks[1].y) as usize][(active_piece.position.x + active_piece.blocks[1].x) as usize].filled ||
        board.blocks[(active_piece.position.y + active_piece.blocks[2].y) as usize][(active_piece.position.x + active_piece.blocks[2].x) as usize].filled ||
        board.blocks[(active_piece.position.y + active_piece.blocks[3].y) as usize][(active_piece.position.x + active_piece.blocks[3].x) as usize].filled
    {
        return true;
    } else {
        return false;
    }
}

fn print_board(board: &Board, active_piece: &Piece) {
    execute!(
        stdout(),
        Clear(ClearType::All),
        MoveLeft(50),
        MoveUp(20),
    ).unwrap();

    let mut simple_board: [[char; 10]; 20] = [[' '; 10]; 20];

    for y in 0..board.blocks.len() {
        for x in 0..board.blocks[0].len() {
            if
                x == (active_piece.position.x + active_piece.blocks[0].x) as usize && y == (active_piece.position.y + active_piece.blocks[0].y) as usize ||
                x == (active_piece.position.x + active_piece.blocks[1].x) as usize && y == (active_piece.position.y + active_piece.blocks[1].y) as usize ||
                x == (active_piece.position.x + active_piece.blocks[2].x) as usize && y == (active_piece.position.y + active_piece.blocks[2].y) as usize ||
                x == (active_piece.position.x + active_piece.blocks[3].x) as usize && y == (active_piece.position.y + active_piece.blocks[3].y) as usize
            {
                simple_board[y][x] = active_piece.pattern;
            } else {
                simple_board[y][x] = board.blocks[y][x].pattern;
            }
        }
    }
    for line in simple_board {
        println!("{:?}", line);
    }
}
