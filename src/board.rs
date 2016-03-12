extern crate rand;

use self::rand::{thread_rng, Rng};

use super::srs;
use super::tetromino::{
    Direction,
    Point,
    Rotation, 
    Tetromino, 
    TetrominoType, 
    TYPES
};

pub const WIDTH: usize = 10;
pub const HEIGHT: usize = 22;

const SPAWN: Point = Point { x: 3, y: 0 };

const LEFT: isize = -1;
const RIGHT: isize = 1;
const UP: isize = -1;
const DOWN: isize = 1;
const NEUTRAL: isize = 0;

pub type Field = [[Option<TetrominoType>; WIDTH]; HEIGHT];

/// A struct representing a 10x22 Tetris board
pub struct Board {
    pub field: Field,
    seq: [TetrominoType; TYPES],
    curr: Tetromino,
    next: usize,
}

impl Board {

    /// Initializes a new Board struct
    pub fn new() -> Self {
        let field: Field = [[None; WIDTH]; HEIGHT];

        // Create a sequence of pieces and shuffle them
        let mut seq = [
            TetrominoType::I, 
            TetrominoType::J, 
            TetrominoType::L, 
            TetrominoType::O, 
            TetrominoType::S, 
            TetrominoType::T, 
            TetrominoType::Z,
        ];

        let mut rng = thread_rng();
        rng.shuffle(&mut seq);

        let mut board = Board {
            field: field,
            seq: seq,
            curr: Tetromino::new(SPAWN, seq[0].clone(), Rotation::Spawn),
            next: 0
        };

        board.spawn();
        board
    }

    /// Applies gravity and clears any lines
    pub fn tick(&mut self) {
        self.apply_gravity();
        self.clear_lines();
    }

    /// Applies gravity to the field
    fn apply_gravity(&mut self) {
        let offset = Point::new(NEUTRAL, DOWN);

        if self.is_moveable(&offset) {
            self.do_move(&offset);
        }

        else {
            self.spawn();
        }
    }

    /// Clears any lines from the field
    fn clear_lines(&self) {
        // TODO
    }

    /// Moves the current Tetromino to the left
    pub fn left(&mut self) {
        self.move_tetromino(Point::new(LEFT, NEUTRAL));
    }

    /// Moves the current Tetromino to the right
    pub fn right(&mut self) {
        self.move_tetromino(Point::new(RIGHT, NEUTRAL));
    }

    /// Moves the current Tetromino up
    pub fn up(&mut self) {
        self.move_tetromino(Point::new(NEUTRAL, UP));
    }

    /// Moves the current Tetromino down
    pub fn down(&mut self) {
        self.move_tetromino(Point::new(NEUTRAL, DOWN));
    }

    /// Moves the current Tetromino by an (x, y) offset
    fn move_tetromino(&mut self, offset: Point) {
        if self.is_moveable(&offset) {
            self.do_move(&offset);
        }
    }

    /// Determines if an offset to the current Tetromino is possible or not
    fn is_moveable(&self, offset: &Point) -> bool {
        for &mino in self.curr.minos.iter() {
            let org = Point::new(self.curr.pos.x + mino.x, self.curr.pos.y + mino.y);
            let pos = Point::new(org.x + offset.x, org.y + offset.y);

            // Determine if the field's boundaries are respected
            if pos.x < 0 || pos.y < 0 || (pos.x as usize) >= WIDTH || (pos.y as usize) >= HEIGHT {
                return false;
            }

             // Determine if the current Tetromino overlaps only itself if moved
            if self.field[pos.y as usize][pos.x as usize].is_some() && !self.overlaps_active(&pos) {
                return false;
            }
        }

        true
    }

    /// Determines if an offset to the current Tetromino would cause it to 
    /// only overlap itself or not
    fn overlaps_active(&self, pos: &Point) -> bool {
        for &mino in self.curr.minos.iter() {
            if pos.x == (self.curr.pos.x + mino.x) && pos.y == (self.curr.pos.y + mino.y) {
                return true;
            }
        }

        false
    }

    /// Performs the actual movement
    fn do_move(&mut self, offset: &Point) {
        let mut minos = self.curr.minos.clone();

        // Moving down or right requires us to handle the blocks in
        // reverse order to prevent blocks from being erased
        match (offset.x, offset.y) {
            (RIGHT, NEUTRAL) | (NEUTRAL, DOWN) => minos.reverse(),
            _ => { },
        }

        // Perform the move
        for &mino in minos.iter() {
            let org = Point::new(self.curr.pos.x + mino.x, self.curr.pos.y + mino.y);
            let pos = Point::new(org.x + offset.x, org.y + offset.y);
            self.field[pos.y as usize][pos.x as usize] = self.field[org.y as usize][org.x as usize];
            self.field[org.y as usize][org.x as usize] = None;
        }

        self.curr.pos = Point::new(self.curr.pos.x + offset.x, self.curr.pos.y + offset.y);
    }

    /// Rotates the current Tetromino in a specified direction
    pub fn rotate(&mut self, dir: Direction) {
        match srs::rotate(&self.field, &self.curr, dir) {
            Some((field, rotated)) => {
                self.field = field;
                self.curr = rotated;
            },

            None => { },
        }
    }

    /// Peeks at the next Tetromino
    pub fn peek_next(&self) -> Tetromino {
        Tetromino::new(SPAWN, self.seq[self.next].clone(), Rotation::Spawn)
    }

    /// Spawns the next Tetromino in the sequence
    fn spawn(&mut self) {
        self.curr = Tetromino::new(SPAWN, self.seq[self.next].clone(), Rotation::Spawn);
        self.next = self.next + 1;

        // All of the pieces have been picked, so reshuffle them
        if self.next % TYPES == 0 {
            let mut rng = thread_rng();
            rng.shuffle(&mut self.seq);
            self.next = 0;
        }

        // Add the new blocks to the board
        for &mino in self.curr.minos.iter() {
            self.field[(self.curr.pos.y + mino.y) as usize][(self.curr.pos.x + mino.x) as usize] = Some(self.curr.tetro_type);
        }
    }
}