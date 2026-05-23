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
            // Opcional: Podrías dibujar una última vez o mostrar "Game Over"
            println!("   GAME OVER - Score: {}   ", game.score);
            std::thread::sleep(Duration::from_secs(2)); // Pausa para que el usuario vea que perdió
            break; 
        }
        std::thread::sleep(std::time::Duration::from_millis(5));
    }
    input::cleanup_terminal(); 
}
