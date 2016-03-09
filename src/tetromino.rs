extern crate rustbox;

const MINOS: usize = 4;

#[derive(Copy, Clone)]
pub struct Tetromino {
    pub minos: [Point; MINOS],
    pub pos: Point,
    pub tetro_type: TetrominoType,
    pub rot: Rotation,
}

#[derive(Copy, Clone)]
pub struct Point {
    pub x: isize,
    pub y: isize,
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

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub enum Rotation {
    Spawn,
    Right,
    Rot2,
    Left,
}

#[derive(Copy, Clone)]
pub enum Direction {
    Clockwise,
    CounterClockwise,
}

impl Tetromino {
    /// Initializes a new Tetromino struct of a specified type and rotation
    pub fn new(pos: Point, tetro_type: TetrominoType, rot: Rotation) -> Self {
        let mut tetromino = TETROMINOS[tetro_type as usize][rot as usize].clone();
        tetromino.pos = pos;
        tetromino
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

        pos: Point { x: 0, y: 0 },
        tetro_type: TetrominoType::I,
        rot: Rotation::Spawn,
    },

    Tetromino {
        minos: [
            Point { x: 2, y: 0 },
            Point { x: 2, y: 1 },
            Point { x: 2, y: 2 },
            Point { x: 2, y: 3 },
        ],

        pos: Point { x: 0, y: 0 },
        tetro_type: TetrominoType::I,
        rot: Rotation::Right,
    },

    Tetromino {
        minos: [
            Point { x: 0, y: 2 },
            Point { x: 1, y: 2 },
            Point { x: 2, y: 2 },
            Point { x: 3, y: 2 },
        ],

        pos: Point { x: 0, y: 0 },
        tetro_type: TetrominoType::I,
        rot: Rotation::Rot2,
    },

    Tetromino {
        minos: [
            Point { x: 1, y: 0 },
            Point { x: 1, y: 1 },
            Point { x: 1, y: 2 },
            Point { x: 1, y: 3 },
        ],

        pos: Point { x: 0, y: 0 },
        tetro_type: TetrominoType::I,
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

        pos: Point { x: 0, y: 0 },
        tetro_type: TetrominoType::J,
        rot: Rotation::Spawn,
    },

    Tetromino {
        minos: [
            Point { x: 1, y: 0 },
            Point { x: 1, y: 1 },
            Point { x: 1, y: 2 },
            Point { x: 2, y: 0 },
        ],

        pos: Point { x: 0, y: 0 },
        tetro_type: TetrominoType::J,
        rot: Rotation::Right,
    },

    Tetromino {
        minos: [
            Point { x: 0, y: 1 },
            Point { x: 1, y: 1 },
            Point { x: 2, y: 1 },
            Point { x: 2, y: 2 },
        ],

        pos: Point { x: 0, y: 0 },
        tetro_type: TetrominoType::J,
        rot: Rotation::Rot2,
    },

    Tetromino {
        minos: [
            Point { x: 1, y: 0 },
            Point { x: 1, y: 1 },
            Point { x: 1, y: 2 },
            Point { x: 0, y: 2 },
        ],

        pos: Point { x: 0, y: 0 },
        tetro_type: TetrominoType::J,
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

        pos: Point { x: 0, y: 0 },
        tetro_type: TetrominoType::L,
        rot: Rotation::Spawn,
    },

    Tetromino {
        minos: [
            Point { x: 1, y: 0 },
            Point { x: 1, y: 1 },
            Point { x: 1, y: 2 },
            Point { x: 2, y: 2 },
        ],

        pos: Point { x: 0, y: 0 },
        tetro_type: TetrominoType::L,
        rot: Rotation::Right,
    },

    Tetromino {
        minos: [
            Point { x: 0, y: 1 },
            Point { x: 1, y: 1 },
            Point { x: 2, y: 1 },
            Point { x: 0, y: 2 },
        ],

        pos: Point { x: 0, y: 0 },
        tetro_type: TetrominoType::L,
        rot: Rotation::Rot2,
    },

    Tetromino {
        minos: [
            Point { x: 0, y: 0 },
            Point { x: 1, y: 0 },
            Point { x: 1, y: 1 },
            Point { x: 1, y: 2 },
        ],

        pos: Point { x: 0, y: 0 },
        tetro_type: TetrominoType::L,
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

        pos: Point { x: 0, y: 0 },
        tetro_type: TetrominoType::O,
        rot: Rotation::Spawn,
    },

    Tetromino {
        minos: [
            Point { x: 1, y: 0 },
            Point { x: 2, y: 0 },
            Point { x: 1, y: 1 },
            Point { x: 2, y: 1 },            
        ],

        pos: Point { x: 0, y: 0 },
        tetro_type: TetrominoType::O,
        rot: Rotation::Right,
    },

    Tetromino {
        minos: [
            Point { x: 1, y: 0 },
            Point { x: 2, y: 0 },
            Point { x: 1, y: 1 },
            Point { x: 2, y: 1 },
        ],

        pos: Point { x: 0, y: 0 },
        tetro_type: TetrominoType::O,
        rot: Rotation::Rot2,
    },

    Tetromino {
        minos: [
            Point { x: 1, y: 0 },
            Point { x: 2, y: 0 },
            Point { x: 1, y: 1 },
            Point { x: 2, y: 1 },
        ],

        pos: Point { x: 0, y: 0 },
        tetro_type: TetrominoType::O,
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

        pos: Point { x: 0, y: 0 },
        tetro_type: TetrominoType::S,
        rot: Rotation::Spawn,
    },

    Tetromino {
        minos: [
            Point { x: 1, y: 0 },
            Point { x: 1, y: 1 },
            Point { x: 2, y: 1 },
            Point { x: 2, y: 2 },
        ],

        pos: Point { x: 0, y: 0 },
        tetro_type: TetrominoType::S,
        rot: Rotation::Right,
    },

    Tetromino {
        minos: [
            Point { x: 1, y: 1 },
            Point { x: 2, y: 1 },
            Point { x: 0, y: 2 },
            Point { x: 1, y: 2 },
        ],

        pos: Point { x: 0, y: 0 },
        tetro_type: TetrominoType::S,
        rot: Rotation::Rot2,
    },

    Tetromino {
        minos: [
            Point { x: 0, y: 0 },
            Point { x: 0, y: 1 },
            Point { x: 1, y: 1 },
            Point { x: 1, y: 2 },
        ],

        pos: Point { x: 0, y: 0 },
        tetro_type: TetrominoType::S,
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

        pos: Point { x: 0, y: 0 },
        tetro_type: TetrominoType::T,
        rot: Rotation::Spawn,
    },

    Tetromino {
        minos: [
            Point { x: 1, y: 0 },
            Point { x: 1, y: 1 },
            Point { x: 2, y: 1 },
            Point { x: 1, y: 2 },
        ],

        pos: Point { x: 0, y: 0 },
        tetro_type: TetrominoType::T,
        rot: Rotation::Right,
    },

    Tetromino {
        minos: [
            Point { x: 0, y: 1 },
            Point { x: 1, y: 1 },
            Point { x: 2, y: 1 },
            Point { x: 1, y: 2 },
        ],

        pos: Point { x: 0, y: 0 },
        tetro_type: TetrominoType::T,
        rot: Rotation::Rot2,
    },

    Tetromino {
        minos: [
            Point { x: 1, y: 0 },
            Point { x: 0, y: 1 },
            Point { x: 1, y: 1 },
            Point { x: 1, y: 2 },
        ],

        pos: Point { x: 0, y: 0 },
        tetro_type: TetrominoType::T,
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

        pos: Point { x: 0, y: 0 },
        tetro_type: TetrominoType::Z,
        rot: Rotation::Spawn,
    },

    Tetromino {
        minos: [
            Point { x: 2, y: 0 },
            Point { x: 1, y: 1 },
            Point { x: 2, y: 1 },
            Point { x: 1, y: 2 },
        ],

        pos: Point { x: 0, y: 0 },
        tetro_type: TetrominoType::Z,
        rot: Rotation::Right,
    },

    Tetromino {
        minos: [
            Point { x: 0, y: 1 },
            Point { x: 1, y: 1 },
            Point { x: 1, y: 2 },
            Point { x: 2, y: 2 },
        ],

        pos: Point { x: 0, y: 0 },
        tetro_type: TetrominoType::Z,
        rot: Rotation::Rot2,
    },

    Tetromino {
        minos: [
            Point { x: 1, y: 0 },
            Point { x: 0, y: 1 },
            Point { x: 1, y: 1 },
            Point { x: 0, y: 2 },
        ],

        pos: Point { x: 0, y: 0 },
        tetro_type: TetrominoType::Z,
        rot: Rotation::Left,
    },
];