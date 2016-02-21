extern crate rustbox;

use std::error::Error;
use std::default::Default;
use std::time::Duration;

use self::rustbox::{Color, RustBox};
use self::rustbox::Key;

/// A controller between the terminal view and game state
pub struct Game {
    rb: RustBox,
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
            timeout: Duration::from_millis(100),
        }
    }

    /// Renders the game state and board to the terminal
    pub fn render(&self) {
        // TODO: Implementation of GameState and Board required
    }

    // Render the initial elements including the board, stats, and next piece area
    pub fn setup(&self) {
        let style = rustbox::RB_NORMAL;
        let fg = Color::White;
        let bg = Color::Black;

        // Render the borders of the board
        self.rb.print(0, 0, style, fg, bg, "┌────────┐");

        for i in 1..20 {
            self.rb.print_char(0, i, style, fg, bg, '│');
            self.rb.print_char(9, i, style, fg, bg, '│');
        }

        self.rb.print(0, 20, style, fg, bg, "└────────┘");

        // Render the player's score
        self.rb.print(11, 0, style, fg, bg, "┌───SCORE───┐");
        self.rb.print(11, 1, style, fg, bg, "│ 000000000 │");
        self.rb.print(11, 2, style, fg, bg, "└───────────┘");

        // Render the player's level
        self.rb.print(11, 4, style, fg, bg, "┌───LEVEL───┐");
        self.rb.print(11, 5, style, fg, bg, "│     0     │");
        self.rb.print(11, 6, style, fg, bg, "└───────────┘");

        // Render the lines cleared by the player
        self.rb.print(11, 8, style, fg, bg,  "┌───LINES───┐");
        self.rb.print(11, 9, style, fg, bg,  "│     0     │");
        self.rb.print(11, 10, style, fg, bg, "└───────────┘");

        // Render the next piece area
        self.rb.print(11, 12, style, fg, bg, "┌───────────┐");

        for i in 1..5 {
            self.rb.print_char(11, i + 12, style, fg, bg, '│');
            self.rb.print_char(23, i + 12, style, fg, bg, '│');
        }

        self.rb.print(11, 17, style, fg, bg, "└───────────┘");


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