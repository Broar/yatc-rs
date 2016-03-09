extern crate rustbox;

use std::error::Error;
use std::time::Duration;

use self::rustbox::RustBox;
use self::rustbox::Key;

use super::ui::Ui;
use super::board::Board;
use super::tetromino::Direction;

const DEFAULT_TIMEOUT: u64 = 100;

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

        let mut running = true;
        while running {
            running = self.handle_input();
            self.render();
        }
    }

    /// Renders the game state and board to the terminal
    fn render(&self) {
        self.ui.print_board(&self.board);
        self.ui.print_next(self.board.peek_next());
        self.rb.present();
    }

    /// Renders the initial elements of the user interface
    fn setup_ui(&self) {
        self.ui.setup();
        self.rb.present();
    }

    /// Handles input events from the user, updating the game state if necessary.
    /// Returns false if the user quits; otherwise, true
    fn handle_input(&mut self) -> bool {

        // Peek at events to avoid blocking if there is no input
        match self.rb.peek_event(Duration::from_millis(DEFAULT_TIMEOUT), false) {
            Ok(rustbox::Event::KeyEvent(key)) => {
                match key {
                    Key::Esc => false,

                    Key::Left => {
                        self.board.left(); 
                        true 
                    },

                    Key::Right => {
                        self.board.right();
                        true
                    },

                    Key::Up => {
                        self.board.up();
                        true
                    },

                    Key::Down => {
                        self.board.down();
                        true
                    },

                    _ => true
                }
            },

            Err(e) => panic!("{}", e.description()),

            _ => { true }
        }
    }
}