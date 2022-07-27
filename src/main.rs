use std::io::stdout;
use crossterm::{execute};
use crossterm::cursor::{MoveLeft, MoveUp};
use std::{time};

struct Vector2 {
    x: u8,
    y: u8,
}

struct Square {
    position: Vector2,
    blocks: [[u8; 2]; 4],
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

    let mut board: Board = Board {
        blocks: [[' '; 10]; 20],
    };

    let mut square: Square = Square {
        position: Vector2 { x: 0, y: 0 },
        blocks: [[0, 0], [0, 1], [1, 0], [1, 1]],
        pattern: 'O',
    };

    print_board(&board, &square);

    loop {
        now = start.elapsed().as_micros();
        delta_time = now - last_frame_start_time;
        last_frame_start_time = now;

        tick_threshold += delta_time;

        if tick_threshold > 1000000 {
            tick_threshold -= 1000000;
            square.position.y += 1;
            print_board(&board, &square);
        }
    }
}

fn print_board(board: &Board, active_piece: &Square) {
    let mut board_copy: Board = Board {
        blocks: [[' '; 10]; 20],
    };

    for y in 0..board.blocks.len() {
        for x in 0..board.blocks[0].len() {
            if
                x == (active_piece.position.x + active_piece.blocks[0][0]).into() && y == (active_piece.position.y + active_piece.blocks[0][1]).into() ||
                x == (active_piece.position.x + active_piece.blocks[1][0]).into() && y == (active_piece.position.y + active_piece.blocks[1][1]).into() ||
                x == (active_piece.position.x + active_piece.blocks[2][0]).into() && y == (active_piece.position.y + active_piece.blocks[2][1]).into() ||
                x == (active_piece.position.x + active_piece.blocks[3][0]).into() && y == (active_piece.position.y + active_piece.blocks[3][1]).into()
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

    execute!(
        stdout(),
        MoveLeft(50),
        MoveUp(20),
    ).unwrap();
}
