extern crate rustbox;

const BLOCKS: usize = 4;

#[derive(Copy, Clone)]
pub struct Tetromino {
    position: (usize, usize),
    blocks: [(usize, usize); BLOCKS],
}

#[derive(Copy, Clone)]
pub enum TetrominoType {
    I,
    J,
    L,
    O,
    S,
    T,
    Z,
}

#[derive(Copy, Clone)]
pub enum Rotation {
    Spawn,
    Right,
    Rot2,
    Left,
}

impl Tetromino {
    /// Initializes a new Tetromino struct of a specified type and rotation
    pub fn new(x: usize, y: usize, t_type: TetrominoType, rot: Rotation) -> Self {
        let mut tetromino = TETROMINOS[t_type as usize][rot as usize].clone();
        tetromino.set_position(x, y);
        tetromino
    }

    /// Get a reference to the Tetromino's position
    pub fn position(&self) -> &(usize, usize) {
        &self.position
    }

    /// Update the Tetromino's position
    pub fn set_position(&mut self, x: usize, y: usize) {
        self.position = (x, y);
    }

    /// Get a reference to the Tetromino's blocks
    pub fn blocks(&self) -> &[(usize, usize); BLOCKS] {
        &self.blocks
    }
}

pub const TYPES: usize = 7;
pub const ROTS: usize = 4;
const TETROMINOS: [[Tetromino; ROTS]; TYPES] = [I, J, L, O, S, T, Z];

const I: [Tetromino; ROTS] = [
    Tetromino {
        position: (0, 0),
        blocks: [(0, 1), (1, 1), (2, 1), (3, 1)]
    },

    Tetromino {
        position: (0, 0),
        blocks: [(2, 0), (2, 1), (2, 2), (2, 3)]
    },

    Tetromino {
        position: (0, 0),
        blocks: [(0, 2), (1, 2), (2, 2), (3, 2)]
    },

    Tetromino {
        position: (0, 0),
        blocks: [(0, 1), (1, 1), (2, 1), (3, 1)]
    },
];

const J: [Tetromino; ROTS] = [
    Tetromino {
        position: (0, 0),
        blocks: [(0, 0), (0, 1), (1, 1), (2, 1)]
    },

    Tetromino {
        position: (0, 0),
        blocks: [(1, 0), (1, 1), (1, 2), (2, 0)]
    },

    Tetromino {
        position: (0, 0),
        blocks: [(0, 1), (1, 1), (2, 1), (2, 2)]
    },

    Tetromino {
        position: (0, 0),
        blocks: [(1, 0), (1, 1), (1, 2), (0, 2)]
    },
];

const L: [Tetromino; ROTS] = [
    Tetromino {
        position: (0, 0),
        blocks: [(2, 0), (0, 1), (1, 1), (2, 1)]
    },

    Tetromino {
        position: (0, 0),
        blocks: [(1, 0), (1, 1), (1, 2), (2, 2)]
    },

    Tetromino {
        position: (0, 0),
        blocks: [(0, 1), (1, 1), (2, 1), (0, 2)]
    },

    Tetromino {
        position: (0, 0),
        blocks: [(0, 0), (1, 0), (1, 1), (1, 2)]
    },
];

const O: [Tetromino; ROTS] = [
    Tetromino {
        position: (0, 0),
        blocks: [(1, 0), (2, 0), (1, 1), (2, 1)]
    },

    Tetromino {
        position: (0, 0),
        blocks: [(1, 0), (2, 0), (1, 1), (2, 1)]
    },

    Tetromino {
        position: (0, 0),
        blocks: [(1, 0), (2, 0), (1, 1), (2, 1)]
    },

    Tetromino {
        position: (0, 0),
        blocks: [(1, 0), (2, 0), (1, 1), (2, 1)]
    },
];

const S: [Tetromino; ROTS] = [
    Tetromino {
        position: (0, 0),
        blocks: [(1, 0), (2, 0), (0, 1), (1, 1)]
    },

    Tetromino {
        position: (0, 0),
        blocks: [(1, 0), (1, 1), (2, 1), (2, 2)]
    },

    Tetromino {
        position: (0, 0),
        blocks: [(1, 1), (2, 1), (0, 2), (1, 2)]
    },

    Tetromino {
        position: (0, 0),
        blocks: [(0, 0), (0, 1), (1, 1), (1, 2)]
    },
];

const T: [Tetromino; ROTS] = [
    Tetromino {
        position: (0, 0),
        blocks: [(1, 0), (0, 1), (1, 1), (2, 1)]
    },

    Tetromino {
        position: (0, 0),
        blocks: [(1, 0), (1, 1), (2, 1), (1, 2)]
    },

    Tetromino {
        position: (0, 0),
        blocks: [(0, 1), (1, 1), (2, 1), (1, 2)]
    },

    Tetromino {
        position: (0, 0),
        blocks: [(1, 0), (0, 1), (1, 1), (1, 2)]
    },
];

const Z: [Tetromino; ROTS] = [
    Tetromino {
        position: (0, 0),
        blocks: [(0, 0), (1, 0), (1, 1), (2, 1)]
    },

    Tetromino {
        position: (0, 0),
        blocks: [(2, 2), (1, 1), (2, 1), (1, 2)]
    },

    Tetromino {
        position: (0, 0),
        blocks: [(0, 1), (1, 1), (1, 2), (2, 2)]
    },

    Tetromino {
        position: (0, 0),
        blocks: [(1, 0), (0, 1), (1, 1), (0, 2)]
    },
];