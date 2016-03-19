extern crate rustbox;

use std::error::Error;
use std::time::Duration;
use std::thread;

use self::rustbox::RustBox;
use self::rustbox::Key;

use super::ui::Ui;
use super::board::Board;
use super::srs::Direction;

const DEFAULT_TIMEOUT: u64 = 100;
const FPS: u64 = 15;

/// A controller between the terminal view and game state
pub struct Game<'a> {
    rb: &'a RustBox,
    ui: Ui<'a>,
    board: Board,
}

impl<'a> Game<'a> {

    /// Initializes a new Game struct
    pub fn new(rb: &'a RustBox) -> Self {
        Game {
            rb: rb,
            ui: Ui::new(rb),
            board: Board::new(),
        }
    }

    /// Starts the main game loop
    pub fn run(&mut self) {
        self.setup_ui();

        loop {

            // Handle the player input. Peek at events to avoid blocking
            match self.rb.peek_event(Duration::from_millis(DEFAULT_TIMEOUT), false) {
                Ok(rustbox::Event::KeyEvent(key)) => {
                    match key {
                        Key::Esc => break,
                        Key::Left => self.board.left(),
                        Key::Right => self.board.right(),
                        Key::Up => self.board.up(),
                        Key::Down => self.board.down(),
                        Key::Char('z') => self.board.rotate(Direction::CounterClockwise),
                        Key::Char('x') => self.board.rotate(Direction::Clockwise),
                        Key::Char('c') => self.board.drop_tetromino(),
                        Key::Char('r') => self.board = Board::new(),
                        _ => { }
                    }
                },

                Err(e) => panic!("{}", e.description()),

                _ => { }
            }

            self.board.tick();

            // The player has lost; we will just restart the game for now
            if self.board.is_topped_out() {
                self.board = Board::new();
                continue;
            }

            self.render();
            thread::sleep(Duration::from_millis(1000 / FPS));
        }
    }

    /// Renders the game state and board to the terminal
    fn render(&self) {
        self.ui.print_board(&self.board);
        self.ui.print_next(self.board.peek_next());
        self.ui.print_score(self.board.score());
        self.ui.print_level(self.board.level());
        self.ui.print_lines(self.board.cleared());
        self.rb.present();
    }

    /// Renders the initial elements of the user interface
    fn setup_ui(&self) {
        self.ui.setup();
        self.rb.present();
    }
}