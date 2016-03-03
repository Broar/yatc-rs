extern crate rand;
extern crate rustbox;

use self::rustbox::Color;
use self::rand::{thread_rng, Rng};

use super::tetromino::{Tetromino, TetrominoType, Rotation, TYPES};

pub const WIDTH: usize = 10;
pub const HEIGHT: usize = 24;

/// A struct representing a 10x24 Tetris board
pub struct Board {
    pub blocks: [[Option<Color>; WIDTH]; HEIGHT],
    seq: [TetrominoType; TYPES],
    curr: Tetromino,
    next: usize,
}

impl Board {

    /// Initializes a new Board struct
    pub fn new() -> Self {
        let blocks: [[Option<Color>; WIDTH]; HEIGHT] = [[None; WIDTH]; HEIGHT];

        // Create a sequence of pieces and shuffle them
        let mut seq = [
            TetrominoType::I, 
            TetrominoType::J, 
            TetrominoType::L, 
            TetrominoType::O, 
            TetrominoType::S, 
            TetrominoType::T, 
            TetrominoType::Z
        ];

        let mut rng = thread_rng();
        rng.shuffle(&mut seq);

        Board {
            blocks: blocks,
            seq: seq,
            curr: Tetromino::new(seq[0].clone(), Rotation::Spawn),
            next: 1
        }
    }

    /// Peek at the next Tetromino
    pub fn peek_next(&self) -> Tetromino {
        Tetromino::new(self.seq[self.next].clone(), Rotation::Spawn)
    }

    /// Move the next Tetromino from the sequence into play
    fn next(&mut self) {
        self.curr = Tetromino::new(self.seq[self.next].clone(), Rotation::Spawn);
        self.next = self.next + 1;

        // All of the pieces have been drawn, so reshuffle them
        let mut rng = thread_rng();
        if self.next % TYPES == 0 {
            rng.shuffle(&mut self.seq);
        }
    }
}