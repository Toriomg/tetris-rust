struct Playfield {
    width: u32,
    height: u32,
}

#[allow(dead_code)] //quitar es q quedan feos de ver lso errores
#[derive(Clone)]
enum Tetromino {
    I, // 4 Cell bar
    O, // Square
    T,
    S,
    Z,
    J,
    L,
}

#[derive(Clone)]
enum Cell {
    Empty,
    Taken(Tetromino),
}

enum Actions {
    Right,
    Left,
    Down,
    Rotate,
}

struct Game {
    board: Playfield,
    playfield_mtrx: Vec<Vec<Cell>>,
    score: u32,
}

impl Game {
    fn new(width: u32, height: u32) -> Self {
        let mtx = vec![vec![Cell::Empty; width as usize]; height as usize];
        let g = Self {
            board: Playfield {
                width: width,
                height: height,
            },
            score: 0,
            playfield_mtrx: mtx,
        };
        if cfg!(debug_assertions) {
            println!("Tablero inicializado");
        }
        g
    }

    fn draw(&self) {
        for row in &self.playfield_mtrx {
            for _ in row {
                print!("█▓▒")
            }
            println!()
        }
    }
}

fn main() {
    let game = Game::new(10, 18);
    game.draw();
}
