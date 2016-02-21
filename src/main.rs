mod game;

fn main() {
    let game = game::Game::new();
    game.setup();

    // Main game loop. We continually check for and handle user input before
    // rendering the state of the game to the screen
    let mut running = true;
    while running {
    	running = game.handle_input();
    	game.render();
    }

    game.close();
}
