use super::board::{HEIGHT, WIDTH};
use super::tetromino::{
	Direction,
    Point,
    Rotation, 
    Tetromino, 
    TetrominoType,
};

const TESTS: usize = 5;
const DIRECTIONS: usize = 2;
const ROTS: usize = 4;

// Wall kick data as specified by the Super Rotation System
const WALL_KICKS: [[[Point; TESTS]; DIRECTIONS]; ROTS] = [SPAWN_WK, RIGHT_WK, ROT2_WK, LEFT_WK];
const WALL_KICKS_I: [[[Point; TESTS]; DIRECTIONS]; ROTS] = [SPAWN_WK_I, RIGHT_WK_I, ROT2_WK_I, LEFT_WK_I];

// The wall kick data for J, L, O, S, T, and Z
const SPAWN_WK: [[Point; TESTS]; DIRECTIONS] = [
    [
        Point { x:  0, y:  0 },
        Point { x: -1, y:  0 },
        Point { x: -1, y: -1 },
        Point { x:  0, y:  2 },
        Point { x: -1, y:  2 }, 
    ], 

    [   
        Point { x:  0, y:  0 },
        Point { x:  1, y:  0 },
        Point { x:  1, y: -1 },
        Point { x:  0, y:  2 },
        Point { x:  1, y:  2 },
    ]
];

const RIGHT_WK: [[Point; TESTS]; DIRECTIONS] = [
    [
        Point { x:  0, y:  0 },
        Point { x:  1, y:  0 },
        Point { x:  1, y:  1 },
        Point { x:  0, y: -2 },
        Point { x:  1, y: -2 }, 
    ], 

    [   
        Point { x:  0, y:  0 },
        Point { x:  1, y:  0 },
        Point { x:  1, y:  1 },
        Point { x:  0, y: -2 },
        Point { x:  1, y: -2 },
    ]
];

const ROT2_WK: [[Point; TESTS]; DIRECTIONS] = [
    [
        Point { x:  0, y:  0 },
        Point { x:  1, y:  0 },
        Point { x:  1, y: -1 },
        Point { x:  0, y:  2 },
        Point { x:  1, y:  2 }, 
    ], 

    [   
        Point { x:  0, y:  0 },
        Point { x: -1, y:  0 },
        Point { x: -1, y: -1 },
        Point { x:  0, y:  2 },
        Point { x: -1, y:  2 },
    ]
];

const LEFT_WK: [[Point; TESTS]; DIRECTIONS] = [
    [
        Point { x:  0, y:  0 },
        Point { x: -1, y:  0 },
        Point { x: -1, y:  1 },
        Point { x:  0, y: -2 },
        Point { x: -1, y: -2 }, 
    ], 

    [   
        Point { x:  0, y:  0 },
        Point { x: -1, y:  0 },
        Point { x: -1, y:  1 },
        Point { x:  0, y:  2 },
        Point { x: -1, y: -2 },
    ]
];

// The wall kick data for I
const SPAWN_WK_I: [[Point; TESTS]; DIRECTIONS] = [
    [
        Point { x:  0, y:  0 },
        Point { x: -2, y:  0 },
        Point { x:  1, y:  0 },
        Point { x: -2, y:  1 },
        Point { x:  1, y: -2 }, 
    ], 

    [   
        Point { x:  0, y:  0 },
        Point { x: -1, y:  0 },
        Point { x:  2, y:  0 },
        Point { x: -1, y: -2 },
        Point { x:  2, y:  1 },
    ]
];

const RIGHT_WK_I: [[Point; TESTS]; DIRECTIONS] = [
    [
        Point { x:  0, y:  0 },
        Point { x: -1, y:  0 },
        Point { x:  2, y:  0 },
        Point { x: -1, y: -2 },
        Point { x:  2, y:  1 }, 
    ], 

    [   
        Point { x:  0, y:  0 },
        Point { x:  2, y:  0 },
        Point { x: -1, y:  0 },
        Point { x:  2, y:  1 },
        Point { x: -1, y:  2 },
    ]
];

const ROT2_WK_I: [[Point; TESTS]; DIRECTIONS] = [
    [
        Point { x:  0, y:  0 },
        Point { x:  2, y:  0 },
        Point { x: -1, y:  0 },
        Point { x:  2, y: -1 },
        Point { x: -1, y:  2 }, 
    ], 

    [   
        Point { x:  0, y:  0 },
        Point { x:  1, y:  0 },
        Point { x: -2, y:  0 },
        Point { x:  1, y:  2 },
        Point { x: -2, y: -1 },
    ]
];

const LEFT_WK_I: [[Point; TESTS]; DIRECTIONS] = [
    [
        Point { x:  0, y:  0 },
        Point { x:  1, y:  0 },
        Point { x: -2, y:  0 },
        Point { x:  1, y:  2 },
        Point { x: -2, y: -1 }, 
    ], 

    [   
        Point { x:  0, y:  0 },
        Point { x: -1, y:  0 },
        Point { x:  2, y:  0 },
        Point { x: -1, y: -2 },
        Point { x:  2, y:  1 },
    ]
];