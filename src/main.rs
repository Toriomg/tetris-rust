mod game;
mod tetromino;
use game::Game;

fn main() {
    let mut game = Game::new(10, 22);
    loop {
        game.update();
        game.draw();
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
}
