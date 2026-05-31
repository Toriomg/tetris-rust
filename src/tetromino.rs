use rand::Rng;

#[derive(Debug, Clone, Copy, PartialEq)]
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

    pub fn get_base_shape(&self) -> [(i32, i32); 4] {
        match self {
            TypeTetromino::I => [(-1, 0), (0, 0), (1, 0), (2, 0)],
            TypeTetromino::O => [(0, 0), (1, 0), (0, 1), (1, 1)],
            TypeTetromino::T => [(-1, 0), (0, 0), (1, 0), (0, 1)],
            TypeTetromino::S => [(0, 0), (1, 0), (-1, 1), (0, 1)],
            TypeTetromino::Z => [(-1, 0), (0, 0), (0, 1), (1, 1)],
            TypeTetromino::J => [(0, -1), (0, 0), (-1, 1), (0, 1)],
            TypeTetromino::L => [(0, -1), (0, 0), (1, 1), (0, 1)],
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Tetromino {
    pub t_type: TypeTetromino,
    pub x: i32,
    pub y: i32,
    pub rotation: i32,
}

impl Tetromino {
    pub fn new(t_type: TypeTetromino, playfield_width: u32) -> Self {
        Self {
            t_type,
            x: (playfield_width as i32) / 2 - 1, // Minus 1 as it starts in 0
            y: 0,
            rotation: 0,
        }
    }

    pub fn shape(&self) -> [(i32, i32); 4] {
        let base = self.t_type.get_base_shape();
        base.map(|(x, y)| match self.rotation {
            0 | 360 => (x, y),
            90 => (-y, x),
            180 => (-x, -y),
            270 => (y, -x),
            _ => (x, y),
        })
    }

    pub fn rotate(&mut self) {
        match self.t_type {
            TypeTetromino::O => (), // The square piece does not rotate
            _ => self.rotation = (self.rotation + 90) % 360,
        }
    }
}
