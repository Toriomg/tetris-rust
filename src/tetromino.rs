use rand::Rng;

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

impl TypeTetromino {
    /**
     *  Returns a random tetromino type
     */
    pub fn random() -> Self {
        let mut rng = rand::thread_rng();
        match rng.gen_range(0..7) {
            0 => Self::I,
            1 => Self::O,
            2 => Self::T,
            3 => Self::S,
            4 => Self::Z,
            5 => Self::J,
            _ => Self::L, // Default case for safety
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Rotation {
    Deg0,
    Deg90,
    Deg180,
    Deg270,
}

#[derive(Debug, Clone, Copy)]
pub struct Tetromino {
    pub t_type: TypeTetromino,
    pub x: i32,
    pub y: i32,
    rotation: Rotation
}

impl Tetromino {
    pub fn new(t_type: TypeTetromino, playfield_width: u32) -> Self {
        Self {
            t_type,
            x: (playfield_width as i32) / 2 - 1, // Minus 1 as it starts in 0
            y: 1,
            rotation: Rotation::Deg0,
        }
    }
    pub fn shape(&self) -> [(i32, i32); 4] {
        // By the coordinates place the shape of the piece
        match self.t_type {
            TypeTetromino::I => [(-1, 0), (0, 0), (1, 0), (2, 0)],
            TypeTetromino::O => [(0, 0), (1, 0), (0, 1), (1, 1)],
            TypeTetromino::T => [(-1, 0), (0, 0), (1, 0), (0, 1)],
            TypeTetromino::S => [(0, 0), (1, 0), (-1, 1), (0, 1)],
            TypeTetromino::Z => [(-1, 0), (0, 0), (0, 1), (1, 1)],
            TypeTetromino::J => [(0, -1), (0, 0), (-1, 1), (0, 1)],
            TypeTetromino::L => [(0, -1), (0, 0), (1, 1), (0, 1)],
        }
    }
    pub fn rotate(&mut self){

    }
}
