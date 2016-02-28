extern crate rustbox;

use self::rustbox::Color;

#[derive(Copy, Clone)]
/// A block piece of a tetromino
pub struct Block {
    x: usize,
    y: usize,
    pub color: Color,
}

impl Block {
    
    /// Initializes a new Block struct
    pub fn new(x: usize, y: usize, color: Color) -> Self {
        Block {
            x: x,
            y: y,
            color: color,
        }
    }
}