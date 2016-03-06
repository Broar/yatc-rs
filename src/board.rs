extern crate rand;
extern crate rustbox;

use self::rustbox::Color;
use self::rand::{thread_rng, Rng};

use super::tetromino::{
    Rotation, 
    Tetromino, 
    TetrominoType, 
    TYPES
};

pub const WIDTH: usize = 10;
pub const HEIGHT: usize = 24;

const SPAWN_X: isize = 3;
const SPAWN_Y: isize = 0;

const LEFT: isize = -1;
const RIGHT: isize = 1;
const UP: isize = -1;
const DOWN: isize = 1;
const NEUTRAL: isize = 0;

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

        let mut board = Board {
            blocks: blocks,
            seq: seq,
            curr: Tetromino::new(SPAWN_X, SPAWN_Y, seq[0].clone(), Rotation::Spawn),
            next: 0
        };

        board.spawn();
        board
    }

    /// Applies gravity and clears any lines
    pub fn tick(&self) {
        // TODO
    }

    /// Moves the current Tetromino to the left
    pub fn left(&mut self) {
        self.move_tetromino(LEFT, NEUTRAL);
    }

    /// Moves the current Tetromino to the right
    pub fn right(&mut self) {
        self.move_tetromino(RIGHT, NEUTRAL);
    }

    /// Moves the current Tetromino up
    pub fn up(&mut self) {
        self.move_tetromino(NEUTRAL, UP);
    }

    /// Moves the current Tetromino down
    pub fn down(&mut self) {
        self.move_tetromino(NEUTRAL, DOWN);
    }

    /// Moves the current Tetromino by an (x, y) offset
    fn move_tetromino(&mut self, x_offset: isize, y_offset: isize) {
        let &(tetro_x, tetro_y) = self.curr.position();
        let mut is_moveable = true;

        // Check the new position lies within the board and that the space
        // is not already occupied. If these conditions aren't satisfied,
        // then the Tetromino cannot be moved
        for &(x, y) in self.curr.blocks().iter() {
            let new_x = tetro_x + (x as isize) + x_offset;
            let new_y = tetro_y + (y as isize) + y_offset;

            // Check that board boundaries are respected
            if new_x >= 0 && new_y >= 0 && (new_x as usize) < WIDTH && (new_y as usize) < HEIGHT {

                // Check that overlapping blocks belong to the active Tetromino
                if self.blocks[new_y as usize][new_x as usize].is_some() {

                    let mut overlaps_active = false;

                    for &(temp_x, temp_y) in self.curr.blocks().iter() {

                        // Only overlapping an active block, so we can still move
                        if new_x == (tetro_x + (temp_x as isize)) && new_y == (tetro_y + (temp_y as isize)) {
                            overlaps_active = true;
                            break;
                        }
                    }

                    // The overlapping block was not active, so we cannot move
                    if !overlaps_active {
                        is_moveable = false;
                        break;
                    }
                }
            }

            else {
                is_moveable = false;
                break;
            }
        }

        // Move the Tetromino to the new position
        if is_moveable {
            let mut blocks = self.curr.blocks().clone();

            // Moving down or right requires us to handle the blocks in
            // reverse order to prevent blocks from being erased
            match (x_offset, y_offset) {
                (RIGHT, NEUTRAL) | (NEUTRAL, DOWN) => blocks.reverse(),
                _ => { }
            }

            for &(x, y) in blocks.iter() {
                let org_x = (tetro_x + (x as isize)) as usize;
                let org_y = (tetro_y + (y as isize)) as usize;

                // We already checked for negative values in the previous loop,
                // so we lose no information by casting to a usize 
                let new_x = (org_x as isize + x_offset) as usize;
                let new_y = (org_y as isize + y_offset) as usize;

                self.blocks[new_y][new_x] = self.blocks[org_y][org_x];
                self.blocks[org_y][org_x] = None;
            }

            // Update origin of the Tetromino to reflect the offset
            self.curr.set_position(tetro_x + x_offset, tetro_y + y_offset);
        }
    }

    /// Peeks at the next Tetromino
    pub fn peek_next(&self) -> Tetromino {
        Tetromino::new(0, 0, self.seq[self.next].clone(), Rotation::Spawn)
    }

    /// Spawns the next Tetromino in the sequence
    fn spawn(&mut self) {
        self.curr = Tetromino::new(SPAWN_X, SPAWN_Y, self.seq[self.next].clone(), Rotation::Spawn);
        self.next = self.next + 1;

        // All of the pieces have been drawn, so reshuffle them
        if self.next % TYPES == 0 {
            let mut rng = thread_rng();
            rng.shuffle(&mut self.seq);
            self.next = 0;
        }

        // Add the new blocks to the board
        for &(x, y) in self.curr.blocks().iter() {
            let &(tetro_x, tetro_y) = self.curr.position();
            self.blocks[(tetro_y + (y as isize)) as usize][(tetro_x + (x as isize)) as usize] = Some(Color::White);
        }
    }
}