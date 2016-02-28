extern crate rustbox;

use std::error::Error;
use std::default::Default;
use std::time::Duration;

use self::rustbox::RustBox;
use self::rustbox::Key;

use super::ui::Ui;
use super::board::Board;

/// A controller between the terminal view and game state
pub struct Game {
    rb: RustBox,
    ui: Ui,
    board: Board,
    timeout: Duration,
}

impl Game {

    /// Initializes a new Game struct
    pub fn new() -> Self {
        let rb = match RustBox::init(Default::default()) {
            Result::Ok(v) => v,
            Result::Err(e) => panic!("{}", e.description()),
        };

        Game {
            rb: rb,
            ui: Ui::new(),
            board: Board::new(),
            timeout: Duration::from_millis(100),
        }
    }

    /// Starts the main game loop
    pub fn run(&self) {
        self.setup_ui();

        let mut running = true;
        while running {
            running = self.handle_input();
            self.render();
        }
    }

    /// Renders the game state and board to the terminal
    pub fn render(&self) {
        self.ui.update_board(&self.rb, &self.board);
        self.rb.present();
    }

    /// Renders the initial elements of the user interface
    pub fn setup_ui(&self) {
        self.ui.setup(&self.rb);
        self.rb.present();
    }

    /// Handles input events from the user, updating the game state if necessary.
    /// Returns false if the user quits; otherwise, true
    pub fn handle_input(&self) -> bool {

        // Peek at events to avoid blocking if there is no input
        match self.rb.peek_event(self.timeout, false) {
            Ok(rustbox::Event::KeyEvent(key)) => {
                match key {
                    Key::Char('q') => { false },
                    _ => { true } 
                }
            },

            Err(e) => panic!("{}", e.description()),

            _ => { true }
        }
    }
}