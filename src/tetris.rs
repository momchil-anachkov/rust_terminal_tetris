use rand::seq::SliceRandom;
use crate::tetris::MoveOutcome::{GameOver, NothingSpecial, SpawnedNewPiece};

pub const BOARD_WIDTH:  usize = 10;
pub const BOARD_HEIGHT: usize = 20;

#[derive(Clone)]
pub enum PieceType {
    O,
    I,
    L,
    J,
    S,
    Z,
    T
}

pub struct GameState {
    pub active_piece: Piece,
    pub ghost_piece: Piece,
    pub board: Board,
}

pub struct Game {
    sequence_index: usize,
    sequence: Vec<PieceType>,
    active_piece: Piece,
    board: Board,
}

impl Game {
    pub fn new() -> Game {
        let board = Board {
            blocks: [[Block { filled: false, pattern: '🖤' }; 10]; 20]
        };

        let sequence_index: usize = 0;
        let sequence = Game::make_piece_sequence();
        let active_piece = Game::make_piece(&PieceType::I);

        let mut game = Game {
            sequence_index,
            sequence,
            active_piece,
            board,
        };

        game.spawn_next_piece();

        return game;
    }

    fn get_piece_from_sequence(sequence: &Vec<PieceType>, index: usize) -> &PieceType {
        return sequence.iter().cycle().nth(index).unwrap();
    }

    fn move_piece_to_spawn_point(piece: &mut Piece, board: &Board) {
        let width = board.blocks[0].len();
        piece.position.x = (width / 2) as i8;
        piece.position.y = 0;
        if is_invalid_state(piece, board) {
            piece.position.y += 1;
        }
    }

    fn spawn_next_piece(self: &mut Game) {
        let a = Game::get_piece_from_sequence(&self.sequence, self.sequence_index);
        self.active_piece = Game::make_piece(&a);
        Game::move_piece_to_spawn_point(&mut self.active_piece, &self.board);
        self.sequence_index += 1;
    }

    fn make_piece_sequence() -> Vec<PieceType> {
        let mut rng = rand::thread_rng();

        let mut sequence: Vec<PieceType> = Vec::new();

        for _ in 0..50 {
            let mut bag = [
                PieceType::O,
                PieceType::I,
                PieceType::L,
                PieceType::J,
                PieceType::S,
                PieceType::Z,
                PieceType::T,
            ];
            bag.shuffle(&mut rng);
            sequence.extend_from_slice(&bag);
        }

        return sequence;
    }

    fn make_piece(piece_type: &PieceType) -> Piece {
        match piece_type {
            PieceType::O => Piece::make_o(),
            PieceType::I => Piece::make_i(),
            PieceType::L => Piece::make_l(),
            PieceType::J => Piece::make_j(),
            PieceType::S => Piece::make_s(),
            PieceType::Z => Piece::make_z(),
            PieceType::T => Piece::make_t(),
        }
    }

    pub fn rotate_clockwise(self: &mut Game) {
        self.active_piece.rotate_clockwise();

        if is_invalid_state(&self.active_piece, &self.board) {
            self.try_and_move_left();
        }

        if is_invalid_state(&self.active_piece, &self.board) {
            self.try_and_move_right();
        }

        if is_invalid_state(&self.active_piece, &self.board) {
            self.try_and_move_down();
        }

        if is_invalid_state(&self.active_piece, &self.board) {
            self.try_and_move_down_left();
        }

        if is_invalid_state(&self.active_piece, &self.board) {
            self.try_and_move_up();
        }

        if is_invalid_state(&self.active_piece, &self.board) {
            self.try_and_move_up_right();
        }

        if is_invalid_state(&self.active_piece, &self.board) {
            self.active_piece.rotate_counterclockwise();
        }
    }

    pub fn rotate_counterclockwise(self: &mut Game) {
        self.active_piece.rotate_counterclockwise();

        if is_invalid_state(&self.active_piece, &self.board) {
            self.try_and_move_left();
        }

        if is_invalid_state(&self.active_piece, &self.board) {
            self.try_and_move_right();
        }

        if is_invalid_state(&self.active_piece, &self.board) {
            self.try_and_move_down();
        }

        if is_invalid_state(&self.active_piece, &self.board) {
            self.try_and_move_down_right();
        }

        if is_invalid_state(&self.active_piece, &self.board) {
            self.try_and_move_up();
        }

        if is_invalid_state(&self.active_piece, &self.board) {
            self.try_and_move_up_left();
        }

        if is_invalid_state(&self.active_piece, &self.board) {
            self.active_piece.rotate_clockwise();
        }
    }

    pub fn try_and_move_left(self: &mut Game) {
        self.active_piece.position.x -= 1;

        if is_invalid_state(&self.active_piece, &self.board) {
            self.active_piece.position.x += 1;
        }
    }

    pub fn try_and_move_right(self: &mut Game) {
        self.active_piece.position.x += 1;

        if is_invalid_state(&self.active_piece, &self.board) {
            self.active_piece.position.x -= 1;
        }
    }

    pub fn try_and_move_down(self: &mut Game) {
        self.active_piece.position.y += 1;

        if is_invalid_state(&self.active_piece, &self.board) {
            self.active_piece.position.y -= 1;
        }
    }

    fn try_and_move_up(self: &mut Game) {
        self.active_piece.position.x += 1;

        if is_invalid_state(&self.active_piece, &self.board) {
            self.active_piece.position.x -= 1;
        }
    }

    fn try_and_move_up_left(self: &mut Game) {
        self.active_piece.position.y -= 1;
        self.active_piece.position.x -= 1;

        if is_invalid_state(&self.active_piece, &self.board) {
            self.active_piece.position.y += 1;
            self.active_piece.position.x += 1;
        }
    }

    fn try_and_move_up_right(self: &mut Game) {
        self.active_piece.position.y -= 1;
        self.active_piece.position.x += 1;

        if is_invalid_state(&self.active_piece, &self.board) {
            self.active_piece.position.y += 1;
            self.active_piece.position.x -= 1;
        }
    }

    fn try_and_move_down_left(self: &mut Game) {
        self.active_piece.position.y += 1;
        self.active_piece.position.x -= 1;

        if is_invalid_state(&self.active_piece, &self.board) {
            self.active_piece.position.y -= 1;
            self.active_piece.position.x += 1;
        }
    }

    fn try_and_move_down_right(self: &mut Game) {
        self.active_piece.position.y += 1;
        self.active_piece.position.x += 1;

        if is_invalid_state(&self.active_piece, &self.board) {
            self.active_piece.position.y -= 1;
            self.active_piece.position.x -= 1;
        }
    }

    pub fn move_down_and_stick(self: &mut Game) -> MoveOutcome {
        self.active_piece.position.y += 1;

        if is_invalid_state(&self.active_piece, &self.board) {
            self.active_piece.position.y -= 1;

            self.stick_current_piece();
            self.spawn_next_piece();

            if is_invalid_state(&self.active_piece, &self.board) {
                return GameOver;
            } else {
                return SpawnedNewPiece;
            }
        }

        return NothingSpecial;
    }

    pub fn slam(self: &mut Game) -> MoveOutcome {
        loop {
            self.active_piece.position.y += 1;

            if is_invalid_state(&self.active_piece, &self.board) {
                self.active_piece.position.y -=1;

                self.stick_current_piece();
                self.spawn_next_piece();

                if is_invalid_state(&self.active_piece, &self.board) {
                    return GameOver;
                } else {
                    return SpawnedNewPiece;
                }
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

        for line_index in 0..self.board.blocks.len() {
            let line = self.board.blocks[line_index];
            let mut line_is_full = true;

            for block in line.iter() {
                if !block.filled {
                    line_is_full = false;
                }
            }

            if line_is_full {
                for upper_line_index in (1..=line_index).rev() {
                    for column_index in 0..line.len() {
                        self.board.blocks[upper_line_index][column_index].filled = self.board.blocks[upper_line_index-1][column_index].filled;
                        self.board.blocks[upper_line_index][column_index].pattern = self.board.blocks[upper_line_index-1][column_index].pattern;
                    }
                }
            }
        }
    }

    pub fn current_state(self: &mut Game) -> GameState {
        let ghost_piece: Piece = calculate_and_create_ghost_piece(&self.active_piece, &self.board);

        return GameState {
            board: self.board,
            active_piece: self.active_piece,
            ghost_piece,
        }
    }
}

#[derive(Copy)]
#[derive(Clone)]
pub struct Vector2 {
    pub x: i8,
    pub y: i8,
}

#[derive(Copy)]
#[derive(Clone)]
pub struct Piece {
    pub position: Vector2,
    pub pattern: char,
    current_rotation: usize,
    rotations: [[Vector2; 4]; 4],
}

#[derive(Copy)]
#[derive(Clone)]
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
    pub fn blocks(self: &Piece) -> &[Vector2; 4] {
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

    fn make_o() -> Piece {
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

#[derive(PartialEq)]
pub enum MoveOutcome {
    SpawnedNewPiece,
    GameOver,
    NothingSpecial,
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
