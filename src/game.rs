use crate::tetromino::{self, Tetromino, TypeTetromino};
use rand::Rng;
use std::io::{self, Write};

struct Playfield {
    width: u32,
    height: u32,
}

enum Actions {
    Still,
    Right,
    Left,
    Down,
    Rotate,
}

impl Actions {
    /**
     *  Returns a random tetromino type
     */
    fn random() -> Self {
        let mut rng = rand::thread_rng();
        match rng.gen_range(0..5) {
            0 => Self::Still,
            1 => Self::Right,
            2 => Self::Left,
            //3 => Self::Down,
            4 => Self::Rotate,
            _ => Self::Still, // Default case for safety
        }
    }
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
    current_piece: Tetromino,
}

#[derive(Debug, Clone, Copy)]
enum Cell {
    Empty,
    Taken(TypeTetromino),
}

impl Cell {
    pub fn draw(&self) {
        match self {
            Cell::Empty => print!("   "),
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
                // Reset the color
                let reset = "\x1b[0m";
                print!("{}[■]{}", color, reset);
                //print!("{} █ {}", color, reset);
            }
        }
    }
}

impl Game {
    pub fn new(width: u32, height: u32) -> Self {
        let mtx = vec![vec![Cell::Empty; width as usize]; height as usize];
        let random_type = TypeTetromino::random();
        let g = Self {
            board: Playfield {
                width: width,
                height: height,
            },
            score: 0,
            playfield_mtrx: mtx,
            state: State::Playing,
            current_piece: Tetromino::new(random_type, width),
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
        }
        let width = self.playfield_mtrx[0].len();

        print!("╔");
        for _ in 0..width {
            print!("═══");
        }
        println!("╗");

        for (y, row) in self.playfield_mtrx.iter().enumerate() {
            print!("║");
            for (x, cell) in row.iter().enumerate() {
                let mut is_piece_part = false;
                for (offset_x, offset_y) in self.current_piece.shape() {
                    if self.current_piece.x + offset_x == x as i32
                        && self.current_piece.y + offset_y == y as i32
                    {
                        is_piece_part = true;
                        break;
                    }
                }
                if is_piece_part {
                    Cell::Taken(self.current_piece.t_type).draw();
                } else {
                    cell.draw();
                }
            }
            println!("║");
        }
        print!("╚");
        for _ in 0..width {
            print!("═══");
        }
        println!("╝");
    }

    fn spawn_piece(&mut self) {
        let random_type = TypeTetromino::random();
        self.current_piece = Tetromino::new(random_type, self.board.width);

        // if its colliding its game over
        if !self.is_valid_move(self.current_piece.x, self.current_piece.y) {
            self.state = State::GameOver;
        }
    }

    pub fn update(&mut self) {
        match self.state {
            State::Playing => {
                self.move_piece(Actions::Down);
                let random_movement = Actions::random();
                self.move_piece(random_movement);
            }
            State::GameOver => {
                std::process::exit(0);
            }
            _ => (),
        }
    }

    fn clear_screen() {
        // \x1B[2J clean screen
        // \x1B[1;1H set the cursor to the start of the screen
        print!("{}[2J{}[1;1H", 27 as char, 27 as char);

        io::stdout().flush().unwrap();
    }

    // Checks wheter a piece can move
    fn is_valid_move(&self, piece_x: i32, piece_y: i32) -> bool {
        for (offset_x, offset_y) in self.current_piece.shape() {
            let abs_x = piece_x + offset_x;
            let abs_y = piece_y + offset_y;

            // lat left
            if abs_x < 0 {
                return false;
            }

            // lat right
            if abs_x >= self.board.width as i32 {
                return false;
            }

            // floor
            if abs_y >= self.board.height as i32 {
                return false;
            }

            // check collision
            if abs_y >= 0 {
                // check inside the matrix
                if let Cell::Taken(_) = self.playfield_mtrx[abs_y as usize][abs_x as usize] {
                    return false;
                }
            }
        }
        true
    }

    fn move_piece(&mut self, action: Actions) {
        match action {
            Actions::Left => {
                if self.is_valid_move(self.current_piece.x - 1, self.current_piece.y) {
                    self.current_piece.x -= 1;
                }
            }
            Actions::Right => {
                if self.is_valid_move(self.current_piece.x + 1, self.current_piece.y) {
                    self.current_piece.x += 1;
                }
            }
            Actions::Down => {
                if self.is_valid_move(self.current_piece.x, self.current_piece.y + 1) {
                    self.current_piece.y += 1;
                } else {
                    // piece placed if cannot move more
                    self.place_piece();
                    self.spawn_piece();
                }
            }
            _ => (),
        }
    }

    fn place_piece(&mut self) {
        // Iterate through the 4 relative points of tetromino shape
        for (offset_x, offset_y) in self.current_piece.shape() {
            // Calculate absolute coordinates on board
            let abs_x = self.current_piece.x + offset_x;
            let abs_y = self.current_piece.y + offset_y;

            // Ensure piece is within the matrix limits
            if abs_x >= 0
                && abs_x < self.board.width as i32
                && abs_y >= 0
                && abs_y < self.board.height as i32
            {
                let x_idx = abs_x as usize;
                let y_idx = abs_y as usize;

                // Update cell
                self.playfield_mtrx[y_idx][x_idx] = Cell::Taken(self.current_piece.t_type);
            }
        }
        self.clear_lines();
    }

    fn clear_lines(&mut self) {
        // safe height for late check then
        let old_height = self.playfield_mtrx.len();

        // clean non full lines
        self.playfield_mtrx
            .retain(|row| row.iter().any(|cell| matches!(cell, Cell::Empty)));

        let deleted_count = old_height - self.playfield_mtrx.len();

        if deleted_count > 0 {
            // Add score
            self.score += deleted_count as u32 * 100;

            for _ in 0..deleted_count {
                // New rows
                let new_row = vec![Cell::Empty; self.board.width as usize];
                // inserted into the start
                self.playfield_mtrx.insert(0, new_row);
            }
        }
    }
}
