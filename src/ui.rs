extern crate rustbox;

use self::rustbox::{Color, Style, RustBox};

use super::board::{Board, HEIGHT, WIDTH};
use super::tetromino::{Tetromino, TetrominoType};
use super::window::Window;

// Default scaling factor for the board
const SCALE: usize = 2;

// Default values for styling terminal output
const DEFAULT_STYLE: Style = rustbox::RB_NORMAL;
const DEFAULT_FG: Color = Color::White;
const DEFAULT_BG: Color = Color::Black;

// Default colors for Tetrominos
const I: Color = Color::Cyan;
const J: Color = Color::Blue;
const L: Color = Color::White;
const O: Color = Color::Yellow;
const S: Color = Color::Green;
const T: Color = Color::Magenta;
const Z: Color = Color::Red;

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
            board: Window::new(0, 0, (11 * SCALE) - 1, 21, rb),
            score: Window::new(12 * SCALE, 0, 11, 2, rb),
            level: Window::new(12 * SCALE, 4, 11, 2, rb),
            lines: Window::new(12 * SCALE, 8, 11, 2, rb),
            next_piece: Window::new(12 * SCALE, 12, 11, 6, rb)
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
    }

    /// Print the state of the board
    pub fn print_board(&self, board: &Board) {

        // Start at 2 because only 20 of the board's rows should be displayed
        for y in 2..HEIGHT {
            for x in 0..WIDTH {
                match board.field()[y][x] {

                    // When printing the board, offset x and y to compensate
                    // for the Window's borders and showing only 20 rows
                    Some(ref mino) =>  {
                        let color = self.get_tetromino_color(mino);
                        self.board.print_char((x * SCALE) + 1, y - 1, DEFAULT_STYLE, color, DEFAULT_BG, '■');
                        self.board.print_char((x * SCALE) + 2, y - 1, DEFAULT_STYLE, color, DEFAULT_BG, '■');
                    }

                    None => {
                        self.board.print_char((x * SCALE) + 1, y - 1, DEFAULT_STYLE, DEFAULT_FG, DEFAULT_BG, ' ');
                        self.board.print_char((x * SCALE) + 2, y - 1, DEFAULT_STYLE, DEFAULT_FG, DEFAULT_BG, '.');
                    },
                }
            }
        }
    }

    /// Get the color associated with a TetrominoType
    fn get_tetromino_color(&self, tetromino_type: &TetrominoType) -> Color {
        match tetromino_type {
            &TetrominoType::I => I,
            &TetrominoType::J => J,
            &TetrominoType::L => L,
            &TetrominoType::O => O,
            &TetrominoType::S => S,
            &TetrominoType::T => T,
            &TetrominoType::Z => Z,
        }
    }

    /// Print the next Tetromino
    pub fn print_next(&self, tetromino: Tetromino) {
        for i in 1..self.next_piece.w {
            for j in 1..self.next_piece.h {
                self.next_piece.print_char(i, j, DEFAULT_STYLE, DEFAULT_FG, DEFAULT_BG, ' ');
            }
        }

        for &mino in tetromino.minos().iter() {
            let color = self.get_tetromino_color(&tetromino.tetromino_type());
            self.next_piece.print_char((mino.x + 4) as usize, (mino.y + 2) as usize, DEFAULT_STYLE, color, DEFAULT_BG, '■');
        }
    }

    /// Print the player's score
    pub fn print_score(&self, score: usize) {
        self.score.print(1, 1, DEFAULT_STYLE, DEFAULT_FG, DEFAULT_BG, &format!("{:}", score));
    }

    /// Print the difficulty level
    pub fn print_level(&self, level: usize) {
        self.level.print(1, 1, DEFAULT_STYLE, DEFAULT_FG, DEFAULT_BG, &format!("{:}", level));
    }

    /// Print the number of lines cleared
    pub fn print_lines(&self, lines: usize) {
        self.lines.print(1, 1, DEFAULT_STYLE, DEFAULT_FG, DEFAULT_BG, &format!("{:}", lines));
    }
}