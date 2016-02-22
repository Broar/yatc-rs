extern crate rustbox;

use self::rustbox::{Color, Style, RustBox};

/// A drawable area and element of the screen
/// The origin of a Window is (0, 0) and is located at the top left. All
/// drawing operations must be done relative to this coordinate position
pub struct Window {
    x: usize,
    y: usize,
    w: usize,
    h: usize,
}

impl Window {

	/// Initializes a new Window struct at an (x, y) position with a specified width and height
	pub fn new(x: usize, y: usize, w: usize, h: usize) -> Self {
		Window {
			x: x,
			y: y,
			w: w,
			h: h,
		}
	}

    /// Prints a character at an (x, y) position relative to the origin of the Window
	pub fn print_char(&self, rb: &RustBox, x: usize, y: usize, style: Style, fg: Color, bg: Color, c: char) {
		// x and y are unsigned, so we do not need to check for negative values
		if x <= self.w && y <= self.h {
		    rb.print_char(self.x + x, self.y + y, style, fg, bg, c);
		}
	}

    /// Prints a string at an (x, y) position relative to the origin of the Window
	pub fn print(&self, rb: &RustBox, x: usize, y: usize, style: Style, fg: Color, bg: Color, s: &str) {
		// x and y are unsigned, so we do not need to check for negative values
		if x + s.len() <= self.w && y <= self.h {
		    rb.print(self.x + x, self.y + y, style, fg, bg, s);
		}
	}

    /// Convenience method to print a border around the area of the Window
	pub fn print_borders(&self, rb: &RustBox, style: Style, fg: Color, bg: Color) {

		// Draw the top border
		self.print_char(rb, 0, 0, style, fg, bg, '┌');

		for i in 1..self.w {
			self.print_char(rb, i, 0, style, fg, bg, '─');
		}

		self.print_char(rb, self.w, 0, style, fg, bg, '┐');

        // Draw the side borders
		for i in 1..self.h {
			self.print_char(rb, 0, i, style, fg, bg, '│');
			self.print_char(rb, self.w, i, style, fg, bg, '│');
		}

		// Draw the bottom border
		self.print_char(rb, 0, self.h, style, fg, bg, '└');

		for i in 1..self.w {
			self.print_char(rb, i, self.h, style, fg, bg, '─');
		}

		self.print_char(rb, self.w, self.h, style, fg, bg, '┘');
	}
}