use crate::cell::Cell;
use crate::game::{Game, State};
use std::io::{Write, stdout};

// Layout Constants
const SIDEBAR_WIDTH: usize = 12;
const ROWS_PER_PREVIEW: usize = 4;
const PREVIEW_X_OFFSET: i32 = 1;
const PREVIEW_Y_OFFSET: i32 = 1;
const PREVIEW_INTERNAL_GRID_WIDTH: usize = 5;
const SIDEBAR_GAP: &str = "  "; // Gap between board and sidebar
const EMPTY_CELL: &str = "  ";

pub fn draw_game(game: &Game) {
    crate::utils::clear_screen();

    let board_height = game.playfield_mtrx.len();
    let board_width = game.playfield_mtrx[0].len();

    draw_header(board_width, SIDEBAR_WIDTH, crate::game::PREVIEW_COUNT);

    for y in 0..board_height {
        draw_board_row(y, board_width, game);
        
        // Only draw sidebar if PREVIEW_COUNT > 0
        if crate::game::PREVIEW_COUNT > 0 {
            draw_sidebar_row(y, SIDEBAR_WIDTH, game);
        }
        
        crate::println_raw!("");
    }

    draw_footer(board_width, game.score);
    let _ = stdout().flush();
}

// Draws the top border of the game and sidebar
fn draw_header(board_width: usize, sidebar_width: usize, preview_count: usize) {
    // Board header
    print!("╔");
    let title = " GB TETRIS ";
    print!("{title}{}", "═".repeat(board_width * 2 - title.len()));

    if preview_count > 0 {
        print!("╗{}", SIDEBAR_GAP);
        print!("╔");
        let next_title = " NEXT ";
        print!("{next_title}{}", "═".repeat(sidebar_width - next_title.len()));
        crate::println_raw!("╗");
    } else {
        crate::println_raw!("╗");
    }
}

// Draws a single row of the playfield
fn draw_board_row(y: usize, width: usize, game: &Game) {
    print!("║");
    for x in 0..width {
        if is_active_piece_at(x as i32, y as i32, game) {
            let p_type = game.current_piece.as_ref().unwrap().t_type;
            Cell::Taken(p_type).draw();
        } else {
            game.playfield_mtrx[y][x].draw();
        }
    }
    
    // print gap iff sidebar exists
    if crate::game::PREVIEW_COUNT > 0 {
        print!("║{}", SIDEBAR_GAP);
    } else {
        print!("║");
    }
}

/// Draws a single row of the right sidebar
fn draw_sidebar_row(y: usize, width: usize, game: &Game) {
    let preview_idx = y / ROWS_PER_PREVIEW;
    let local_y = (y % ROWS_PER_PREVIEW) as i32;

    if preview_idx < game.next_pieces.len() {
        print!("║  ");
        let t_type = game.next_pieces[preview_idx];
        let shape = t_type.get_base_shape();

        for local_x in 0..PREVIEW_INTERNAL_GRID_WIDTH {
            let is_part = shape.iter().any(|(ox, oy)| {
                ox + PREVIEW_X_OFFSET == local_x as i32 && oy + PREVIEW_Y_OFFSET == local_y
            });

            if is_part { Cell::Taken(t_type).draw(); } else { print!("{}", EMPTY_CELL); }
        }
        print!("║");
    } else if y == (game.next_pieces.len() * ROWS_PER_PREVIEW) {
        print!("╚{}╝", "═".repeat(width));
    } else {
        // Line change: use repeat with width for consistency
        print!("  {}", " ".repeat(width));
    }
}

// Draws the bottom border and the score
fn draw_footer(board_width: usize, score: u32) {
    print!("╚");
    let score_str = format!(" SCORE: {score} ");
    print!(
        "{}{score_str}",
        "═".repeat(board_width * 2 - score_str.len())
    );
    crate::println_raw!("╝");
}

// Helper to check if the current active piece occupies a specific coordinate
fn is_active_piece_at(x: i32, y: i32, game: &Game) -> bool {
    if game.state != State::Playing {
        return false;
    }

    if let Some(piece) = &game.current_piece {
        return piece
            .shape()
            .iter()
            .any(|(ox, oy)| piece.x + ox == x && piece.y + oy == y);
    }
    false
}
