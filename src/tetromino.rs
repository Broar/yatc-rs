extern crate rustbox;

const BLOCKS: usize = 4;

#[derive(Copy, Clone)]
pub struct Tetromino {
    position: (isize, isize),
    blocks: [(usize, usize); BLOCKS],
    tetro_type: TetrominoType,
    rot: Rotation,
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
    pub fn new(x: isize, y: isize, tetro_type: TetrominoType, rot: Rotation) -> Self {
        let mut tetromino = TETROMINOS[tetro_type as usize][rot as usize].clone();
        tetromino.set_position(x, y);
        tetromino
    }

    /// Gets a reference to the Tetromino's position
    pub fn position(&self) -> &(isize, isize) {
        &self.position
    }

    /// Updates the Tetromino's position
    pub fn set_position(&mut self, x: isize, y: isize) {
        self.position = (x, y);
    }

    /// Gets a reference to the Tetromino's blocks
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
        blocks: [(0, 1), (1, 1), (2, 1), (3, 1)],
        tetro_type: TetrominoType::I,
        rot: Rotation::Spawn,
    },

    Tetromino {
        position: (0, 0),
        blocks: [(2, 0), (2, 1), (2, 2), (2, 3)],
        tetro_type: TetrominoType::I,
        rot: Rotation::Right,
    },

    Tetromino {
        position: (0, 0),
        blocks: [(0, 2), (1, 2), (2, 2), (3, 2)],
        tetro_type: TetrominoType::I,
        rot: Rotation::Rot2,
    },

    Tetromino {
        position: (0, 0),
        blocks: [(0, 1), (1, 1), (2, 1), (3, 1)],
        tetro_type: TetrominoType::I,
        rot: Rotation::Left,
    },
];

const J: [Tetromino; ROTS] = [
    Tetromino {
        position: (0, 0),
        blocks: [(0, 0), (0, 1), (1, 1), (2, 1)],
        tetro_type: TetrominoType::J,
        rot: Rotation::Spawn,
    },

    Tetromino {
        position: (0, 0),
        blocks: [(1, 0), (1, 1), (1, 2), (2, 0)],
        tetro_type: TetrominoType::J,
        rot: Rotation::Right,
    },

    Tetromino {
        position: (0, 0),
        blocks: [(0, 1), (1, 1), (2, 1), (2, 2)],
        tetro_type: TetrominoType::J,
        rot: Rotation::Rot2,
    },

    Tetromino {
        position: (0, 0),
        blocks: [(1, 0), (1, 1), (1, 2), (0, 2)],
        tetro_type: TetrominoType::J,
        rot: Rotation::Left,
    },
];

const L: [Tetromino; ROTS] = [
    Tetromino {
        position: (0, 0),
        blocks: [(2, 0), (0, 1), (1, 1), (2, 1)],
        tetro_type: TetrominoType::L,
        rot: Rotation::Spawn,
    },

    Tetromino {
        position: (0, 0),
        blocks: [(1, 0), (1, 1), (1, 2), (2, 2)],
        tetro_type: TetrominoType::L,
        rot: Rotation::Right,
    },

    Tetromino {
        position: (0, 0),
        blocks: [(0, 1), (1, 1), (2, 1), (0, 2)],
        tetro_type: TetrominoType::L,
        rot: Rotation::Rot2,
    },

    Tetromino {
        position: (0, 0),
        blocks: [(0, 0), (1, 0), (1, 1), (1, 2)],
        tetro_type: TetrominoType::L,
        rot: Rotation::Left,
    },
];

const O: [Tetromino; ROTS] = [
    Tetromino {
        position: (0, 0),
        blocks: [(1, 0), (2, 0), (1, 1), (2, 1)],
        tetro_type: TetrominoType::O,
        rot: Rotation::Spawn,
    },

    Tetromino {
        position: (0, 0),
        blocks: [(1, 0), (2, 0), (1, 1), (2, 1)],
        tetro_type: TetrominoType::O,
        rot: Rotation::Right,
    },

    Tetromino {
        position: (0, 0),
        blocks: [(1, 0), (2, 0), (1, 1), (2, 1)],
        tetro_type: TetrominoType::O,
        rot: Rotation::Rot2,
    },

    Tetromino {
        position: (0, 0),
        blocks: [(1, 0), (2, 0), (1, 1), (2, 1)],
        tetro_type: TetrominoType::O,
        rot: Rotation::Left,
    },
];

const S: [Tetromino; ROTS] = [
    Tetromino {
        position: (0, 0),
        blocks: [(1, 0), (2, 0), (0, 1), (1, 1)],
        tetro_type: TetrominoType::S,
        rot: Rotation::Spawn,
    },

    Tetromino {
        position: (0, 0),
        blocks: [(1, 0), (1, 1), (2, 1), (2, 2)],
        tetro_type: TetrominoType::S,
        rot: Rotation::Right,
    },

    Tetromino {
        position: (0, 0),
        blocks: [(1, 1), (2, 1), (0, 2), (1, 2)],
        tetro_type: TetrominoType::S,
        rot: Rotation::Rot2,
    },

    Tetromino {
        position: (0, 0),
        blocks: [(0, 0), (0, 1), (1, 1), (1, 2)],
        tetro_type: TetrominoType::S,
        rot: Rotation::Left,
    },
];

const T: [Tetromino; ROTS] = [
    Tetromino {
        position: (0, 0),
        blocks: [(1, 0), (0, 1), (1, 1), (2, 1)],
        tetro_type: TetrominoType::T,
        rot: Rotation::Spawn,
    },

    Tetromino {
        position: (0, 0),
        blocks: [(1, 0), (1, 1), (2, 1), (1, 2)],
        tetro_type: TetrominoType::T,
        rot: Rotation::Right,
    },

    Tetromino {
        position: (0, 0),
        blocks: [(0, 1), (1, 1), (2, 1), (1, 2)],
        tetro_type: TetrominoType::T,
        rot: Rotation::Rot2,
    },

    Tetromino {
        position: (0, 0),
        blocks: [(1, 0), (0, 1), (1, 1), (1, 2)],
        tetro_type: TetrominoType::T,
        rot: Rotation::Left,
    },
];

const Z: [Tetromino; ROTS] = [
    Tetromino {
        position: (0, 0),
        blocks: [(0, 0), (1, 0), (1, 1), (2, 1)],
        tetro_type: TetrominoType::Z,
        rot: Rotation::Spawn,
    },

    Tetromino {
        position: (0, 0),
        blocks: [(2, 2), (1, 1), (2, 1), (1, 2)],
        tetro_type: TetrominoType::Z,
        rot: Rotation::Right,
    },

    Tetromino {
        position: (0, 0),
        blocks: [(0, 1), (1, 1), (1, 2), (2, 2)],
        tetro_type: TetrominoType::Z,
        rot: Rotation::Rot2,
    },

    Tetromino {
        position: (0, 0),
        blocks: [(1, 0), (0, 1), (1, 1), (0, 2)],
        tetro_type: TetrominoType::Z,
        rot: Rotation::Right,
    },
];