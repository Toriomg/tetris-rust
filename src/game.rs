use crate::cell::Cell;
use crate::tetromino::{Tetromino, TypeTetromino};
use rand::Rng;
use std::collections::VecDeque;

pub const PREVIEW_COUNT: usize = 3;
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
        let following_pieces = if PREVIEW_COUNT == 0 { 1 } else { PREVIEW_COUNT };
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
                    self.state = State::Spawning(0);
                } else {
                    // New piece
                    self.process_line_clearing();
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
        self.mark_full_lines();
    }

    fn mark_full_lines(&mut self) {
        // Identify lines where every cell is not empty
        for row in self.playfield_mtrx.iter_mut() {
            if row.iter().all(|cell| !matches!(cell, Cell::Empty)) {
                // Set all cells in the full row to Clearing state
                for cell in row.iter_mut() {
                    *cell = Cell::Clearing;
                }
            }
        }
    }

    fn process_line_clearing(&mut self) {
        let mut cleared_count = 0;

        // Count and remove rows that contain Clearing cells
        self.playfield_mtrx.retain(|row| {
            let is_clearing = row.iter().any(|cell| matches!(cell, Cell::Clearing));
            if is_clearing {
                cleared_count += 1;
                false // Remove the row
            } else {
                true // Keep the row
            }
        });

        if cleared_count == 0 {
            return;
        }

        // Calculate score based on lines cleared and current level
        let multiplier = self.level as u32 + 1;
        self.score += match cleared_count {
            1 => SCORE_SINGLE_LINE * multiplier,
            2 => SCORE_DOUBLE_LINE * multiplier,
            3 => SCORE_TRIPLE_LINE * multiplier,
            4 => SCORE_TETRIS_LINE * multiplier,
            _ => 0,
        };

        // Refill the top of the matrix with new empty rows
        for _ in 0..cleared_count {
            let new_row = vec![Cell::Empty; self.board.width as usize];
            self.playfield_mtrx.insert(0, new_row);
        }
    }
}
