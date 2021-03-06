extern crate rand;

use self::rand::{thread_rng, Rng};

use super::srs;
use super::srs::Direction;
use super::tetromino::{
    Point,
    Rotation, 
    Tetromino, 
    TetrominoType, 
};

pub const WIDTH: usize = 10;
pub const HEIGHT: usize = 22;

const SPAWN: Point = Point { x: 3, y: 0 };

const LEFT: Point = Point { x: -1, y: 0 };
const RIGHT: Point = Point { x: 1, y: 0 };
const DOWN: Point = Point { x: 0, y: 1 };

const SOFT_DROP: usize = 1;
const HARD_DROP: usize = 2;

pub type Field = [[Option<TetrominoType>; WIDTH]; HEIGHT];

/// A struct representing a 10x22 Tetris board
pub struct Board {
    field: Field,
    curr: Tetromino,
    hold: Option<Tetromino>,
    is_hold_locked: bool,
    ghost: Tetromino,
    next: Vec<TetrominoType>,
    is_topped_out: bool,
    score: usize,
    level: usize,
    cleared: usize,
}

impl Board {

    /// Initializes a new Board struct
    pub fn new() -> Self {

        // Create a sequence of pieces and shuffle them
        let mut next = vec![
            TetrominoType::I, 
            TetrominoType::J, 
            TetrominoType::L, 
            TetrominoType::O, 
            TetrominoType::S, 
            TetrominoType::T, 
            TetrominoType::Z,
        ];

        let mut rng = thread_rng();
        rng.shuffle(&mut next);

        let tetromino = Tetromino::new(SPAWN, next.pop().unwrap(), Rotation::Spawn);

        let mut board = Board {
            field: [[None; WIDTH]; HEIGHT],
            curr: tetromino,
            hold: None,
            is_hold_locked: false,
            ghost: Tetromino::new_ghost(&tetromino),
            next: next,
            is_topped_out: false,
            score: 0,
            cleared: 0,
            level: 0,
        };

        board.add_current();
        board.drop_ghost(false);
        board
    }

    /// Applies gravity and clears any lines
    pub fn tick(&mut self) {
        self.apply_gravity();
    }

    /// Applies gravity to the field
    fn apply_gravity(&mut self) {
        let offset = DOWN;

        if self.is_moveable(offset) {
            self.do_move(offset);
            self.score += SOFT_DROP;
        }

        else {
            self.is_hold_locked = false;
            self.clear_lines();
            self.spawn();
        }
    }

    /// Clears any lines from the field completed by the current Tetromino
    fn clear_lines(&mut self) {
        let mut cleared = vec![];

        for &mino in self.curr.minos().iter() {
            let row = (self.curr.origin().y + mino.y) as usize;

            if self.is_line(row) {
                for col in 0..WIDTH {
                    self.field[row][col] = None;
                }

                cleared.push(row);
            }
        }

        // Drop the rows if any line clears were made
        if cleared.first().is_some() {
            self.drop_rows(*cleared.first().unwrap(), cleared.len());
            self.cleared += cleared.len();
            self.level = self.cleared / 10;
            self.score_clear(cleared.len());
        }
    }

    /// Determines if a specific row in the field is a complete line
    fn is_line(&self, row: usize) -> bool {
        for col in 0..WIDTH {
            match self.field[row][col] {
                Some(TetrominoType::Ghost) | None => return false,
                _ => continue,
            }
        }

        true
    }

    /// Drops all rows above the given start row by drop
    fn drop_rows(&mut self, start: usize, drop: usize) {

        // Use rev() because Rust doesn't support backwards iteration
        for row in (0..start).rev() {
            for col in 0..WIDTH {
                self.field[row + drop][col] = self.field[row][col].take();
            }
        }
    }

    /// Increases the score based on the number of lines cleared
    fn score_clear(&mut self, lines: usize) {

        // The scoring formula used is identical to the NES version
        self.score += match lines {
            1 =>   40 * (self.level + 1),
            2 =>  100 * (self.level + 1),
            3 =>  300 * (self.level + 1),
            4 => 1200 * (self.level + 1),

            // 4+ lines cleared is impossible in the current system.
            _ => 0,
        }
    }

    /// Moves the current Tetromino to the left
    pub fn left(&mut self) {
        self.move_tetromino(LEFT);
    }

    /// Moves the current Tetromino to the right
    pub fn right(&mut self) {
        self.move_tetromino(RIGHT);
    }

    /// Moves the current Tetromino down
    pub fn down(&mut self) {
        self.move_tetromino(DOWN);
    }

    /// Continually drops the current Tetromino until it locks
    pub fn drop_tetromino(&mut self) {
        while self.is_moveable(DOWN) {
            self.do_move(DOWN);
            self.score += HARD_DROP;
        }

        self.is_hold_locked = false;
        self.clear_lines();
        self.spawn();
    }

    /// Moves the current Tetromino by an (x, y) offset
    fn move_tetromino(&mut self, offset: Point) {
        if self.is_moveable(offset) {
            self.do_move(offset);

            if offset != DOWN {
                self.drop_ghost(true);
            }
        }
    }

    /// Determines if an offset to the current Tetromino is possible or not
    fn is_moveable(&self, offset: Point) -> bool {
        for &mino in self.curr.minos().iter() {
            let pos = self.curr.origin() + mino + offset;

            // Determine if the field's boundaries are respected
            if pos.x < 0 || pos.y < 0 || (pos.x as usize) >= WIDTH || (pos.y as usize) >= HEIGHT {
                return false;
            }

            // Determine if the offset causes any overlap for this Mino 
            match self.field[pos.y as usize][pos.x as usize] {
                Some(TetrominoType::Ghost) | None => continue,
                Some(..) if self.overlaps_active(pos) => continue,
                _ => return false,
            }
        }

        true
    }

    /// Determines if an offset to the current Tetromino would cause it to 
    /// only overlap itself or not
    fn overlaps_active(&self, new: Point) -> bool {
        for &mino in self.curr.minos().iter() {
            let pos = self.curr.origin() + mino;
            if new == pos {
                return true;
            }
        }

        false
    }

    /// Performs the actual movement
    fn do_move(&mut self, offset: Point) {
        let mut minos = self.curr.minos().clone();

        // Moving down or right requires us to handle the blocks in
        // reverse order to prevent blocks from being erased
        match offset {
            RIGHT | DOWN => minos.reverse(),
            _ => { },
        }

        // Perform the move
        for &mino in minos.iter() {
            let org = self.curr.origin() + mino;
            let pos = org + offset;

            self.field[pos.y as usize][pos.x as usize] = self.field[org.y as usize][org.x as usize].take();
        }

        let origin = self.curr.origin() + offset;
        self.curr.set_origin(origin);
    }

    /// Drops the ghost Tetromino beneath the current Tetromino
    fn drop_ghost(&mut self, erase: bool) {

        // Remove the existing ghost from the board
        if erase {
            for &mino in self.ghost.minos().iter() {
                let pos = self.ghost.origin() + mino;

                if let Some(TetrominoType::Ghost) = self.field[pos.y as usize][pos.x as usize] {
                    self.field[pos.y as usize][pos.x as usize] = None;
                }
            }
        }

        self.ghost = Tetromino::new_ghost(&self.curr);

        // Determine the position that the ghost should drop down to
        while self.is_ghost_moveable() {
            let origin = self.ghost.origin() + DOWN;
            self.ghost.set_origin(origin);
        }

        // Perform the ghost drop
        for &mino in self.ghost.minos().iter() {
            let pos = self.ghost.origin() + mino;

            if !self.overlaps_active(pos) {
                self.field[pos.y as usize][pos.x as usize] = Some(TetrominoType::Ghost);
            }
        }   
    }

    /// Determines if the ghost Tetromino can be moved down
    fn is_ghost_moveable(&self) -> bool {
        for &mino in self.ghost.minos().iter() {
            let pos = self.ghost.origin() + mino + DOWN;

            // Check that the boundary conditions are respected. We do not need
            // to check for x-axis boundaries because a ghost is located directly
            // underneath the current Tetromino
            if pos.y as usize >= HEIGHT {
                return false;
            }

            // Determine if an acceptable overlap exists. We consider other
            // ghosts and active Tetrominos to be acceptable for overlaps.
            match self.field[pos.y as usize][pos.x as usize] {
                Some(TetrominoType::Ghost) | None => continue,
                Some(..) if self.overlaps_active(pos) => continue,
                _ => return false,
            }
        }

        true
    }

    /// Rotates the current Tetromino in a specified direction
    pub fn rotate(&mut self, dir: Direction) {
        if let Some((field, rotated)) = srs::rotate(&self.field, &self.curr, dir) {
            self.field = field;
            self.curr = rotated;
            self.drop_ghost(true);
        }
    }

    /// Moves the current Tetromino into hold
    pub fn hold_tetromino(&mut self) {
        if !self.is_hold_locked {
            let curr = self.curr.clone();
            let ghost = self.ghost.clone();

            self.remove(curr);
            self.remove(ghost);
            
            self.curr.set_origin(SPAWN);

            if let Some(hold) = self.hold {
                let temp = Some(self.curr.clone());
                self.curr = hold;
                self.hold = temp;

                self.add_current();
                self.drop_ghost(false);
            }

            else {
                self.hold = Some(self.curr.clone());
                self.spawn();
            }

            self.is_hold_locked = true;
        } 
    }

    // Remove a Tetromino from the field
    fn remove(&mut self, tetromino: Tetromino) {
        for &mino in tetromino.minos().iter() {
            let pos = tetromino.origin() + mino;
            self.field[pos.y as usize][pos.x as usize] = None;
        }
    }

    /// Peeks at the next Tetromino
    pub fn peek_next(&self) -> Tetromino {
        Tetromino::new(SPAWN, *self.next.last().unwrap(), Rotation::Spawn)
    }

    /// Spawns the next Tetromino in the sequence
    fn spawn(&mut self) {
        self.curr = Tetromino::new(SPAWN, self.next.pop().unwrap(), Rotation::Spawn);
        self.drop_ghost(false);
        self.add_current();

        // All of the pieces have been picked, so reshuffle them
        if self.next.is_empty() {
            self.next = vec![
                TetrominoType::I, 
                TetrominoType::J, 
                TetrominoType::L, 
                TetrominoType::O, 
                TetrominoType::S, 
                TetrominoType::T, 
                TetrominoType::Z,
            ];

            let mut rng = thread_rng();
            rng.shuffle(&mut self.next);
        }
    }

    /// Adds the current Tetromino to the field
    fn add_current(&mut self) {

        // Add the new blocks to the board
        for &mino in self.curr.minos().iter() {
            let pos = self.curr.origin() + mino;

            // The field has been topped out and the game is over
            if self.field[pos.y as usize][pos.x as usize].is_some() {
                self.is_topped_out = true;
                break;
            }

            self.field[pos.y as usize][pos.x as usize] = Some(self.curr.tetromino_type());
        }
    }

    // GETTERS / SETTERS

    pub fn field(&self) -> Field {
        self.field
    }

    pub fn hold(&self) -> Option<Tetromino> {
        self.hold
    }

    pub fn is_topped_out(&self) -> bool {
        self.is_topped_out
    }

    pub fn score(&self) -> usize {
        self.score
    }

    pub fn level(&self) -> usize {
        self.level
    }

    pub fn cleared(&self) -> usize {
        self.cleared
    }
}