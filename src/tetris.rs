use crossterm::execute;
use crossterm::terminal::{Clear, ClearType};
use crossterm::cursor::{MoveToColumn, MoveToRow};
use rand::Rng;
use std::io::{stdout, Write};

pub const BOARD_WIDTH:  usize = 10;
pub const BOARD_HEIGHT: usize = 20;

pub struct Game {
    active_piece: Piece,
    board: Board,
}

impl Game {
    pub fn new() -> Game {
        let mut game = Game {
            active_piece: Game::make_random_piece(),
            board: Board {
                blocks: [[Block { filled: false, pattern: '🖤' }; 10]; 20]
            }
        };
        game.spawn_next_piece();
        return game;
    }

    fn spawn_next_piece(self: &mut Game) {
        self.active_piece = Game::make_random_piece();
        self.active_piece.position.y = 0;
        self.active_piece.position.x = 4;
        if is_invalid_state(&self.active_piece, &self.board) {
            self.active_piece.position.y += 1;
        }
    }

    fn make_random_piece() -> Piece {
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

    pub fn rotate_clockwise(self: &mut Game) {
        self.active_piece.rotate_clockwise();

        if is_invalid_state(&self.active_piece, &self.board)
        {
            self.active_piece.rotate_counterclockwise();
        }
    }

    pub fn rotate_counterclockwise(self: &mut Game) {
        self.active_piece.rotate_counterclockwise();

        if is_invalid_state(&self.active_piece, &self.board)
        {
            self.active_piece.rotate_clockwise();
        }
    }

    pub fn move_left(self: &mut Game) {
        self.active_piece.position.x -= 1;

        if is_invalid_state(&self.active_piece, &self.board)
        {
            self.active_piece.position.x += 1;
        }
    }

    pub fn move_right(self: &mut Game) {
        self.active_piece.position.x += 1;

        if is_invalid_state(&self.active_piece, &self.board)
        {
            self.active_piece.position.x -= 1;
        }
    }

    pub fn move_down(self: &mut Game) {
        self.active_piece.position.y += 1;

        if is_invalid_state(&self.active_piece, &self.board)
        {
            self.active_piece.position.y -= 1;
        }
    }

    pub fn move_down_and_stick(self: &mut Game) {
        self.active_piece.position.y += 1;

        if is_invalid_state(&self.active_piece, &self.board)
        {
            self.active_piece.position.y -= 1;

            self.stick_current_piece();
            self.spawn_next_piece();
        }
    }

    pub fn slam(self: &mut Game) {
        loop {
            self.active_piece.position.y += 1;
            if is_invalid_state(&self.active_piece, &self.board) {
                self.active_piece.position.y -=1;

                self.stick_current_piece();
                self.spawn_next_piece();
                break;
            }
        }
    }

    fn stick_current_piece(self: &mut Game) {
        self.board.blocks[(self.active_piece.position.y + self.active_piece.blocks()[0].y) as usize][(self.active_piece.position.x + self.active_piece.blocks()[0].x) as usize].filled = true;
        self.board.blocks[(self.active_piece.position.y + self.active_piece.blocks()[1].y) as usize][(self.active_piece.position.x + self.active_piece.blocks()[1].x) as usize].filled = true;
        self.board.blocks[(self.active_piece.position.y + self.active_piece.blocks()[2].y) as usize][(self.active_piece.position.x + self.active_piece.blocks()[2].x) as usize].filled = true;
        self.board.blocks[(self.active_piece.position.y + self.active_piece.blocks()[3].y) as usize][(self.active_piece.position.x + self.active_piece.blocks()[3].x) as usize].filled = true;

        self.board.blocks[(self.active_piece.position.y + self.active_piece.blocks()[0].y) as usize][(self.active_piece.position.x + self.active_piece.blocks()[0].x) as usize].pattern = self.active_piece.pattern;
        self.board.blocks[(self.active_piece.position.y + self.active_piece.blocks()[1].y) as usize][(self.active_piece.position.x + self.active_piece.blocks()[1].x) as usize].pattern = self.active_piece.pattern;
        self.board.blocks[(self.active_piece.position.y + self.active_piece.blocks()[2].y) as usize][(self.active_piece.position.x + self.active_piece.blocks()[2].x) as usize].pattern = self.active_piece.pattern;
        self.board.blocks[(self.active_piece.position.y + self.active_piece.blocks()[3].y) as usize][(self.active_piece.position.x + self.active_piece.blocks()[3].x) as usize].pattern = self.active_piece.pattern;
    }

    pub fn print_board(self: &mut Game) {
        execute!(
            stdout(),
            MoveToColumn(0),
            MoveToRow(0),
            Clear(ClearType::FromCursorDown),
        ).unwrap();

        let ghost_piece = calculate_and_create_ghost_piece(&self.active_piece, &self.board);

        let mut simple_board: [[char; 10]; 20] = [[' '; 10]; 20];

        for y in 0..self.board.blocks.len() {
            for x in 0..self.board.blocks[0].len() {
                if
                    x == (self.active_piece.position.x + self.active_piece.blocks()[0].x) as usize && y == (self.active_piece.position.y + self.active_piece.blocks()[0].y) as usize ||
                    x == (self.active_piece.position.x + self.active_piece.blocks()[1].x) as usize && y == (self.active_piece.position.y + self.active_piece.blocks()[1].y) as usize ||
                    x == (self.active_piece.position.x + self.active_piece.blocks()[2].x) as usize && y == (self.active_piece.position.y + self.active_piece.blocks()[2].y) as usize ||
                    x == (self.active_piece.position.x + self.active_piece.blocks()[3].x) as usize && y == (self.active_piece.position.y + self.active_piece.blocks()[3].y) as usize
                {
                    simple_board[y][x] = self.active_piece.pattern;
                } else if
                    x == (ghost_piece.position.x + ghost_piece.blocks()[0].x) as usize && y == (ghost_piece.position.y + ghost_piece.blocks()[0].y) as usize ||
                    x == (ghost_piece.position.x + ghost_piece.blocks()[1].x) as usize && y == (ghost_piece.position.y + ghost_piece.blocks()[1].y) as usize ||
                    x == (ghost_piece.position.x + ghost_piece.blocks()[2].x) as usize && y == (ghost_piece.position.y + ghost_piece.blocks()[2].y) as usize ||
                    x == (ghost_piece.position.x + ghost_piece.blocks()[3].x) as usize && y == (ghost_piece.position.y + ghost_piece.blocks()[3].y) as usize
                {
                    simple_board[y][x] = ghost_piece.pattern;
                } else
                {
                    simple_board[y][x] = self.board.blocks[y][x].pattern;
                }
            }
        }
        for line in simple_board {
            for c in line {
                // print!("{}", c);
                write!(stdout(), "{}", c).unwrap();
            }

            crossterm::execute!(
                stdout(),
                crossterm::cursor::MoveDown(1),
                crossterm::cursor::MoveToColumn(0),
            ).unwrap();
        }
    }
}

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
    current_rotation: usize,
    rotations: [[Vector2; 4]; 4],
    pattern: char,
}

pub struct Board {
    pub blocks: [[Block; BOARD_WIDTH as usize]; BOARD_HEIGHT as usize],
}

#[derive(Copy)]
#[derive(Clone)]
pub struct Block {
    pub filled: bool,
    pub pattern: char,
}

/// Piece rotations are in clockwise order
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
            current_rotation: 0,
            rotations: [
                [
                    Vector2 { x: -1, y:  0 },
                    Vector2 { x:  0, y:  0 },
                    Vector2 { x:  1, y:  0 },
                    Vector2 { x:  1, y: -1 },
                ],
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
            ],
        }
    }

    fn make_j() -> Piece {
        return Piece {
            pattern: '🟦',
            position: Vector2 { x: 0, y: 0 },
            current_rotation: 0,
            rotations: [
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
                [
                    Vector2 { x:  0, y: -1 },
                    Vector2 { x:  0, y:  0 },
                    Vector2 { x:  0, y:  1 },
                    Vector2 { x: -1, y:  1 },
                ],
            ],
        }
    }

    fn make_s() -> Piece {
        return Piece {
            pattern: '🟩',
            position: Vector2 { x: 0, y: 0 },
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
            current_rotation: 0,
            rotations: [
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
            ],
        }
    }
}

fn calculate_and_create_ghost_piece(piece: &Piece, board: &Board) -> Piece {
    let mut ghost_piece = *piece;
    ghost_piece.pattern = '🤍';

    loop {
        ghost_piece.position.y += 1;
        if is_invalid_state(&ghost_piece, board)
        {
            ghost_piece.position.y -=1;
            break;
        };
    }

    return ghost_piece;
}

fn is_invalid_state(piece: &Piece, board: &Board) -> bool {
    return piece_is_out_of_bounds(piece, board) || collisions_exist(piece, board);
}

fn piece_is_out_of_bounds (piece: &Piece, board: &Board) -> bool {
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

fn collisions_exist(active_piece: &Piece, board: &Board) -> bool {
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
