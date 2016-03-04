extern crate rustbox;

use self::rustbox::RustBox;
use std::default::Default;
use std::error::Error;

mod board;
mod game;
mod tetromino;
mod ui;
mod window;

fn main() {
    // Create a RustBox instance to handle output to the terminal
    let rb = match RustBox::init(Default::default()) {
        Result::Ok(v) => v,
        Result::Err(e) => panic!("{}", e.description()),
    };

    // Start the game
    game::Game::new(&rb).run();
}
