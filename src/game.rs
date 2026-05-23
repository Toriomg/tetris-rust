use crate::tetromino::{self, Tetromino, TypeTetromino};
use rand::Rng;
use std::io::{self, Write};

struct Playfield {
    width: u32,
    height: u32,
}

#[derive(PartialEq)]
pub enum Actions {
    Still,
    Right,
    Left,
    Down,
    Rotate,
    Quit,
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
            3 => Self::Down,
            4 => Self::Rotate,
            _ => Self::Still, // Default case for safety
        }
    }
}

#[derive(PartialEq)]
enum State {
    Playing,
    Paused,
    GameOver,
    Spawning(u32),
}

pub struct Game {
    board: Playfield,
    playfield_mtrx: Vec<Vec<Cell>>,
    pub score: u32,
    state: State,
    current_piece: Option<Tetromino>,
}

#[derive(Debug, Clone, Copy)]
enum Cell {
    Empty,
    Taken(TypeTetromino),
    Clearing,
}

impl Cell {
    pub fn draw(&self) {
        match self {
            Cell::Empty => print!("   "),
            Cell::Clearing => print!("\x1b[47m   \x1b[0m"),
            Cell::Taken(t_type) => {
                let color = match t_type {
                    TypeTetromino::I => "\x1b[36m", // Cyan
                    TypeTetromino::O => "\x1b[33m", // Yellow
                    TypeTetromino::T => "\x1b[34m", // Magenta
                    TypeTetromino::S => "\x1b[32m", // Green
                    TypeTetromino::Z => "\x1b[37m", // Red
                    TypeTetromino::J => "\x1b[35m", // Blue
                    TypeTetromino::L => "\x1b[31m", // White (or use \x1b[38;5;208m for Orange)
                };
                // Reset the color
                let reset = "\x1b[0m";
                //print!("{}[■]{}", color, reset);
                print!("{}███{}", color, reset);
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
            current_piece: Some(Tetromino::new(TypeTetromino::random(), width)),
        };
        if cfg!(debug_assertions) {
            crate::println_raw!("Tablero inicializado");
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
        let message = "GB Tetris";
        print!("{message}");
        for _ in 0..width * 3 - message.len() {
            print!("═");
        }
        crate::println_raw!("╗");

        for (y, row) in self.playfield_mtrx.iter().enumerate() {
            print!("║");
            for (x, cell) in row.iter().enumerate() {
                let mut is_piece_part = false;
                if self.state == State::Playing {
                    if let Some(piece) = &self.current_piece {
                        for (offset_x, offset_y) in piece.shape() {
                            if piece.x + offset_x == x as i32 && piece.y + offset_y == y as i32 {
                                is_piece_part = true;
                                break;
                            }
                        }
                    }
                }
                if is_piece_part {
                    Cell::Taken(self.current_piece.as_ref().unwrap().t_type).draw();
                } else {
                    cell.draw();
                }
            }
            crate::println_raw!("║");
        }
        print!("╚");
        let message = format!("Score: {}", self.score);
        for _ in 0..width * 3 - message.len() {
            print!("═");
        }
        print!("{message}");
        crate::println_raw!("╝");
    }

    fn spawn_piece(&mut self) {
        let random_type = TypeTetromino::random();
        let new_piece = Tetromino::new(random_type, self.board.width);

        // if its colliding its game over
        if !self.is_valid_move(&new_piece) {
            self.state = State::GameOver;
        } else {
            self.current_piece = Some(new_piece);
        }
    }

    pub fn update(&mut self) {
        match self.state {
            State::Playing => {
                self.move_piece(Actions::Down);
            }
            State::Spawning(count) => {
                if count > 0 {
                    // Cecrement spawning counter
                    self.state = State::Spawning(count - 1);
                } else {
                    // New piece
                    self.state = State::Playing;
                    self.spawn_piece();
                }
            }
            _ => (),
        }
    }

    pub fn is_game_over(&self) -> bool {
        return matches!(self.state, State::GameOver)
    }

    fn clear_screen() {
        // \x1B[2J clean screen
        // \x1B[1;1H set the cursor to the start of the screen
        print!("{}[2J{}[1;1H", 27 as char, 27 as char);

        io::stdout().flush().unwrap();
    }

    // Checks wheter a piece can move
    fn is_valid_move(&self, piece: &Tetromino) -> bool {
        for (offset_x, offset_y) in piece.shape() {
            let abs_x = piece.x + offset_x;
            let abs_y = piece.y + offset_y;

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

    pub fn move_piece(&mut self, action: Actions) {
        // copy of the piece in the future
        let Some(mut next_piece) = self.current_piece else {
            return;
        };

        match action {
            Actions::Left => next_piece.x -= 1,
            Actions::Right => next_piece.x += 1,
            Actions::Down => next_piece.y += 1,
            Actions::Rotate => next_piece.rotate(),
            _ => (),
        }

        if self.is_valid_move(&next_piece) {
            // if its valid change the piece
            self.current_piece = Some(next_piece);
        } else if action == Actions::Down {
            // If cannot descend more then new piece
            self.place_piece();
            self.state = State::Spawning(1);
        }
    }

    fn place_piece(&mut self) {
        // Iterate through the 4 relative points of tetromino shape
        if let Some(piece) = self.current_piece.take() {
            for (offset_x, offset_y) in piece.shape() {
                // Calculate absolute coordinates on board
                let abs_x = piece.x + offset_x;
                let abs_y = piece.y + offset_y;

                // Ensure piece is within the matrix limits
                if abs_x >= 0
                    && abs_x < self.board.width as i32
                    && abs_y >= 0
                    && abs_y < self.board.height as i32
                {
                    let x_idx = abs_x as usize;
                    let y_idx = abs_y as usize;

                    // Update cell
                    self.playfield_mtrx[y_idx][x_idx] = Cell::Taken(piece.t_type);
                }
            }
        }
        self.score += 4;
        self.clear_lines();
    }

    fn clear_lines(&mut self) {
    let mut full_lines = Vec::new();

    // 1. Identificar qué filas están llenas
    for (y, row) in self.playfield_mtrx.iter().enumerate() {
        if row.iter().all(|cell| !matches!(cell, Cell::Empty)) {
            full_lines.push(y);
        }
    }

    if full_lines.is_empty() { return; }

    // 2. Efecto visual: Convertir esas filas a estado 'Clearing'
    for &y in &full_lines {
        for x in 0..self.board.width as usize {
            self.playfield_mtrx[y][x] = Cell::Clearing;
        }
    }

    // 3. Forzar un dibujo para que el usuario vea las líneas blancas
    self.draw(); 
    
    // 4. Pausa dramática (ajusta los ms a tu gusto)
    std::thread::sleep(std::time::Duration::from_millis(150));

    // 5. Ahora sí, eliminamos físicamente las líneas (usando tu lógica anterior)
    self.playfield_mtrx.retain(|row| {
        // Ahora descartamos las que son Clearing (que eran las Full)
        !row.iter().all(|cell| matches!(cell, Cell::Clearing))
    });

    // 6. Reponer las líneas arriba y sumar puntuación
    let deleted_count = full_lines.len();
    self.score += deleted_count as u32 * 100;
    for _ in 0..deleted_count {
        let new_row = vec![Cell::Empty; self.board.width as usize];
        self.playfield_mtrx.insert(0, new_row);
    }
}
}
