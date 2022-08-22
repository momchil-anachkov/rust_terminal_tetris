use rand::seq::SliceRandom;
use crate::tetris::MoveOutcome::{GameOver, NothingSpecial, SpawnedNewPiece};

pub const BOARD_WIDTH:  usize = 10;
pub const BOARD_HEIGHT: usize = 20;

#[derive(Clone)]
#[derive(Copy)]
#[derive(Debug)]
pub enum PieceType {
    O,
    I,
    L,
    J,
    S,
    Z,
    T
}

impl PieceType {
    fn from_block_type(block_type: &BlockType) -> PieceType {
        match block_type {
            BlockType::O => PieceType::O,
            BlockType::I => PieceType::I,
            BlockType::L => PieceType::L,
            BlockType::J => PieceType::J,
            BlockType::S => PieceType::S,
            BlockType::Z => PieceType::Z,
            BlockType::T => PieceType::T,

            _ => {panic!("Tried to make a piece_type from an invalid block_type")}
        }
    }
}

#[derive(Copy)]
#[derive(Clone)]
#[derive(PartialEq)]
pub enum BlockType {
    O,
    I,
    L,
    J,
    S,
    Z,
    T,
    Ghost,
    Empty,
}

pub struct RenderState {
    pub board: Board,
    pub next_pieces_board: NextPiecesBoard,
    pub held_piece_board: HeldPieceBoard,
}

pub struct Game {
    sequence_index: usize,
    sequence: PieceSequence,
    active_piece: Piece,
    held_piece: Option<PieceType>,
    board: Board,
}

type PieceSequence = [PieceType; 350];

impl Game {
    pub fn new() -> Game {
        let board = Board {
            blocks: [[Block { block_type: BlockType::Empty }; 10]; 20]
        };

        let sequence_index: usize = 0;
        let sequence = Game::make_piece_sequence();
        let active_piece = Piece::from_piece_type(&PieceType::I);

        let mut game = Game {
            sequence_index,
            sequence,
            active_piece,
            held_piece: None,
            board,
        };

        game.spawn_next_piece();

        return game;
    }

    fn get_piece_from_sequence(sequence: &[PieceType; 350], index: usize) -> &PieceType {
        return &sequence[index % sequence.len()];
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
        self.active_piece = Piece::from_piece_type(&a);
        Game::move_piece_to_spawn_point(&mut self.active_piece, &self.board);
        self.sequence_index += 1;
    }

    fn make_piece_sequence() -> [PieceType; 350] {
        let mut rng = rand::thread_rng();

        let mut sequence: [PieceType; 350] = [PieceType::I; 350];

        for bag_index in 0..50 {
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

            for item_index in 0..7 {
                sequence[bag_index * 7 + item_index] = bag[item_index];
            }
        }

        return sequence;
    }

    pub fn hold_piece(self: &mut Game) {
        match self.held_piece {
            None => {
                self.held_piece = Some(PieceType::from_block_type(&self.active_piece.block_type));
                self.spawn_next_piece();
            }
            Some(held_piece) => {
                let new_active_piece = Piece::from_piece_type(&held_piece);
                self.held_piece = Some(PieceType::from_block_type(&self.active_piece.block_type));
                self.active_piece = new_active_piece;
                Game::move_piece_to_spawn_point(&mut self.active_piece, &self.board);
            }
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

    // Just move down, and report if hit bottom
    // Stick is a separate command

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
        Game::stick_piece_to_board(&self.active_piece, &mut self.board);
        Game::clear_full_lines(&mut self.board);
    }

    fn clear_full_lines(board: &mut Board) {
        // Scan all the lines down
        for line_index in 0..board.blocks.len() {
            let line = board.blocks[line_index];

            let mut line_is_full = true;
            for block in line.iter() {
                if block.block_type == BlockType::Empty {
                    line_is_full = false;
                }
            }

            if line_is_full {
                // Move all the lines above one down.
                for upper_line_index in (1..=line_index).rev() {
                    for column_index in 0..line.len() {
                        board.blocks[upper_line_index][column_index].block_type = board.blocks[upper_line_index - 1][column_index].block_type;
                    }
                }

                // Clear the top line
                for column_index in 0..line.len() {
                    board.blocks[0][column_index].block_type = BlockType::Empty;
                }
            }
        }
    }

    fn stick_piece_to_board(piece: &Piece, board: &mut Board) {
        board.blocks[(piece.position.y + piece.blocks()[0].y) as usize][(piece.position.x + piece.blocks()[0].x) as usize].block_type = piece.block_type;
        board.blocks[(piece.position.y + piece.blocks()[1].y) as usize][(piece.position.x + piece.blocks()[1].x) as usize].block_type = piece.block_type;
        board.blocks[(piece.position.y + piece.blocks()[2].y) as usize][(piece.position.x + piece.blocks()[2].x) as usize].block_type = piece.block_type;
        board.blocks[(piece.position.y + piece.blocks()[3].y) as usize][(piece.position.x + piece.blocks()[3].x) as usize].block_type = piece.block_type;
    }

    pub fn render_state(self: &mut Game) -> RenderState {
        let ghost_piece: Piece = calculate_and_create_ghost_piece(&self.active_piece, &self.board);

        let next_pieces_board = NextPiecesBoard::from_sequence(&self.sequence, &self.sequence_index);
        let held_piece_board = HeldPieceBoard::from_piece_type(&self.held_piece);

        let mut board = self.board;

        Game::stick_piece_to_board(&self.active_piece, &mut board);
        Game::stick_piece_to_board(&ghost_piece, &mut board);

        return RenderState {
            board,
            held_piece_board,
            next_pieces_board,
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
    pub block_type: BlockType,
    current_rotation: usize,
    rotations: [[Vector2; 4]; 4],
}

#[derive(Copy)]
#[derive(Clone)]
pub struct Board {
    pub blocks: [[Block; BOARD_WIDTH as usize]; BOARD_HEIGHT as usize],
}

pub struct HeldPieceBoard {
    pub blocks: [[Block; 6]; 5],
}

impl HeldPieceBoard {
    fn from_piece_type(maybe_piece_type: &Option<PieceType>) -> HeldPieceBoard {
        let mut held_piece_board = HeldPieceBoard {
            blocks: [ [Block{ block_type: BlockType::Empty }; 6] ; 5]
        };

        match maybe_piece_type {
            None => {}
            Some(piece_type) => {
                let mut held_piece = Piece::from_piece_type(&piece_type);
                held_piece.position.x = 2;
                held_piece.position.y = 2;

                held_piece_board.blocks[(held_piece.position.y + held_piece.blocks()[0].y) as usize][(held_piece.position.x + held_piece.blocks()[0].x) as usize].block_type = held_piece.block_type;
                held_piece_board.blocks[(held_piece.position.y + held_piece.blocks()[1].y) as usize][(held_piece.position.x + held_piece.blocks()[1].x) as usize].block_type = held_piece.block_type;
                held_piece_board.blocks[(held_piece.position.y + held_piece.blocks()[2].y) as usize][(held_piece.position.x + held_piece.blocks()[2].x) as usize].block_type = held_piece.block_type;
                held_piece_board.blocks[(held_piece.position.y + held_piece.blocks()[3].y) as usize][(held_piece.position.x + held_piece.blocks()[3].x) as usize].block_type = held_piece.block_type;
            }
        }

        return held_piece_board;
    }
}

pub struct NextPiecesBoard {
    pub blocks: [[Block; 6]; 16],
}

impl NextPiecesBoard {
    fn from_sequence(sequence: &PieceSequence, sequence_index: &usize) -> NextPiecesBoard {
        let normalized_index: usize = sequence_index % sequence.len();

        let next_pieces_types: [&PieceType; 4] = [
            &sequence[normalized_index + 0],
            &sequence[normalized_index + 1],
            &sequence[normalized_index + 2],
            &sequence[normalized_index + 3],
        ];

        let mut next_pieces = next_pieces_types.map(|piece_type| {
            return Piece::from_piece_type(piece_type);
        });

        next_pieces[0].position.x = 2;
        next_pieces[0].position.y = 2;
        next_pieces[1].position.x = 2;
        next_pieces[1].position.y = 6;
        next_pieces[2].position.x = 2;
        next_pieces[2].position.y = 10;
        next_pieces[3].position.x = 2;
        next_pieces[3].position.y = 14;

        let mut next_pieces_board = NextPiecesBoard {
            blocks: [ [Block{ block_type: BlockType::Empty }; 6] ; 16]
        };

        for next_piece in next_pieces {
            next_pieces_board.blocks[(next_piece.position.y + next_piece.blocks()[0].y) as usize][(next_piece.position.x + next_piece.blocks()[0].x) as usize].block_type = next_piece.block_type;
            next_pieces_board.blocks[(next_piece.position.y + next_piece.blocks()[1].y) as usize][(next_piece.position.x + next_piece.blocks()[1].x) as usize].block_type = next_piece.block_type;
            next_pieces_board.blocks[(next_piece.position.y + next_piece.blocks()[2].y) as usize][(next_piece.position.x + next_piece.blocks()[2].x) as usize].block_type = next_piece.block_type;
            next_pieces_board.blocks[(next_piece.position.y + next_piece.blocks()[3].y) as usize][(next_piece.position.x + next_piece.blocks()[3].x) as usize].block_type = next_piece.block_type;
        }

        return next_pieces_board;
    }
}

#[derive(Copy)]
#[derive(Clone)]
pub struct Block {
    pub block_type: BlockType,
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

    fn from_piece_type(piece_type: &PieceType) -> Piece {
        return match piece_type {
            PieceType::O => Piece::make_o(),
            PieceType::I => Piece::make_i(),
            PieceType::L => Piece::make_l(),
            PieceType::J => Piece::make_j(),
            PieceType::S => Piece::make_s(),
            PieceType::Z => Piece::make_z(),
            PieceType::T => Piece::make_t(),
        }
    }

    fn make_o() -> Piece {
        return Piece {
            block_type: BlockType::O,
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
            block_type: BlockType::L,
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
            block_type: BlockType::J,
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
            block_type: BlockType::S,
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
            block_type: BlockType::Z,
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
            block_type: BlockType::I,
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
            block_type: BlockType::T,
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
    ghost_piece.block_type = BlockType::Ghost;

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
        board.blocks[(active_piece.position.y + active_piece.blocks()[0].y) as usize][(active_piece.position.x + active_piece.blocks()[0].x) as usize].block_type != BlockType::Empty ||
        board.blocks[(active_piece.position.y + active_piece.blocks()[1].y) as usize][(active_piece.position.x + active_piece.blocks()[1].x) as usize].block_type != BlockType::Empty ||
        board.blocks[(active_piece.position.y + active_piece.blocks()[2].y) as usize][(active_piece.position.x + active_piece.blocks()[2].x) as usize].block_type != BlockType::Empty ||
        board.blocks[(active_piece.position.y + active_piece.blocks()[3].y) as usize][(active_piece.position.x + active_piece.blocks()[3].x) as usize].block_type != BlockType::Empty
    {
        return true;
    } else {
        return false;
    }
}
