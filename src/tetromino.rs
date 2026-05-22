use rand::Rng;

#[allow(dead_code)] //quitar es q quedan feos de ver lso errores
#[derive(Clone)]
pub enum Tetromino {
    I, // 4 Cell bar
    O, // Square
    T,
    S,
    Z,
    J,
    L,
}

#[derive(Clone)]
pub enum Cell {
    Empty,
    Taken(Tetromino),
}

impl Cell{
    pub fn draw(&self) {
        //print!("█▓▒");
        let is_even: bool = rand::thread_rng().gen_range(1..=100) % 2 == 0;
                let cell: &str = if is_even { "█" } else { " " };
                print!("[{cell}]");
        /*
        match self {
            Cell::Empty => print!("[ ]"),
            Cell::Taken(t) => {
                print!("[■]");
            }
        }
        */
    }
}