use std::io::{self, Write};


#[macro_export]
macro_rules! println_raw {
    ($($arg:tt)*) => {
        print!("{}\r\n", format_args!($($arg)*));
    };
}
pub fn clear_screen() {
    /*
    \x1B[2J clean screen
    \x1B[1;1H set the cursor to the start of the screen
    */
    print!("{}[2J{}[1;1H", 27 as char, 27 as char);

    io::stdout().flush().unwrap();
}
