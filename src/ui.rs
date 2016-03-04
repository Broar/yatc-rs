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
pub struct Ui<'a> {
    board: Window<'a>,
    score: Window<'a>,
    level: Window<'a>,
    lines: Window<'a>,
    next_piece: Window<'a>,
}

impl<'a> Ui<'a> {

    /// Initializes a new Ui struct
    pub fn new(rb: &'a RustBox) -> Self {
        Ui {
            board: Window::new(0, 0, 11, 22, rb),
            score: Window::new(12, 0, 11, 2, rb),
            level: Window::new(12, 4, 11, 2, rb),
            lines: Window::new(12, 8, 11, 2, rb),
            next_piece: Window::new(12, 12, 11, 6, rb)
        }
    }

    /// Setup the default elements of the user interface 
    pub fn setup(&self) {

        // Print out the borders for the player stats
        self.board.print_borders(DEFAULT_STYLE, DEFAULT_FG, DEFAULT_BG);
        self.score.print_borders(DEFAULT_STYLE, DEFAULT_FG, DEFAULT_BG);
        self.level.print_borders(DEFAULT_STYLE, DEFAULT_FG, DEFAULT_BG);
        self.lines.print_borders(DEFAULT_STYLE, DEFAULT_FG, DEFAULT_BG);
        self.next_piece.print_borders(DEFAULT_STYLE, DEFAULT_FG, DEFAULT_BG);

        // Print out the initial values for the player stats
        self.update_score(0);
        self.update_level(0);
        self.update_lines(0);
    }

    /// Update the board presented on the screen
    pub fn update_board(&self, board: &Board) {
        let height = board::HEIGHT - 3;
        let width = board::WIDTH;

        for y in 0..height {
            for x in 0..width {
                match board.blocks[y][x] {
                    Some(ref color) => self.board.print_char(x + 1, y + 1, DEFAULT_STYLE, color.clone(), DEFAULT_BG, '@'),
                    None => { },
                }
            }
        }
    }

    /// Update the next tetromino to be put into play
    pub fn update_next_tetromino(&self, tetromino: Tetromino) {
        let blocks = tetromino.blocks();

        for &(x, y) in blocks.iter() {
            self.next_piece.print_char(x + 4, y + 2, DEFAULT_STYLE, DEFAULT_FG, DEFAULT_BG, '@');
        }
    }

    /// Update the score display
    pub fn update_score(&self, score: u32) {
        self.score.print(1, 1, DEFAULT_STYLE, DEFAULT_FG, DEFAULT_BG, &format!("{:010}", score));
    }

    /// Update the level display
    pub fn update_level(&self, level: u8) {
        self.level.print(1, 1, DEFAULT_STYLE, DEFAULT_FG, DEFAULT_BG, &format!("{:}", level));
    }

    /// Update the number of lines cleared
    pub fn update_lines(&self, lines: u8) {
        self.lines.print(1, 1, DEFAULT_STYLE, DEFAULT_FG, DEFAULT_BG, &format!("{:}", lines));
    }
}