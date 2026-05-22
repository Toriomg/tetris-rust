mod game;
mod tetromino;
use game::Game;

fn main() {
    let game = Game::new(10, 18);
    loop {
        game.update();
        game.draw();
    }
}
