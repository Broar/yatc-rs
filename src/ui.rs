extern crate rustbox;

use self::rustbox::{Color, Style, RustBox};

use super::board::Board;
use super::board;
use super::tetromino::{Mino, Tetromino, TetrominoType};
use super::window::Window;

// Default values for styling terminal output
const DEFAULT_STYLE: Style = rustbox::RB_NORMAL;
const DEFAULT_FG: Color = Color::White;
const DEFAULT_BG: Color = Color::Black;

// Default character and color pairs
const I: (char, Color) = ('@', Color::Cyan);
const J: (char, Color) = ('#', Color::Blue);
const L: (char, Color) = ('&', Color::White);
const O: (char, Color) = ('%', Color::Yellow);
const S: (char, Color) = ('$', Color::Green);
const T: (char, Color) = ('*', Color::Magenta);
const Z: (char, Color) = ('O', Color::Red);

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
                match board.field[y][x] {
                    Some(ref mino) =>  {
                        let (rune, color) = self.get_style(mino);
                        self.board.print_char(x + 1, y + 1, DEFAULT_STYLE, color, DEFAULT_BG, rune);
                    }

                    None => self.board.print_char(x + 1, y + 1, DEFAULT_STYLE, DEFAULT_FG, DEFAULT_BG, ' '),
                }
            }
        }
    }

    /// Get the print style for a Mino
    fn get_style(&self, mino: &Mino) -> (char, Color) {
        match mino.tetro_type {
            TetrominoType::I => I,
            TetrominoType::J => J,
            TetrominoType::L => L,
            TetrominoType::O => O,
            TetrominoType::S => S,
            TetrominoType::T => T,
            TetrominoType::Z => Z,
        }
    }

    /// Update the next tetromino to be put into play
    pub fn update_next_tetromino(&self, tetromino: Tetromino) {
        for &mino in tetromino.minos.iter() {
            let (x, y) = (mino.pos.x as usize, mino.pos.y as usize);
            let (rune, color) = self.get_style(&mino);
            self.next_piece.print_char((x + 4) as usize, (y + 2) as usize, DEFAULT_STYLE, color, DEFAULT_BG, rune);
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