use crossterm::execute;
use crossterm::terminal::{Clear, ClearType};
use crossterm::cursor::{MoveLeft, MoveUp};
use rand::Rng;
use std::io::stdout;

pub const BOARD_WIDTH:  usize = 10;
pub const BOARD_HEIGHT: usize = 20;

#[derive(Copy)]
#[derive(Clone)]
pub struct Vector2 {
    x: i8,
    y: i8,
}

#[derive(Copy)]
#[derive(Clone)]
pub struct Piece {
    position: Vector2,
    spawn_offset: Vector2,
    current_rotation: usize,
    rotations: [[Vector2; 4]; 4],
    pattern: char,
}

pub struct Board {
    pub blocks: [[Block; BOARD_WIDTH as usize]; BOARD_HEIGHT as usize],
}

impl Piece {
    fn blocks(self: &Piece) -> &[Vector2; 4] {
        return &self.rotations[self.current_rotation];
    }

    fn rotate_clockwise(self: &mut Piece) {
        if self.current_rotation == 3 {
            self.current_rotation = 0;
        } else {
            self.current_rotation += 1;
        }
    }

    fn rotate_counterclockwise(self: &mut Piece) {
        if self.current_rotation == 0 {
            self.current_rotation = 3;
        } else {
            self.current_rotation -= 1;
        }
    }

    fn make_square() -> Piece {
        return Piece {
            pattern: '🟨',
            position: Vector2 { x: 0, y: 0 },
            spawn_offset: Vector2 { x: 0, y: 0 },
            current_rotation: 0,
            rotations: [
                [
                    Vector2 { x: 0, y: 0 },
                    Vector2 { x: 1, y: 0 },
                    Vector2 { x: 0, y: 1 },
                    Vector2 { x: 1, y: 1 },
                ],
                [
                    Vector2 { x: 0, y: 0 },
                    Vector2 { x: 1, y: 0 },
                    Vector2 { x: 0, y: 1 },
                    Vector2 { x: 1, y: 1 },
                ],
                [
                    Vector2 { x: 0, y: 0 },
                    Vector2 { x: 1, y: 0 },
                    Vector2 { x: 0, y: 1 },
                    Vector2 { x: 1, y: 1 },
                ],
                [
                    Vector2 { x: 0, y: 0 },
                    Vector2 { x: 1, y: 0 },
                    Vector2 { x: 0, y: 1 },
                    Vector2 { x: 1, y: 1 },
                ],
            ],
        }
    }

    fn make_l() -> Piece {
        return Piece {
            pattern: '🟧',
            position: Vector2 { x: 0, y: 0 },
            spawn_offset: Vector2 { x: 0, y: 1 },
            current_rotation: 0,
            rotations: [
                [
                    Vector2 { x: 0, y: -1 },
                    Vector2 { x: 0, y:  0 },
                    Vector2 { x: 0, y:  1 },
                    Vector2 { x: 1, y:  1 },
                ],
                [
                    Vector2 { x: -1, y: 1 },
                    Vector2 { x: -1, y: 0 },
                    Vector2 { x: 0,  y: 0 },
                    Vector2 { x: 1,  y: 0 },
                ],
                [
                    Vector2 { x: -1, y: -1 },
                    Vector2 { x:  0, y: -1 },
                    Vector2 { x:  0, y: 0  },
                    Vector2 { x:  0, y: 1  },
                ],
                [
                    Vector2 { x: -1, y:  0 },
                    Vector2 { x:  0, y:  0 },
                    Vector2 { x:  1, y:  0 },
                    Vector2 { x:  1, y: -1 },
                ],
            ],
        }
    }

    fn make_j() -> Piece {
        return Piece {
            pattern: '🟦',
            position: Vector2 { x: 0, y: 0 },
            spawn_offset: Vector2 { x: 1, y: 1 },
            current_rotation: 0,
            rotations: [
                [
                    Vector2 { x:  0, y: -1 },
                    Vector2 { x:  0, y:  0 },
                    Vector2 { x:  0, y:  1 },
                    Vector2 { x: -1, y:  1 },
                ],
                [
                    Vector2 { x: -1, y: -1 },
                    Vector2 { x: -1, y:  0 },
                    Vector2 { x:  0, y:  0 },
                    Vector2 { x:  1, y:  0 },
                ],
                [
                    Vector2 { x: 1, y: -1 },
                    Vector2 { x: 0, y: -1 },
                    Vector2 { x: 0, y:  0 },
                    Vector2 { x: 0, y:  1 },
                ],
                [
                    Vector2 { x: -1, y: 0 },
                    Vector2 { x:  0, y: 0 },
                    Vector2 { x:  1, y: 0 },
                    Vector2 { x:  1, y: 1 },
                ],
            ],
        }
    }

    fn make_s() -> Piece {
        return Piece {
            pattern: '🟩',
            position: Vector2 { x: 0, y: 0 },
            spawn_offset: Vector2 { x: 1, y: 0 },
            current_rotation: 0,
            rotations: [
                [
                    Vector2 { x:  1, y: 0 },
                    Vector2 { x:  0, y: 0 },
                    Vector2 { x:  0, y: 1 },
                    Vector2 { x: -1, y: 1 },
                ],
                [
                    Vector2 { x: 0, y: -1 },
                    Vector2 { x: 0, y:  0 },
                    Vector2 { x: 1, y:  0 },
                    Vector2 { x: 1, y:  1 },
                ],
                [
                    Vector2 { x:  1, y: 1 },
                    Vector2 { x:  0, y: 1 },
                    Vector2 { x:  0, y: 2 },
                    Vector2 { x: -1, y: 2 },
                ],
                [
                    Vector2 { x: -1, y: -1 },
                    Vector2 { x: -1, y:  0 },
                    Vector2 { x:  0, y:  0 },
                    Vector2 { x:  0, y:  1 },
                ],
            ],
        }
    }

    fn make_z() -> Piece {
        return Piece {
            pattern: '🟥',
            position: Vector2 { x: 0, y: 0 },
            spawn_offset: Vector2 { x: 1, y: 0 },
            current_rotation: 0,
            rotations: [
                [
                    Vector2 { x: -1, y: 0 },
                    Vector2 { x:  0, y: 0 },
                    Vector2 { x:  0, y: 1 },
                    Vector2 { x:  1, y: 1 },
                ],
                [
                    Vector2 { x: 1, y: -1 },
                    Vector2 { x: 1, y:  0 },
                    Vector2 { x: 0, y:  0 },
                    Vector2 { x: 0, y:  1 },
                ],
                [
                    Vector2 { x: -1, y: 1 },
                    Vector2 { x:  0, y: 1 },
                    Vector2 { x:  0, y: 2 },
                    Vector2 { x:  1, y: 2 },
                ],
                [
                    Vector2 { x:  0, y: -1 },
                    Vector2 { x:  0, y:  0 },
                    Vector2 { x: -1, y:  0 },
                    Vector2 { x: -1, y:  1 },
                ],
            ],
        }
    }

    fn make_i() -> Piece {
        return Piece {
            pattern: '🟪',
            position: Vector2 { x: 0, y: 0 },
            spawn_offset: Vector2 { x: 1, y: 0 },
            current_rotation: 0,
            rotations: [
                [
                    Vector2 { x: -1, y: 0 },
                    Vector2 { x:  0, y: 0 },
                    Vector2 { x:  1, y: 0 },
                    Vector2 { x:  2, y: 0 },
                ],
                [
                    Vector2 { x: 1, y: -1 },
                    Vector2 { x: 1, y:  0 },
                    Vector2 { x: 1, y:  1 },
                    Vector2 { x: 1, y:  2 },
                ],
                [
                    Vector2 { x: -1, y: 1 },
                    Vector2 { x:  0, y: 1 },
                    Vector2 { x:  1, y: 1 },
                    Vector2 { x:  2, y: 1 },
                ],
                [
                    Vector2 { x: 0, y: -1 },
                    Vector2 { x: 0, y:  0 },
                    Vector2 { x: 0, y:  1 },
                    Vector2 { x: 0, y:  2 },
                ],
            ],
        }
    }

    fn make_t() -> Piece {
        return Piece {
            pattern: '🟫',
            position: Vector2 { x: 0, y: 0 },
            spawn_offset: Vector2 { x: 1, y: 0 },
            current_rotation: 0,
            rotations: [
                [
                    Vector2 { x: -1, y: 0 },
                    Vector2 { x:  0, y: 0 },
                    Vector2 { x:  0, y: 1 },
                    Vector2 { x:  1, y: 0 },
                ],
                [
                    Vector2 { x:  0, y: -1 },
                    Vector2 { x:  0, y:  0 },
                    Vector2 { x: -1, y:  0 },
                    Vector2 { x:  0, y:  1 },
                ],
                [
                    Vector2 { x: -1, y:  0 },
                    Vector2 { x:  0, y:  0 },
                    Vector2 { x:  0, y: -1 },
                    Vector2 { x:  1, y:  0 },
                ],
                [
                    Vector2 { x: 0, y: -1 },
                    Vector2 { x: 0, y:  0 },
                    Vector2 { x: 1, y:  0 },
                    Vector2 { x: 0, y:  1 },
                ],
            ],
        }
    }
}

#[derive(Copy)]
#[derive(Clone)]
pub struct Block {
    pub filled: bool,
    pub pattern: char,
}

pub fn spawn_next_piece() -> Piece {
    let mut piece = make_random_piece();
    piece.position.y = 0 + piece.spawn_offset.y;
    piece.position.x = 0 + piece.spawn_offset.x;
    return piece;
}

pub fn make_random_piece() -> Piece {
    let mut rng = rand::thread_rng();
    let number: u8 = rng.gen_range(0..6);
    match number {
        0 => Piece::make_square(),
        1 => Piece::make_i(),
        2 => Piece::make_j(),
        3 => Piece::make_l(),
        4 => Piece::make_s(),
        5 => Piece::make_t(),
        6 => Piece::make_z(),
        _ => Piece::make_square(),
    }
}

pub fn rotate_clockwise(active_piece: &mut Piece, board: &Board) {
    active_piece.rotate_clockwise();

    if piece_is_out_of_bounds(active_piece, board) || collisions_exist(active_piece, board)
    {
        active_piece.rotate_counterclockwise();
    }
}

pub fn rotate_counterclockwise(active_piece: &mut Piece, board: &Board) {
    active_piece.rotate_counterclockwise();

    if piece_is_out_of_bounds(active_piece, board) || collisions_exist(active_piece, board)
    {
        active_piece.rotate_clockwise();
    }
}

pub fn move_left(active_piece: &mut Piece, board: &Board) {
    active_piece.position.x -= 1;

    if piece_is_out_of_bounds(active_piece, board) || collisions_exist(active_piece, board)
    {
        active_piece.position.x += 1;
    }
}

pub fn move_right(active_piece: &mut Piece, board: &Board) {
    active_piece.position.x += 1;

    if piece_is_out_of_bounds(active_piece, board) || collisions_exist(active_piece, board)
    {
        active_piece.position.x -= 1;
    }
}

pub fn move_down(active_piece: &mut Piece, board: &mut Board) {
    active_piece.position.y += 1;

    if piece_is_out_of_bounds(active_piece, board) || collisions_exist(active_piece, board)
    {
        active_piece.position.y -= 1;
    }
}

pub fn move_down_and_stick(active_piece: &mut Piece, board: &mut Board) {
    active_piece.position.y += 1;

    if piece_is_out_of_bounds(active_piece, board) || collisions_exist(active_piece, board)
    {
        active_piece.position.y -= 1;
        board.blocks[(active_piece.position.y + active_piece.blocks()[0].y) as usize][(active_piece.position.x + active_piece.blocks()[0].x) as usize].filled = true;
        board.blocks[(active_piece.position.y + active_piece.blocks()[1].y) as usize][(active_piece.position.x + active_piece.blocks()[1].x) as usize].filled = true;
        board.blocks[(active_piece.position.y + active_piece.blocks()[2].y) as usize][(active_piece.position.x + active_piece.blocks()[2].x) as usize].filled = true;
        board.blocks[(active_piece.position.y + active_piece.blocks()[3].y) as usize][(active_piece.position.x + active_piece.blocks()[3].x) as usize].filled = true;

        board.blocks[(active_piece.position.y + active_piece.blocks()[0].y) as usize][(active_piece.position.x + active_piece.blocks()[0].x) as usize].pattern = active_piece.pattern;
        board.blocks[(active_piece.position.y + active_piece.blocks()[1].y) as usize][(active_piece.position.x + active_piece.blocks()[1].x) as usize].pattern = active_piece.pattern;
        board.blocks[(active_piece.position.y + active_piece.blocks()[2].y) as usize][(active_piece.position.x + active_piece.blocks()[2].x) as usize].pattern = active_piece.pattern;
        board.blocks[(active_piece.position.y + active_piece.blocks()[3].y) as usize][(active_piece.position.x + active_piece.blocks()[3].x) as usize].pattern = active_piece.pattern;

        *active_piece = spawn_next_piece();
        active_piece.position.y = 0 + active_piece.spawn_offset.y;
        active_piece.position.x = 0 + active_piece.spawn_offset.x;
    }
}

pub fn collisions_exist(active_piece: &Piece, board: &Board) -> bool {
    if
    board.blocks[(active_piece.position.y + active_piece.blocks()[0].y) as usize][(active_piece.position.x + active_piece.blocks()[0].x) as usize].filled ||
        board.blocks[(active_piece.position.y + active_piece.blocks()[1].y) as usize][(active_piece.position.x + active_piece.blocks()[1].x) as usize].filled ||
        board.blocks[(active_piece.position.y + active_piece.blocks()[2].y) as usize][(active_piece.position.x + active_piece.blocks()[2].x) as usize].filled ||
        board.blocks[(active_piece.position.y + active_piece.blocks()[3].y) as usize][(active_piece.position.x + active_piece.blocks()[3].x) as usize].filled
    {
        return true;
    } else {
        return false;
    }
}

pub fn calculate_and_create_ghost_piece(piece: &Piece, board: &Board) -> Piece {
    let mut ghost_piece = *piece;
    ghost_piece.pattern = '🤍';

    loop {
        ghost_piece.position.y += 1;
        if piece_is_out_of_bounds(&ghost_piece, board) || collisions_exist(&ghost_piece, board)
        {
            ghost_piece.position.y -=1;
            break;
        };
    }

    return ghost_piece;
}

pub fn piece_is_out_of_bounds (piece: &Piece, board: &Board) -> bool {
    let board_height = board.blocks.len();
    let board_width = board.blocks[0].len();

    return
        piece.position.x + piece.blocks()[0].x < 0 ||
            piece.position.x + piece.blocks()[1].x < 0 ||
            piece.position.x + piece.blocks()[2].x < 0 ||
            piece.position.x + piece.blocks()[3].x < 0 ||
            piece.position.y + piece.blocks()[0].y < 0 ||
            piece.position.y + piece.blocks()[1].y < 0 ||
            piece.position.y + piece.blocks()[2].y < 0 ||
            piece.position.y + piece.blocks()[3].y < 0 ||
            piece.position.x + piece.blocks()[0].x == board_width  as i8 ||
            piece.position.x + piece.blocks()[1].x == board_width  as i8 ||
            piece.position.x + piece.blocks()[2].x == board_width  as i8 ||
            piece.position.x + piece.blocks()[3].x == board_width  as i8 ||
            piece.position.y + piece.blocks()[3].y == board_height as i8 ||
            piece.position.y + piece.blocks()[0].y == board_height as i8 ||
            piece.position.y + piece.blocks()[1].y == board_height as i8 ||
            piece.position.y + piece.blocks()[2].y == board_height as i8
}

pub fn print_board(board: &Board, active_piece: &Piece) {
    execute!(
        stdout(),
        Clear(ClearType::All),
        MoveLeft(50),
        MoveUp(20),
    ).unwrap();

    let ghost_piece = calculate_and_create_ghost_piece(active_piece, board);

    let mut simple_board: [[char; 10]; 20] = [[' '; 10]; 20];

    for y in 0..board.blocks.len() {
        for x in 0..board.blocks[0].len() {
            if
                x == (active_piece.position.x + active_piece.blocks()[0].x) as usize && y == (active_piece.position.y + active_piece.blocks()[0].y) as usize ||
                x == (active_piece.position.x + active_piece.blocks()[1].x) as usize && y == (active_piece.position.y + active_piece.blocks()[1].y) as usize ||
                x == (active_piece.position.x + active_piece.blocks()[2].x) as usize && y == (active_piece.position.y + active_piece.blocks()[2].y) as usize ||
                x == (active_piece.position.x + active_piece.blocks()[3].x) as usize && y == (active_piece.position.y + active_piece.blocks()[3].y) as usize
            {
                simple_board[y][x] = active_piece.pattern;
            } else if
                x == (ghost_piece.position.x + ghost_piece.blocks()[0].x) as usize && y == (ghost_piece.position.y + ghost_piece.blocks()[0].y) as usize ||
                x == (ghost_piece.position.x + ghost_piece.blocks()[1].x) as usize && y == (ghost_piece.position.y + ghost_piece.blocks()[1].y) as usize ||
                x == (ghost_piece.position.x + ghost_piece.blocks()[2].x) as usize && y == (ghost_piece.position.y + ghost_piece.blocks()[2].y) as usize ||
                x == (ghost_piece.position.x + ghost_piece.blocks()[3].x) as usize && y == (ghost_piece.position.y + ghost_piece.blocks()[3].y) as usize
            {
                simple_board[y][x] = ghost_piece.pattern;
            } else
            {
                simple_board[y][x] = board.blocks[y][x].pattern;
            }
        }
    }
    for line in simple_board {
        for c in line {
            print!("{}", c);
        }
        print!("\n");
    }
}