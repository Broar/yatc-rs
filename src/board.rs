extern crate rand;
extern crate rustbox;

use self::rustbox::Color;

use super::tetromino::Block;

pub const WIDTH: usize = 10;
pub const HEIGHT: usize = 24;

/// A struct representing a 10x24 Tetris board
pub struct Board {
	pub blocks: [[Option<Block>; WIDTH]; HEIGHT],
}

impl Board {

	/// Initializes a new Board struct
	pub fn new() -> Self {
		let mut blocks: [[Option<Block>; WIDTH]; HEIGHT] = [[None; WIDTH]; HEIGHT];

		// Temporarily generate random block pieces
		for i in 0..HEIGHT {
			for j in 0..WIDTH {
				if rand::random() {
					blocks[i][j] = Some(Block::new(i, j, get_random_color()));
				}
			}
		}

		Board {
			blocks: blocks,
		}
	}
}

/// Generate a random RustBox color
fn get_random_color() -> Color {
	let color = rand::random::<u8>() % 8;
	match color {
		0 => Color::Default,
		1 => Color::Black,
		2 => Color::Red,
		3 => Color::Green,
		4 => Color::Yellow,
		5 => Color::Blue,
		6 => Color::Magenta,
		7 => Color::Cyan,
		_ => Color::White,
	}
}