use crate::tetromino::TypeTetromino;

#[derive(Debug, Clone, Copy)]
pub enum Cell {
    Empty,
    Taken(TypeTetromino),
    Clearing,
}

impl Cell {
    pub fn draw(&self) {
        match self {
            Cell::Empty => print!("  "),
            Cell::Clearing => print!("\x1b[47m  \x1b[0m"),
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
                print!("{}██{}", color, reset);
            }
        }
    }
}
