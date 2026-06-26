mod bag_system;
mod cell;
mod game;
mod input;
mod tetromino;
mod ui;
mod utils;
use game::Game;

use std::time::{Duration, Instant};

fn main() {
    input::init_terminal();
    let mut game = Game::new(10, 18, bag_system::GeneratorMode::_Classic);
    let mut last_drop = Instant::now();
    let drop_interval = Duration::from_millis(150);

    loop {
        // process input
        if let Some(action) = input::poll_action() {
            // Close game if handle_actions returns false
            if !game.handle_action(action) {
                break;
            }
        }

        if last_drop.elapsed() >= drop_interval {
            game.update();
            last_drop = Instant::now();
        }
        game.draw();

        if game.is_game_over() {
            println!("   GAME OVER - Score: {}   ", game.score);
            break;
        }
        std::thread::sleep(std::time::Duration::from_millis(5));
    }
    input::cleanup_terminal();
}
