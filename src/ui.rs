extern crate rustbox;

use self::rustbox::{Color, Style, RustBox};

use super::board::Board;
use super::board;
use super::tetromino::Tetromino;
use super::window::Window;

// Default values for styling terminal output
const DEFAULT_STYLE: Style = rustbox::RB_NORMAL;
const DEFAULT_FG: Color = Color::White;
const DEFAULT_BG: Color = Color::Black;

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

        // Print out the borders for the player stats
        self.board.print_borders(rb, DEFAULT_STYLE, DEFAULT_FG, DEFAULT_BG);
        self.score.print_borders(rb, DEFAULT_STYLE, DEFAULT_FG, DEFAULT_BG);
        self.level.print_borders(rb, DEFAULT_STYLE, DEFAULT_FG, DEFAULT_BG);
        self.lines.print_borders(rb, DEFAULT_STYLE, DEFAULT_FG, DEFAULT_BG);
        self.next_piece.print_borders(rb, DEFAULT_STYLE, DEFAULT_FG, DEFAULT_BG);

        // Print out the initial values for the player stats
        self.update_score(rb, 0);
        self.update_level(rb, 0);
        self.update_lines(rb, 0);
    }

    /// Update the board presented on the screen
    pub fn update_board(&self, rb: &RustBox, board: &Board) {
        let height = board::HEIGHT - 3;
        let width = board::WIDTH;

        for y in 0..height {
            for x in 0..width {
                match board.blocks[y][x] {
                    Some(ref color) => self.board.print_char(rb, x + 1, y + 1, DEFAULT_STYLE, color.clone(), DEFAULT_BG, '@'),
                    None => { },
                }
            }
        }
    }

    /// Update the next tetromino to be put into play
    pub fn update_next_tetromino(&self, rb: &RustBox, tetromino: Tetromino) {
        let blocks = tetromino.blocks();

        for &(x, y) in blocks.iter() {
            self.next_piece.print_char(rb, x + 4, y + 2, DEFAULT_STYLE, DEFAULT_FG, DEFAULT_BG, '@');
        }
    }

    /// Update the score display
    pub fn update_score(&self, rb: &RustBox, score: u32) {
        self.score.print(rb, 1, 1, DEFAULT_STYLE, DEFAULT_FG, DEFAULT_BG, &format!("{:010}", score));
    }

    /// Update the level display
    pub fn update_level(&self, rb: &RustBox, level: u8) {
        self.level.print(rb, 1, 1, DEFAULT_STYLE, DEFAULT_FG, DEFAULT_BG, &format!("{:}", level));
    }

    /// Update the number of lines cleared
    pub fn update_lines(&self, rb: &RustBox, lines: u8) {
        self.lines.print(rb, 1, 1, DEFAULT_STYLE, DEFAULT_FG, DEFAULT_BG, &format!("{:}", lines));
    }
}