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

/// A collection of Window structs representing the user interface
pub struct Ui<'a> {
    board: Window<'a>,
    score: Window<'a>,
    level: Window<'a>,
    lines: Window<'a>,
    next: Window<'a>,
    hold: Window<'a>,
}

impl<'a> Ui<'a> {

    /// Initializes a new Ui struct
    pub fn new(rb: &'a RustBox) -> Self {
        Ui {
            board: Window::new(0, 5, (11 * SCALE) - 1, 21, rb),
            score: Window::new(12 * SCALE, 6, 11, 1, rb),
            level: Window::new(12 * SCALE, 10, 11, 1, rb),
            lines: Window::new(12 * SCALE, 14, 11, 1, rb),
            next:  Window::new(5 , 1, (5 * SCALE) + 1, 4, rb),
            hold: Window::new(12 * SCALE, 18, (5 * SCALE) + 1, 5, rb),
        }
    }

    /// Setup the default elements of the user interface 
    pub fn setup(&self) {
        self.board.print_borders(DEFAULT_STYLE, DEFAULT_FG, DEFAULT_BG);
        self.next.print_borders(DEFAULT_STYLE, DEFAULT_FG, DEFAULT_BG);
        self.hold.print_borders(DEFAULT_STYLE, DEFAULT_FG, DEFAULT_BG);
        self.print_score(0);
        self.print_level(0);
        self.print_lines(0);
    }

    /// Resets some of the user interface elements
    pub fn reset(&self) {
        self.score.clear();
        self.level.clear();
        self.lines.clear();

        self.print_score(0);
        self.print_level(0);
        self.print_lines(0);
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
                        let rune = self.get_tetromino_rune(mino);
                        self.board.print_char((x * SCALE) + 1, y - 1, DEFAULT_STYLE, color, DEFAULT_BG, rune);
                        self.board.print_char((x * SCALE) + 2, y - 1, DEFAULT_STYLE, color, DEFAULT_BG, rune);
                    }

                    None => {
                        self.board.print_char((x * SCALE) + 1, y - 1, DEFAULT_STYLE, DEFAULT_FG, DEFAULT_BG, ' ');
                        self.board.print_char((x * SCALE) + 2, y - 1, DEFAULT_STYLE, DEFAULT_FG, DEFAULT_BG, '.');
                    },
                }
            }
        }
    }

    /// Gets the character associated with a TetrominoType
    fn get_tetromino_rune(&self, tetromino_type: &TetrominoType) -> char {
        match tetromino_type {
            &TetrominoType::Ghost => '□',
            _ => '■',
        }
    }

    /// Gets the color associated with a TetrominoType
    fn get_tetromino_color(&self, tetromino_type: &TetrominoType) -> Color {
        match tetromino_type {
            &TetrominoType::I => Color::Cyan,
            &TetrominoType::J => Color::Blue,
            &TetrominoType::L | &TetrominoType::Ghost => Color::White,
            &TetrominoType::O => Color::Yellow,
            &TetrominoType::S => Color::Green,
            &TetrominoType::T => Color::Magenta,
            &TetrominoType::Z => Color::Red,
        }
    }

    /// Prints the next Tetromino
    pub fn print_next(&self, tetromino: Tetromino) {
        self.print_tetromino(tetromino, &self.next);
    }

    /// Prints the hold Tetromino
    pub fn print_hold(&self, hold: Option<Tetromino>) {
        if let Some(tetromino) = hold {
            self.print_tetromino(tetromino, &self.hold);
        }
    }

    // Prints a Tetromino to a specified Window
    fn print_tetromino(&self, tetromino: Tetromino, window: &Window) {
        window.clear();
        window.print_borders(DEFAULT_STYLE, DEFAULT_FG, DEFAULT_BG);

        for &mino in tetromino.minos().iter() {
            let color = self.get_tetromino_color(&tetromino.tetromino_type());
            window.print_char(((mino.x as usize) * SCALE + 2), (mino.y + 1) as usize, DEFAULT_STYLE, color, DEFAULT_BG, '■');
            window.print_char(((mino.x as usize) * SCALE + 3), (mino.y + 1) as usize, DEFAULT_STYLE, color, DEFAULT_BG, '■');
        }
    }

    /// Prints the player's score
    pub fn print_score(&self, score: usize) {
        self.score.print(0, 0, DEFAULT_STYLE, DEFAULT_FG, DEFAULT_BG, &format!("{:}", score));
    }

    /// Prints the difficulty level
    pub fn print_level(&self, level: usize) {
        self.level.print(0, 0, DEFAULT_STYLE, DEFAULT_FG, DEFAULT_BG, &format!("{:}", level));
    }

    /// Prints the number of lines cleared
    pub fn print_lines(&self, lines: usize) {
        self.lines.print(0, 0, DEFAULT_STYLE, DEFAULT_FG, DEFAULT_BG, &format!("{:}", lines));
    }
}