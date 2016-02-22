mod game;
mod ui;
mod window;

fn main() {
    let game = game::Game::new();
    game.setup_ui();

    // Main game loop. We continually check for and handle user input before
    // rendering the state of the game to the screen
    let mut running = true;
    while running {
    	running = game.handle_input();
    	game.render();
    }
}
