mod game;
mod input;
mod tetromino;
mod utils;
use game::Game;

use std::time::{Duration, Instant};

fn main() {
    input::init_terminal();
    let mut game = Game::new(10, 22);
    let mut last_drop = Instant::now();
    let drop_interval = Duration::from_millis(500);
    loop {
        if let Some(action) = input::poll_action() {
            match action {
                game::Actions::Quit => {
                    break;
                }
                _ => game.move_piece(action),
            }
        }
        if last_drop.elapsed() >= drop_interval {
            game.update();
            last_drop = Instant::now();
        }
        game.draw();
        std::thread::sleep(std::time::Duration::from_millis(50));
    }
    input::cleanup_terminal(); 
}
