struct Playfield {
    width: u32,
    height: u32,
}

enum Tetromino {
    I, // 4 Cell bar
    O, // Square
    T,
    S,
    Z,
    J,
    L,
}

enum Cell {
    empty,
    taken(Tetromino),
}

struct Game{
    board: struct Playfield,
    playfieldMtx: [[Cell; 10]; 18]
}

impl Game {
    fn new(){
        board.width  = 10;
        board.height = 10;
    }
}

fn main() {
    println!("Hello, world!");
}
