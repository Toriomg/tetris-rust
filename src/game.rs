use std::io::{self, Write};
use std::thread;
use std::time::Duration;

use crate::tetromino::Cell; 

struct Playfield {
    width: u32,
    height: u32,
}

enum Actions {
    Right,
    Left,
    Down,
    Rotate,
}

enum State {
    Playing,
    Paused,
    GameOver,
}

pub struct Game {
    board: Playfield,
    playfield_mtrx: Vec<Vec<Cell>>,
    score: u32,
    state: State,
}

impl Game {
    pub fn new(width: u32, height: u32) -> Self {
        let mtx = vec![vec![Cell::Empty; width as usize]; height as usize];
        let g = Self {
            board: Playfield {
                width: width,
                height: height,
            },
            score: 0,
            playfield_mtrx: mtx,
            state: State::Playing,
        };
        if cfg!(debug_assertions) {
            println!("Tablero inicializado");
        }
        g
    }

    pub fn draw(&self) {
        Self::clear_screen();
        let height = self.playfield_mtrx.len();
        if height == 0 {
            return;
        } // Seguridad
        let width = self.playfield_mtrx[0].len();

        print!("╔");
        for _ in 0..width {
            print!("═══"); 
        println!("╗");

        for row in &self.playfield_mtrx {
            print!("║");
            for cell in row {
                cell.draw()
            }
            println!("║");
        }
        print!("╚");
        for _ in 0..width {
            print!("═══");
        }
        println!("╝");
        thread::sleep(Duration::from_secs(1));
    }

    fn clear_screen() {
        // \x1B[2J clean screen
        // \x1B[1;1H set the cursor to the start of the screen
        print!("{}[2J{}[1;1H", 27 as char, 27 as char);

        io::stdout().flush().unwrap();
    }

    pub fn update(&self) {}
}
