extern crate rustbox;

use self::rustbox::{Color, Style, RustBox};

use super::window::Window;
use super::board;
use super::board::Board;

/// A collection of Window structs representing the user interface
pub struct Ui {
    board: Window,
    score: Window,
    level: Window,
    lines: Window,
    next_piece: Window,
}

impl Ui {

    /// Initializes a new Ui struct
    pub fn new() -> Self {
        Ui {
            board: Window::new(0, 0, 11, 22),
            score: Window::new(12, 0, 11, 2),
            level: Window::new(12, 4, 11, 2),
            lines: Window::new(12, 8, 11, 2),
            next_piece: Window::new(12, 12, 11, 6)
        }
    }

    /// Setup the default elements of the user interface 
    pub fn setup(&self, rb: &RustBox) {
        let style = rustbox::RB_NORMAL;
        let fg = Color::White;
        let bg = Color::Black;

        // Print out the borders for the player stats
        self.board.print_borders(rb, style, fg, bg);
        self.score.print_borders(rb, style, fg, bg);
        self.level.print_borders(rb, style, fg, bg);
        self.lines.print_borders(rb, style, fg, bg);
        self.next_piece.print_borders(rb, style, fg, bg);

        // Print out the initial values for the player stats
        self.update_score(rb, 0, style, fg, bg);
        self.update_level(rb, 0, style, fg, bg);
        self.update_lines(rb, 0, style, fg, bg);
    }

    /// Update the board presented on the screen
    pub fn update_board(&self, rb: &RustBox, board: &Board) {
        let height = board::HEIGHT - 3;
        let width = board::WIDTH;

        for y in 0..height {
            for x in 0..width {
                match board.blocks[y][x] {
                    Some(ref block) => self.board.print_char(rb, x + 1, y + 1, rustbox::RB_NORMAL, block.color, Color::Black, '■'),
                    None => { },
                }
            }
        }
    }

    /// Update the score display to a specified value
    pub fn update_score(&self, rb: &RustBox, score: u32, style: Style, fg: Color, bg: Color) {
        self.score.print(rb, 1, 1, style, fg, bg, &format!("{:010}", score));
    }

    /// Update the level display to a specified value
    pub fn update_level(&self, rb: &RustBox, level: u8, style: Style, fg: Color, bg: Color) {
        self.level.print(rb, 1, 1, style, fg, bg, &format!("{:}", level));
    }

    /// Update the current number of lines cleared display to a specified value
    pub fn update_lines(&self, rb: &RustBox, lines: u8, style: Style, fg: Color, bg: Color) {
        self.lines.print(rb, 1, 1, style, fg, bg, &format!("{:}", lines));
    }
}