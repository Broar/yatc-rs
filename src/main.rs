mod board;
mod game;
mod tetromino;
mod ui;
mod window;

fn main() {
    let game = game::Game::new();
    game.run();
}
