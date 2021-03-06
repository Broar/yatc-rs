extern crate rustbox;

use self::rustbox::{Color, Style, RustBox};

/// A drawable area and element of the screen.
///
/// The origin of a Window is (0, 0) and is located at the top left. All
/// drawing operations are done relative to this coordinate position
pub struct Window<'a> {
    pub x: usize,
    pub y: usize,
    pub w: usize,
    pub h: usize,
    rb: &'a RustBox,
}

impl<'a> Window<'a> {

    /// Initializes a new Window struct at an (x, y) position with a specified width and height
    pub fn new(x: usize, y: usize, w: usize, h: usize, rb: &'a RustBox) -> Self {
        Window {
            x: x,
            y: y,
            w: w,
            h: h,
            rb: rb,
        }
    }

    /// Prints a character at an (x, y) position
    pub fn print_char(&self, x: usize, y: usize, style: Style, fg: Color, bg: Color, c: char) {
        if x <= self.w && y <= self.h {
            self.rb.print_char(self.x + x, self.y + y, style, fg, bg, c);
        }
    }

    /// Prints a string at an (x, y) position
    pub fn print(&self, x: usize, y: usize, style: Style, fg: Color, bg: Color, s: &str) {
        if x + s.len() <= self.w && y <= self.h {
            self.rb.print(self.x + x, self.y + y, style, fg, bg, s);
        }
    }

    /// Erases a character at an (x, y) position
    pub fn erase(&self, x: usize, y: usize) {
        self.print_char(x, y, rustbox::RB_NORMAL, Color::Black, Color::Black, ' ');
    }

    /// Erases the area of a Window
    pub fn clear(&self) {
        for i in 0..self.w {
            for j in 0..self.h {
                self.erase(i, j);
            }
        }
    }

    /// Convenience method to print a border around the area of the Window
    pub fn print_borders(&self, style: Style, fg: Color, bg: Color) {

        // Draw the top border
        self.print_char(0, 0, style, fg, bg, '┌');

        for i in 1..self.w {
            self.print_char(i, 0, style, fg, bg, '─');
        }

        self.print_char(self.w, 0, style, fg, bg, '┐');

        // Draw the side borders
        for i in 1..self.h {
            self.print_char(0, i, style, fg, bg, '│');
            self.print_char(self.w, i, style, fg, bg, '│');
        }

        // Draw the bottom border
        self.print_char(0, self.h, style, fg, bg, '└');

        for i in 1..self.w {
            self.print_char(i, self.h, style, fg, bg, '─');
        }

        self.print_char(self.w, self.h, style, fg, bg, '┘');
    }
}