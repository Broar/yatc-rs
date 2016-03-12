use super::board::{Field, HEIGHT, WIDTH};
use super::tetromino::{
    Direction,
    Point,
    Rotation, 
    Tetromino, 
    TetrominoType,
};

/// Rotates a Tetromino located inside a field in a specified direction
pub fn rotate(field: &Field, tetromino: &Tetromino, dir: Direction) -> Option<(Field, Tetromino)> {
    let mut field = field.clone();

    // Remove the original piece from the field. Since we cloned the field,
    // we are able to freely manipulate it without worry. Assuming the
    // rotation is possible, this will make performing it easier later on
    erase(&mut field, &tetromino);

    // Rotate the piece if possible
    match is_rotatable(&field, &tetromino, dir) {
        Some(rotated) => {
            do_rotation(&mut field, &rotated);
            Some((field, rotated))
        },

        None => None,
    }
}

/// Erases a Tetromino from a field
fn erase(field: &mut Field, tetromino: &Tetromino) {
    for &mino in tetromino.minos.iter() {
        let pos = Point::new(tetromino.pos.x + mino.x, tetromino.pos.y + mino.y);
        field[pos.y as usize][pos.x as usize] = None;
    }
}

/// Determines if a Tetromino is rotatable within a specified field. Returns the rotated
/// Tetromino if the rotation is possible
fn is_rotatable(field: &Field, tetromino: &Tetromino, dir: Direction) -> Option<Tetromino> {
    let rotated = next_rotated_tetromino(tetromino, dir);

    for &mino in rotated.minos.iter() {
        let pos = Point::new(tetromino.pos.x + mino.x, tetromino.pos.y + mino.y);

        // Determine if it is possible for the rotated mino to move to the new position.
        // If the new position is outside the bounds or is already occupied, then it is
        // not possible and the entire rotation fails
        if pos.x < 0 || pos.y < 0 || pos.x as usize >= WIDTH  || pos.y as usize >= HEIGHT
                || field[pos.y as usize][pos.x as usize].is_some() {
            return None;
        }
    }

    Some(rotated)
}

/// Performs the actual rotation
fn do_rotation(field: &mut Field, rotated: &Tetromino) {
    for &mino in rotated.minos.iter() {
        let pos = Point::new(rotated.pos.x + mino.x, rotated.pos.y + mino.y);
        field[pos.y as usize][pos.x as usize] = Some(rotated.tetro_type.clone());
    }
}

/// Gets the next rotation of a Tetromino in a specified direction
fn next_rotated_tetromino(tetromino: &Tetromino, dir: Direction) -> Tetromino {
    let rot = match dir {
        Direction::Clockwise => {
            match tetromino.rot {
                Rotation::Spawn => Rotation::Right,
                Rotation::Right => Rotation::Rot2,
                Rotation::Rot2  => Rotation::Left,
                Rotation::Left  => Rotation::Spawn,
            }
        },

        Direction::CounterClockwise => {
            match tetromino.rot {
                Rotation::Spawn => Rotation::Left,
                Rotation::Right => Rotation::Spawn,
                Rotation::Rot2  => Rotation::Right,
                Rotation::Left  => Rotation::Rot2,
            }
        }
    };

    Tetromino::new(tetromino.pos, tetromino.tetro_type, rot)
}

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