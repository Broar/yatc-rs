extern crate rustbox;

use std::error::Error;
use std::time::Duration;
use std::sync::mpsc;
use std::thread;

use std::sync::{Arc, Mutex};

use self::rustbox::RustBox;
use self::rustbox::Key;

use super::ui::Ui;
use super::board::Board;
use super::srs::Direction;

const TIMEOUT: u64 = 100;
const FPS: u64 = 60;
const DELAY: [u64; 20] = [ 800, 720, 630, 550, 470, 
                           380, 300, 220, 130, 100, 
                            80,  80,  80,  70,  70, 
                            70,  50,  50,  50,  30, ];

/// A controller between the terminal view and game state
pub struct Game<'a> {
    rb: &'a RustBox,
    ui: Ui<'a>,
    board: Arc<Mutex<Board>>,
}

impl<'a> Game<'a> {

    /// Initializes a new Game struct
    pub fn new(rb: &'a RustBox) -> Self {
        Game {
            rb: rb,
            ui: Ui::new(rb),
            board: Arc::new(Mutex::new(Board::new())),
        }
    }

    /// Starts the main game loop
    pub fn run(&mut self) {
        self.ui.setup();

        // Create a channel to communicate to from the main thread to 
        // the gravity thread about whether the game has ended
        let (send, recv) = mpsc::channel();

        // Spawn a thread that periodically ticks the board
        let guard = self.board.clone();
        let gravity = thread::spawn(move || {
            let mut delay = DELAY[0];

            loop {

                // Check for a message from the main thread and handle it
                if let Ok(true) = recv.try_recv() {
                    break;
                }

                thread::sleep(Duration::from_millis(delay));

                let mut board = guard.lock().unwrap();
                board.tick();
                
                delay = if board.level() <= 19 { DELAY[board.level()] } else { DELAY[19] };
            }
        });

        // Main thread handles the player input and rendering
        loop {

            let mut board = self.board.lock().unwrap();

            // Handle the player input. Peek at events to avoid blocking
            match self.rb.peek_event(Duration::from_millis(TIMEOUT), false) {
                Ok(rustbox::Event::KeyEvent(key)) => {
                    match key {
                        Key::Esc => {
                            // The player is quitting, so inform the gravity thread
                            send.send(true);
                            break;
                        },

                        Key::Left      => board.left(),
                        Key::Right     => board.right(),
                        Key::Down      => board.down(),
                        Key::Char('z') => board.rotate(Direction::CounterClockwise),
                        Key::Char('x') => board.rotate(Direction::Clockwise),
                        Key::Char('c') => board.drop_tetromino(),
                        Key::Char(' ') => board.hold(),
                        _ => { }
                    }
                },

                Err(e) => panic!("{}", e.description()),

                _ => { }
            }

            // The player has lost, so inform the gravity thread
            if board.is_topped_out() {
                send.send(true);
                break;
            }

            self.render(&board);
            thread::sleep(Duration::from_millis(1000 / FPS));
        }

        gravity.join();
    }

    /// Renders the game state and board to the terminal
    fn render(&self, board: &Board) {
        self.ui.print_board(board);
        self.ui.print_next(board.peek_next());
        self.ui.print_score(board.score());
        self.ui.print_level(board.level());
        self.ui.print_lines(board.cleared());
        self.rb.present();
    }
 }