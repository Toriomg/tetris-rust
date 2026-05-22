use std::io::{self, Write};
use std::thread;
use std::time::Duration;

use crate::tetromino::{self, Tetromino, TypeTetromino};

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

#[derive(Debug, Clone, Copy)]
enum Cell {
    Empty,
    Taken(TypeTetromino),
}

impl Cell {
    pub fn draw(&self) {
        match self {
            Cell::Empty => print!("[ ]"),
            Cell::Taken(t_type) => {
                let color = match t_type {
                    TypeTetromino::I => "\x1b[36m", // Cyan
                    TypeTetromino::O => "\x1b[33m", // Yellow
                    TypeTetromino::T => "\x1b[35m", // Magenta
                    TypeTetromino::S => "\x1b[32m", // Green
                    TypeTetromino::Z => "\x1b[31m", // Red
                    TypeTetromino::J => "\x1b[34m", // Blue
                    TypeTetromino::L => "\x1b[37m", // White (or use \x1b[38;5;208m for Orange)
                };
                let reset = "\x1b[0m";
                // Print the colored block
                print!("{}[■]{}", color, reset);
            }
        }
    }
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
        }
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

    pub fn update(&mut self) {
        let p = Tetromino::new(tetromino::TypeTetromino::J);
        self.place_piece(&p);
    }

    fn clear_screen() {
        // \x1B[2J clean screen
        // \x1B[1;1H set the cursor to the start of the screen
        print!("{}[2J{}[1;1H", 27 as char, 27 as char);

        io::stdout().flush().unwrap();
    }

    pub fn place_piece(&mut self, piece: &Tetromino) {
        // Iterate through the 4 relative points of the tetromino shape
        for (offset_x, offset_y) in piece.shape() {
            // Calculate absolute coordinates on the board
            let abs_x = piece.x + offset_x;
            let abs_y = piece.y + offset_y;

            // Ensure the piece is within the matrix limits
            if abs_x >= 0
                && abs_x < self.board.width as i32
                && abs_y >= 0
                && abs_y < self.board.height as i32
            {
                let x_idx = abs_x as usize;
                let y_idx = abs_y as usize;

                // Update the cell
                self.playfield_mtrx[y_idx][x_idx] = Cell::Taken(piece.t_type);
            }
        }
    }
}
