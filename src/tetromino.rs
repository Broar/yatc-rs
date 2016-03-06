extern crate rustbox;

const MINOS: usize = 4;

#[derive(Copy, Clone)]
pub struct Tetromino {
    pub minos: [Mino; MINOS],
}

#[derive(Copy, Clone)]
pub struct Mino {
    pub pos: Point,
    pub tetro_type: TetrominoType,
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

#[derive(Copy, Clone)]
pub enum Rotation {
    Spawn,
    Right,
    Rot2,
    Left,
}

impl Tetromino {
    /// Initializes a new Tetromino struct of a specified type and rotation
    pub fn new(tetro_type: TetrominoType, rot: Rotation) -> Self {
        TETROMINOS[tetro_type as usize][rot as usize].clone()
    }
}

impl Mino {
    /// Initializes a new Mino struct
    pub fn new(x: isize, y: isize, tetro_type: TetrominoType) -> Self {
        Mino {
            pos: Point::new(x, y),
            tetro_type: tetro_type,
        }
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
            Mino { pos: Point { x: 0, y: 1 }, tetro_type: TetrominoType::I, },
            Mino { pos: Point { x: 1, y: 1 }, tetro_type: TetrominoType::I, },
            Mino { pos: Point { x: 2, y: 1 }, tetro_type: TetrominoType::I, },
            Mino { pos: Point { x: 3, y: 1 }, tetro_type: TetrominoType::I, },
        ],
    },

    Tetromino {
        minos: [
            Mino { pos: Point { x: 2, y: 0 }, tetro_type: TetrominoType::I, },
            Mino { pos: Point { x: 2, y: 1 }, tetro_type: TetrominoType::I, },
            Mino { pos: Point { x: 2, y: 2 }, tetro_type: TetrominoType::I, },
            Mino { pos: Point { x: 2, y: 3 }, tetro_type: TetrominoType::I, },
        ],
    },

    Tetromino {
        minos: [
            Mino { pos: Point { x: 0, y: 2 }, tetro_type: TetrominoType::I, },
            Mino { pos: Point { x: 1, y: 2 }, tetro_type: TetrominoType::I, },
            Mino { pos: Point { x: 2, y: 2 }, tetro_type: TetrominoType::I, },
            Mino { pos: Point { x: 3, y: 2 }, tetro_type: TetrominoType::I, },
        ],
    },

    Tetromino {
        minos: [
            Mino { pos: Point { x: 0, y: 2 }, tetro_type: TetrominoType::I, },
            Mino { pos: Point { x: 1, y: 2 }, tetro_type: TetrominoType::I, },
            Mino { pos: Point { x: 2, y: 2 }, tetro_type: TetrominoType::I, },
            Mino { pos: Point { x: 3, y: 2 }, tetro_type: TetrominoType::I, },
        ],
    },
];

const J: [Tetromino; ROTS] = [
    Tetromino {
        minos: [
            Mino { pos: Point { x: 0, y: 0 }, tetro_type: TetrominoType::J, },
            Mino { pos: Point { x: 0, y: 1 }, tetro_type: TetrominoType::J, },
            Mino { pos: Point { x: 1, y: 1 }, tetro_type: TetrominoType::J, },
            Mino { pos: Point { x: 2, y: 1 }, tetro_type: TetrominoType::J, },
        ],
    },

    Tetromino {
        minos: [
            Mino { pos: Point { x: 1, y: 0 }, tetro_type: TetrominoType::J, },
            Mino { pos: Point { x: 1, y: 1 }, tetro_type: TetrominoType::J, },
            Mino { pos: Point { x: 1, y: 2 }, tetro_type: TetrominoType::J, },
            Mino { pos: Point { x: 2, y: 0 }, tetro_type: TetrominoType::J, },
        ],
    },

    Tetromino {
        minos: [
            Mino { pos: Point { x: 0, y: 1 }, tetro_type: TetrominoType::J, },
            Mino { pos: Point { x: 1, y: 1 }, tetro_type: TetrominoType::J, },
            Mino { pos: Point { x: 2, y: 1 }, tetro_type: TetrominoType::J, },
            Mino { pos: Point { x: 2, y: 2 }, tetro_type: TetrominoType::J, },
        ],
    },

    Tetromino {
        minos: [
            Mino { pos: Point { x: 1, y: 0 }, tetro_type: TetrominoType::J, },
            Mino { pos: Point { x: 1, y: 1 }, tetro_type: TetrominoType::J, },
            Mino { pos: Point { x: 1, y: 2 }, tetro_type: TetrominoType::J, },
            Mino { pos: Point { x: 0, y: 2 }, tetro_type: TetrominoType::J, },
        ],
    },
];

const L: [Tetromino; ROTS] = [
    Tetromino {
        minos: [
            Mino { pos: Point { x: 2, y: 0 }, tetro_type: TetrominoType::L, },
            Mino { pos: Point { x: 0, y: 1 }, tetro_type: TetrominoType::L, },
            Mino { pos: Point { x: 1, y: 1 }, tetro_type: TetrominoType::L, },
            Mino { pos: Point { x: 2, y: 1 }, tetro_type: TetrominoType::L, },
        ],
    },

    Tetromino {
        minos: [
            Mino { pos: Point { x: 1, y: 0 }, tetro_type: TetrominoType::L, },
            Mino { pos: Point { x: 1, y: 1 }, tetro_type: TetrominoType::L, },
            Mino { pos: Point { x: 1, y: 2 }, tetro_type: TetrominoType::L, },
            Mino { pos: Point { x: 2, y: 2 }, tetro_type: TetrominoType::L, },
        ],
    },

    Tetromino {
        minos: [
            Mino { pos: Point { x: 0, y: 1 }, tetro_type: TetrominoType::L, },
            Mino { pos: Point { x: 1, y: 1 }, tetro_type: TetrominoType::L, },
            Mino { pos: Point { x: 2, y: 1 }, tetro_type: TetrominoType::L, },
            Mino { pos: Point { x: 0, y: 2 }, tetro_type: TetrominoType::L, },
        ],
    },

    Tetromino {
        minos: [
            Mino { pos: Point { x: 0, y: 0 }, tetro_type: TetrominoType::L, },
            Mino { pos: Point { x: 1, y: 0 }, tetro_type: TetrominoType::L, },
            Mino { pos: Point { x: 1, y: 1 }, tetro_type: TetrominoType::L, },
            Mino { pos: Point { x: 1, y: 2 }, tetro_type: TetrominoType::L, },
        ],
    },
];

const O: [Tetromino; ROTS] = [
    Tetromino {
        minos: [
            Mino { pos: Point { x: 1, y: 0 }, tetro_type: TetrominoType::O, },
            Mino { pos: Point { x: 2, y: 0 }, tetro_type: TetrominoType::O, },
            Mino { pos: Point { x: 1, y: 1 }, tetro_type: TetrominoType::O, },
            Mino { pos: Point { x: 2, y: 1 }, tetro_type: TetrominoType::O, },
        ],
    },

    Tetromino {
        minos: [
            Mino { pos: Point { x: 1, y: 0 }, tetro_type: TetrominoType::O, },
            Mino { pos: Point { x: 2, y: 0 }, tetro_type: TetrominoType::O, },
            Mino { pos: Point { x: 1, y: 1 }, tetro_type: TetrominoType::O, },
            Mino { pos: Point { x: 2, y: 1 }, tetro_type: TetrominoType::O, },            
        ],
    },

    Tetromino {
        minos: [
            Mino { pos: Point { x: 1, y: 0 }, tetro_type: TetrominoType::O, },
            Mino { pos: Point { x: 2, y: 0 }, tetro_type: TetrominoType::O, },
            Mino { pos: Point { x: 1, y: 1 }, tetro_type: TetrominoType::O, },
            Mino { pos: Point { x: 2, y: 1 }, tetro_type: TetrominoType::O, },
        ],
    },

    Tetromino {
        minos: [
            Mino { pos: Point { x: 1, y: 0 }, tetro_type: TetrominoType::O, },
            Mino { pos: Point { x: 2, y: 0 }, tetro_type: TetrominoType::O, },
            Mino { pos: Point { x: 1, y: 1 }, tetro_type: TetrominoType::O, },
            Mino { pos: Point { x: 2, y: 1 }, tetro_type: TetrominoType::O, },
        ],
    },
];

const S: [Tetromino; ROTS] = [
    Tetromino {
        minos: [
            Mino { pos: Point { x: 1, y: 0 }, tetro_type: TetrominoType::S, },
            Mino { pos: Point { x: 2, y: 0 }, tetro_type: TetrominoType::S, },
            Mino { pos: Point { x: 0, y: 1 }, tetro_type: TetrominoType::S, },
            Mino { pos: Point { x: 1, y: 1 }, tetro_type: TetrominoType::S, },
        ],
    },

    Tetromino {
        minos: [
            Mino { pos: Point { x: 1, y: 0 }, tetro_type: TetrominoType::S, },
            Mino { pos: Point { x: 1, y: 1 }, tetro_type: TetrominoType::S, },
            Mino { pos: Point { x: 2, y: 1 }, tetro_type: TetrominoType::S, },
            Mino { pos: Point { x: 2, y: 2 }, tetro_type: TetrominoType::S, },
        ],
    },

    Tetromino {
        minos: [
            Mino { pos: Point { x: 1, y: 1 }, tetro_type: TetrominoType::S, },
            Mino { pos: Point { x: 2, y: 1 }, tetro_type: TetrominoType::S, },
            Mino { pos: Point { x: 0, y: 2 }, tetro_type: TetrominoType::S, },
            Mino { pos: Point { x: 1, y: 2 }, tetro_type: TetrominoType::S, },
        ],
    },

    Tetromino {
        minos: [
            Mino { pos: Point { x: 0, y: 0 }, tetro_type: TetrominoType::S, },
            Mino { pos: Point { x: 0, y: 1 }, tetro_type: TetrominoType::S, },
            Mino { pos: Point { x: 1, y: 1 }, tetro_type: TetrominoType::S, },
            Mino { pos: Point { x: 1, y: 2 }, tetro_type: TetrominoType::S, },
        ],
    },
];

const T: [Tetromino; ROTS] = [
    Tetromino {
        minos: [
            Mino { pos: Point { x: 1, y: 0 }, tetro_type: TetrominoType::T, },
            Mino { pos: Point { x: 0, y: 1 }, tetro_type: TetrominoType::T, },
            Mino { pos: Point { x: 1, y: 1 }, tetro_type: TetrominoType::T, },
            Mino { pos: Point { x: 2, y: 1 }, tetro_type: TetrominoType::T, },
        ],
    },

    Tetromino {
        minos: [
            Mino { pos: Point { x: 1, y: 0 }, tetro_type: TetrominoType::T, },
            Mino { pos: Point { x: 1, y: 1 }, tetro_type: TetrominoType::T, },
            Mino { pos: Point { x: 2, y: 1 }, tetro_type: TetrominoType::T, },
            Mino { pos: Point { x: 1, y: 2 }, tetro_type: TetrominoType::T, },
        ],
    },

    Tetromino {
        minos: [
            Mino { pos: Point { x: 0, y: 1 }, tetro_type: TetrominoType::T, },
            Mino { pos: Point { x: 1, y: 1 }, tetro_type: TetrominoType::T, },
            Mino { pos: Point { x: 2, y: 1 }, tetro_type: TetrominoType::T, },
            Mino { pos: Point { x: 1, y: 2 }, tetro_type: TetrominoType::T, },
        ],
    },

    Tetromino {
        minos: [
            Mino { pos: Point { x: 1, y: 0 }, tetro_type: TetrominoType::T, },
            Mino { pos: Point { x: 0, y: 1 }, tetro_type: TetrominoType::T, },
            Mino { pos: Point { x: 1, y: 1 }, tetro_type: TetrominoType::T, },
            Mino { pos: Point { x: 1, y: 2 }, tetro_type: TetrominoType::T, },
        ],
    },
];

const Z: [Tetromino; ROTS] = [
    Tetromino {
        minos: [
            Mino { pos: Point { x: 0, y: 0 }, tetro_type: TetrominoType::Z, },
            Mino { pos: Point { x: 1, y: 0 }, tetro_type: TetrominoType::Z, },
            Mino { pos: Point { x: 1, y: 1 }, tetro_type: TetrominoType::Z, },
            Mino { pos: Point { x: 2, y: 1 }, tetro_type: TetrominoType::Z, },
        ],
    },

    Tetromino {
        minos: [
            Mino { pos: Point { x: 2, y: 2 }, tetro_type: TetrominoType::Z, },
            Mino { pos: Point { x: 1, y: 1 }, tetro_type: TetrominoType::Z, },
            Mino { pos: Point { x: 2, y: 1 }, tetro_type: TetrominoType::Z, },
            Mino { pos: Point { x: 1, y: 2 }, tetro_type: TetrominoType::Z, },
        ],
    },

    Tetromino {
        minos: [
            Mino { pos: Point { x: 0, y: 1 }, tetro_type: TetrominoType::Z, },
            Mino { pos: Point { x: 1, y: 1 }, tetro_type: TetrominoType::Z, },
            Mino { pos: Point { x: 1, y: 2 }, tetro_type: TetrominoType::Z, },
            Mino { pos: Point { x: 2, y: 2 }, tetro_type: TetrominoType::Z, },
        ],
    },

    Tetromino {
        minos: [
            Mino { pos: Point { x: 1, y: 0 }, tetro_type: TetrominoType::Z, },
            Mino { pos: Point { x: 0, y: 1 }, tetro_type: TetrominoType::Z, },
            Mino { pos: Point { x: 1, y: 1 }, tetro_type: TetrominoType::Z, },
            Mino { pos: Point { x: 0, y: 2 }, tetro_type: TetrominoType::Z, },
        ],
    },
];