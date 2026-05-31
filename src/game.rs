use crate::cell::Cell;
use crate::tetromino::{Tetromino, TypeTetromino};
use rand::Rng;
use std::collections::VecDeque;

pub const PREVIEW_COUNT: usize = 0;
const SCORE_PER_PIECE: u32 = 4;
const SCORE_SINGLE_LINE: u32 = 40;
const SCORE_DOUBLE_LINE: u32 = 100;
const SCORE_TRIPLE_LINE: u32 = 300;
const SCORE_TETRIS_LINE: u32 = 1200;

pub struct Playfield {
    pub width: u32,
    pub height: u32,
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
    fn _random() -> Self {
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
pub enum State {
    Playing,
    Paused,
    GameOver,
    Spawning(u32),
}

pub struct Game {
    pub board: Playfield,
    pub playfield_mtrx: Vec<Vec<Cell>>,
    pub score: u32,
    pub level: u8,
    pub state: State,
    pub current_piece: Option<Tetromino>,
    pub next_pieces: VecDeque<TypeTetromino>,
}

impl Game {
    pub fn new(width: u32, height: u32) -> Self {
        let mtx = vec![vec![Cell::Empty; width as usize]; height as usize];
        
        // init the new queue
        let mut next_pieces = VecDeque::new();
        let following_pieces = if PREVIEW_COUNT == 0 {1} else {PREVIEW_COUNT};
        for _ in 0..following_pieces {
            next_pieces.push_back(TypeTetromino::random());
        }

        // the first piece
        let first_piece_type = next_pieces.pop_front().unwrap();
        next_pieces.push_back(TypeTetromino::random());

        Self {
            board: Playfield { width, height },
            playfield_mtrx: mtx,
            score: 0,
            level: 9,
            state: State::Playing,
            current_piece: Some(Tetromino::new(first_piece_type, width)),
            next_pieces,
        }
    }

    fn spawn_piece(&mut self) {
        if let Some(t_type) = self.next_pieces.pop_front() {
            // Get the new piece
            self.next_pieces.push_back(TypeTetromino::random());
            let new_piece = Tetromino::new(t_type, self.board.width);
            
            // Check if its a valid move
            if !self.is_valid_move(&new_piece) {
                self.state = State::GameOver;
            } else {
                self.current_piece = Some(new_piece);
            }
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
        return matches!(self.state, State::GameOver);
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

        // identify full lines
        for (y, row) in self.playfield_mtrx.iter().enumerate() {
            if row.iter().all(|cell| !matches!(cell, Cell::Empty)) {
                full_lines.push(y);
            }
        }

        if full_lines.is_empty() {
            return;
        }

        // Make the lines visualy cleared
        for &y in &full_lines {
            for x in 0..self.board.width as usize {
                self.playfield_mtrx[y][x] = Cell::Clearing;
            }
        }
        // force drawing
        //self.draw();
        //std::thread::sleep(std::time::Duration::from_millis(150));

        // clean the lines
        self.playfield_mtrx.retain(|row| {
            // discard the cleared ones
            !row.iter().all(|cell| matches!(cell, Cell::Clearing))
        });

        let deleted_count = full_lines.len();
        // change score
        self.score += match deleted_count {
            2 => 100 * (self.level as u32 + 1),
            3 => 300 * (self.level as u32 + 1),
            4 => 1200 * (self.level as u32 + 1),
            1 | _ => 40 * (self.level as u32 + 1),
        };
        // Append to the start the new rows
        for _ in 0..deleted_count {
            let new_row = vec![Cell::Empty; self.board.width as usize];
            self.playfield_mtrx.insert(0, new_row);
        }
    }
}
