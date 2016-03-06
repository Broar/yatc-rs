extern crate rand;
extern crate rustbox;

use self::rustbox::Color;
use self::rand::{thread_rng, Rng};

use super::tetromino::{
    Mino,
    Point,
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
    pub field: [[Option<Mino>; WIDTH]; HEIGHT],
    seq: [TetrominoType; TYPES],

    // Info about the current Tetromino
    curr: Tetromino,
    curr_pos: Point,
    curr_type: TetrominoType,
    curr_rot: Rotation,

    next: usize,
}

impl Board {

    /// Initializes a new Board struct
    pub fn new() -> Self {
        let field: [[Option<Mino>; WIDTH]; HEIGHT] = [[None; WIDTH]; HEIGHT];

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
            field: field,
            seq: seq,

            curr: Tetromino::new(seq[0].clone(), Rotation::Spawn),
            curr_pos: Point::new(SPAWN_X, SPAWN_Y),
            curr_type: seq[0].clone(),
            curr_rot: Rotation::Spawn,

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
        self.move_tetromino(Point::new(NEUTRAL, DOWN)) ;
    }

    /// Moves the current Tetromino by an (x, y) offset
    fn move_tetromino(&mut self, offset: Point) {
        let mut is_moveable = true;

        // Check the new position lies within the board and that the space
        // is not already occupied. If these conditions aren't satisfied,
        // then the Tetromino cannot be moved
        for &mino in self.curr.minos.iter() {
            let new_pos = Point::new(self.curr_pos.x + mino.pos.x + offset.x, self.curr_pos.y + mino.pos.y + offset.y);

            // Check that board boundaries are respected
            if new_pos.x >= 0 && new_pos.y >= 0 && (new_pos.x as usize) < WIDTH && (new_pos.y as usize) < HEIGHT {

                // Check that overlapping blocks belong to the active Tetromino
                if self.field[new_pos.y as usize][new_pos.x as usize].is_some() {

                    let mut overlaps_active = false;
                    for &temp_mino in self.curr.minos.iter() {

                        // Only overlapping an active block, so we can still move
                        if new_pos.x == (self.curr_pos.x + temp_mino.pos.x) && new_pos.y == (self.curr_pos.y + temp_mino.pos.y) {
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
            let mut minos = self.curr.minos.clone();

            // Moving down or right requires us to handle the blocks in
            // reverse order to prevent blocks from being erased
            match (offset.x, offset.y) {
                (RIGHT, NEUTRAL) | (NEUTRAL, DOWN) => minos.reverse(),
                _ => { }
            }

            for &mino in minos.iter() {
                let org_pos = Point::new(self.curr_pos.x + mino.pos.x, self.curr_pos.y + mino.pos.y);
                let new_pos = Point::new(org_pos.x + offset.x, org_pos.y + offset.y);

                self.field[new_pos.y as usize][new_pos.x as usize] = self.field[org_pos.y as usize][org_pos.x as usize];
                self.field[org_pos.y as usize][org_pos.x as usize] = None;
            }

            // Update origin of the Tetromino to reflect the offset
            self.curr_pos = Point::new(self.curr_pos.x + offset.x, self.curr_pos.y + offset.y);
        }
    }

    /// Peeks at the next Tetromino
    pub fn peek_next(&self) -> Tetromino {
        Tetromino::new(self.seq[self.next].clone(), Rotation::Spawn)
    }

    /// Spawns the next Tetromino in the sequence
    fn spawn(&mut self) {
        self.curr = Tetromino::new(self.seq[self.next].clone(), Rotation::Spawn);
        self.curr_pos = Point::new(SPAWN_X, SPAWN_Y);
        self.curr_type = self.seq[self.next].clone();
        self.curr_rot = Rotation::Spawn;

        self.next = self.next + 1;

        // All of the pieces have been picked, so reshuffle them
        if self.next % TYPES == 0 {
            let mut rng = thread_rng();
            rng.shuffle(&mut self.seq);
            self.next = 0;
        }

        // Add the new blocks to the board
        for &mino in self.curr.minos.iter() {
            self.field[(self.curr_pos.y + mino.pos.y) as usize][(self.curr_pos.x + mino.pos.x) as usize] = Some(mino.clone());
        }
    }
}