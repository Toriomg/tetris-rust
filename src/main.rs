mod game;
mod input;
mod tetromino;
mod utils;
mod cell;
use game::Game;

use std::time::{Duration, Instant};

fn main() {
    input::init_terminal();
    let mut game = Game::new(10, 18);
    let mut last_drop = Instant::now();
    let drop_interval = Duration::from_millis(150);
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
        if game.is_game_over() {
            println!("   GAME OVER - Score: {}   ", game.score);
            break; 
        }
        std::thread::sleep(std::time::Duration::from_millis(5));
    }
    input::cleanup_terminal(); 
}
