//use rand::Rng;

#[derive(Debug, Clone, Copy)]
pub enum TypeTetromino {
    I, // 4 Cell bar
    O, // Square
    T,
    S,
    Z,
    J,
    L,
}

#[derive(Debug, Clone, Copy)]
pub struct Tetromino {
    pub t_type: TypeTetromino,
    pub x: i32,
    pub y: i32,
}

impl Tetromino {
    pub fn new(t_type: TypeTetromino) -> Self {
        Self { t_type, x: 5, y: 4 }
    }
    pub fn shape(&self) -> [(i32, i32); 4] {
        match self.t_type {
            TypeTetromino::I => [(0, -1), (0, 0), (0, 1), (0, 2)],
            TypeTetromino::O => [(0, 0), (1, 0), (0, 1), (1, 1)],
            TypeTetromino::T => [(-1, 0), (0, 0), (1, 0), (0, 1)],
            TypeTetromino::S => [(0, 0), (1, 0), (-1, 1), (0, 1)],
            TypeTetromino::Z => [(-1, 0), (0, 0), (0, 1), (1, 1)],
            TypeTetromino::J => [(0, -1), (0, 0), (-1, 1), (0, 1)],
            TypeTetromino::L => [(0, -1), (0, 0), (1, 1), (0, 1)],
        }
    }
}
