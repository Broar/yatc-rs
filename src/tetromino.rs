extern crate rustbox;

use std::ops::Add;

const MINOS: usize = 4;

#[derive(Copy, Clone, Debug)]
pub struct Tetromino {
    minos: [Point; MINOS],
    origin: Point,
    tetromino_type: TetrominoType,
    rot: Rotation,
}

#[derive(Copy, Clone, Debug)]
pub struct Point {
    pub x: isize,
    pub y: isize,
}

#[derive(Copy, Clone, Debug)]
pub enum TetrominoType {
    I,
    J,
    L,
    O,
    S,
    T,
    Z,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum Rotation {
    Spawn,
    Right,
    Rot2,
    Left,
}

impl Tetromino {

    /// Initializes a new Tetromino struct of a specified type and rotation
    pub fn new(origin: Point, tetromino_type: TetrominoType, rot: Rotation) -> Self {
        let mut tetromino = TETROMINOS[tetromino_type as usize][rot as usize].clone();
        tetromino.origin = origin;
        tetromino
    }

    // GETTERS / SETTERS

    pub fn minos(&self) -> [Point; MINOS] {
        self.minos
    }

    pub fn origin(&self) -> Point {
        self.origin
    }

    pub fn set_origin(&mut self, origin: Point) {
        self.origin = origin;
    }

    pub fn tetromino_type(&self) -> TetrominoType {
        self.tetromino_type
    }

    pub fn rot(&self) -> Rotation {
        self.rot
    }
}

impl Point {
    
    /// Initializes a new Point struct
    pub fn new(x: isize, y: isize) -> Self {
        Point {
            x: x,
            y: y,
        }
    }
}

// Overload '+' to simplify addition between Points
impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y
        }
    }
}

pub const TYPES: usize = 7;
pub const ROTS: usize = 4;
const TETROMINOS: [[Tetromino; ROTS]; TYPES] = [I, J, L, O, S, T, Z];

const I: [Tetromino; ROTS] = [
    Tetromino {
        minos: [
            Point { x: 0, y: 1 },
            Point { x: 1, y: 1 },
            Point { x: 2, y: 1 },
            Point { x: 3, y: 1 },
        ],

        origin: Point { x: 0, y: 0 },
        tetromino_type: TetrominoType::I,
        rot: Rotation::Spawn,
    },

    Tetromino {
        minos: [
            Point { x: 2, y: 0 },
            Point { x: 2, y: 1 },
            Point { x: 2, y: 2 },
            Point { x: 2, y: 3 },
        ],

        origin: Point { x: 0, y: 0 },
        tetromino_type: TetrominoType::I,
        rot: Rotation::Right,
    },

    Tetromino {
        minos: [
            Point { x: 0, y: 2 },
            Point { x: 1, y: 2 },
            Point { x: 2, y: 2 },
            Point { x: 3, y: 2 },
        ],

        origin: Point { x: 0, y: 0 },
        tetromino_type: TetrominoType::I,
        rot: Rotation::Rot2,
    },

    Tetromino {
        minos: [
            Point { x: 1, y: 0 },
            Point { x: 1, y: 1 },
            Point { x: 1, y: 2 },
            Point { x: 1, y: 3 },
        ],

        origin: Point { x: 0, y: 0 },
        tetromino_type: TetrominoType::I,
        rot: Rotation::Left,
    },
];

const J: [Tetromino; ROTS] = [
    Tetromino {
        minos: [
            Point { x: 0, y: 0 },
            Point { x: 0, y: 1 },
            Point { x: 1, y: 1 },
            Point { x: 2, y: 1 },
        ],

        origin: Point { x: 0, y: 0 },
        tetromino_type: TetrominoType::J,
        rot: Rotation::Spawn,
    },

    Tetromino {
        minos: [
            Point { x: 1, y: 0 },
            Point { x: 1, y: 1 },
            Point { x: 1, y: 2 },
            Point { x: 2, y: 0 },
        ],

        origin: Point { x: 0, y: 0 },
        tetromino_type: TetrominoType::J,
        rot: Rotation::Right,
    },

    Tetromino {
        minos: [
            Point { x: 0, y: 1 },
            Point { x: 1, y: 1 },
            Point { x: 2, y: 1 },
            Point { x: 2, y: 2 },
        ],

        origin: Point { x: 0, y: 0 },
        tetromino_type: TetrominoType::J,
        rot: Rotation::Rot2,
    },

    Tetromino {
        minos: [
            Point { x: 1, y: 0 },
            Point { x: 1, y: 1 },
            Point { x: 0, y: 2 },
            Point { x: 1, y: 2 },
        ],

        origin: Point { x: 0, y: 0 },
        tetromino_type: TetrominoType::J,
        rot: Rotation::Left,
    },
];

const L: [Tetromino; ROTS] = [
    Tetromino {
        minos: [
            Point { x: 2, y: 0 },
            Point { x: 0, y: 1 },
            Point { x: 1, y: 1 },
            Point { x: 2, y: 1 },
        ],

        origin: Point { x: 0, y: 0 },
        tetromino_type: TetrominoType::L,
        rot: Rotation::Spawn,
    },

    Tetromino {
        minos: [
            Point { x: 1, y: 0 },
            Point { x: 1, y: 1 },
            Point { x: 1, y: 2 },
            Point { x: 2, y: 2 },
        ],

        origin: Point { x: 0, y: 0 },
        tetromino_type: TetrominoType::L,
        rot: Rotation::Right,
    },

    Tetromino {
        minos: [
            Point { x: 0, y: 1 },
            Point { x: 1, y: 1 },
            Point { x: 2, y: 1 },
            Point { x: 0, y: 2 },
        ],

        origin: Point { x: 0, y: 0 },
        tetromino_type: TetrominoType::L,
        rot: Rotation::Rot2,
    },

    Tetromino {
        minos: [
            Point { x: 0, y: 0 },
            Point { x: 1, y: 0 },
            Point { x: 1, y: 1 },
            Point { x: 1, y: 2 },
        ],

        origin: Point { x: 0, y: 0 },
        tetromino_type: TetrominoType::L,
        rot: Rotation::Left,
    },
];

const O: [Tetromino; ROTS] = [
    Tetromino {
        minos: [
            Point { x: 1, y: 0 },
            Point { x: 2, y: 0 },
            Point { x: 1, y: 1 },
            Point { x: 2, y: 1 },
        ],

        origin: Point { x: 0, y: 0 },
        tetromino_type: TetrominoType::O,
        rot: Rotation::Spawn,
    },

    Tetromino {
        minos: [
            Point { x: 1, y: 0 },
            Point { x: 2, y: 0 },
            Point { x: 1, y: 1 },
            Point { x: 2, y: 1 },            
        ],

        origin: Point { x: 0, y: 0 },
        tetromino_type: TetrominoType::O,
        rot: Rotation::Right,
    },

    Tetromino {
        minos: [
            Point { x: 1, y: 0 },
            Point { x: 2, y: 0 },
            Point { x: 1, y: 1 },
            Point { x: 2, y: 1 },
        ],

        origin: Point { x: 0, y: 0 },
        tetromino_type: TetrominoType::O,
        rot: Rotation::Rot2,
    },

    Tetromino {
        minos: [
            Point { x: 1, y: 0 },
            Point { x: 2, y: 0 },
            Point { x: 1, y: 1 },
            Point { x: 2, y: 1 },
        ],

        origin: Point { x: 0, y: 0 },
        tetromino_type: TetrominoType::O,
        rot: Rotation::Left,
    },
];

const S: [Tetromino; ROTS] = [
    Tetromino {
        minos: [
            Point { x: 1, y: 0 },
            Point { x: 2, y: 0 },
            Point { x: 0, y: 1 },
            Point { x: 1, y: 1 },
        ],

        origin: Point { x: 0, y: 0 },
        tetromino_type: TetrominoType::S,
        rot: Rotation::Spawn,
    },

    Tetromino {
        minos: [
            Point { x: 1, y: 0 },
            Point { x: 1, y: 1 },
            Point { x: 2, y: 1 },
            Point { x: 2, y: 2 },
        ],

        origin: Point { x: 0, y: 0 },
        tetromino_type: TetrominoType::S,
        rot: Rotation::Right,
    },

    Tetromino {
        minos: [
            Point { x: 1, y: 1 },
            Point { x: 2, y: 1 },
            Point { x: 0, y: 2 },
            Point { x: 1, y: 2 },
        ],

        origin: Point { x: 0, y: 0 },
        tetromino_type: TetrominoType::S,
        rot: Rotation::Rot2,
    },

    Tetromino {
        minos: [
            Point { x: 0, y: 0 },
            Point { x: 0, y: 1 },
            Point { x: 1, y: 1 },
            Point { x: 1, y: 2 },
        ],

        origin: Point { x: 0, y: 0 },
        tetromino_type: TetrominoType::S,
        rot: Rotation::Left,
    },
];

const T: [Tetromino; ROTS] = [
    Tetromino {
        minos: [
            Point { x: 1, y: 0 },
            Point { x: 0, y: 1 },
            Point { x: 1, y: 1 },
            Point { x: 2, y: 1 },
        ],

        origin: Point { x: 0, y: 0 },
        tetromino_type: TetrominoType::T,
        rot: Rotation::Spawn,
    },

    Tetromino {
        minos: [
            Point { x: 1, y: 0 },
            Point { x: 1, y: 1 },
            Point { x: 2, y: 1 },
            Point { x: 1, y: 2 },
        ],

        origin: Point { x: 0, y: 0 },
        tetromino_type: TetrominoType::T,
        rot: Rotation::Right,
    },

    Tetromino {
        minos: [
            Point { x: 0, y: 1 },
            Point { x: 1, y: 1 },
            Point { x: 2, y: 1 },
            Point { x: 1, y: 2 },
        ],

        origin: Point { x: 0, y: 0 },
        tetromino_type: TetrominoType::T,
        rot: Rotation::Rot2,
    },

    Tetromino {
        minos: [
            Point { x: 1, y: 0 },
            Point { x: 0, y: 1 },
            Point { x: 1, y: 1 },
            Point { x: 1, y: 2 },
        ],

        origin: Point { x: 0, y: 0 },
        tetromino_type: TetrominoType::T,
        rot: Rotation::Left,
    },
];

const Z: [Tetromino; ROTS] = [
    Tetromino {
        minos: [
            Point { x: 0, y: 0 },
            Point { x: 1, y: 0 },
            Point { x: 1, y: 1 },
            Point { x: 2, y: 1 },
        ],

        origin: Point { x: 0, y: 0 },
        tetromino_type: TetrominoType::Z,
        rot: Rotation::Spawn,
    },

    Tetromino {
        minos: [
            Point { x: 2, y: 0 },
            Point { x: 1, y: 1 },
            Point { x: 2, y: 1 },
            Point { x: 1, y: 2 },
        ],

        origin: Point { x: 0, y: 0 },
        tetromino_type: TetrominoType::Z,
        rot: Rotation::Right,
    },

    Tetromino {
        minos: [
            Point { x: 0, y: 1 },
            Point { x: 1, y: 1 },
            Point { x: 1, y: 2 },
            Point { x: 2, y: 2 },
        ],

        origin: Point { x: 0, y: 0 },
        tetromino_type: TetrominoType::Z,
        rot: Rotation::Rot2,
    },

    Tetromino {
        minos: [
            Point { x: 1, y: 0 },
            Point { x: 0, y: 1 },
            Point { x: 1, y: 1 },
            Point { x: 0, y: 2 },
        ],

        origin: Point { x: 0, y: 0 },
        tetromino_type: TetrominoType::Z,
        rot: Rotation::Left,
    },
];