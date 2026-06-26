use crate::game::Actions;
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    terminal::{disable_raw_mode, enable_raw_mode},
};
use std::time::Duration;

pub fn init_terminal() {
    enable_raw_mode().expect("Failed to enable raw mode");
}

pub fn cleanup_terminal() {
    disable_raw_mode().expect("Failed to disable raw mode");
}

pub fn poll_action() -> Option<Actions> {
    // Non-blocking check for events
    if event::poll(Duration::from_millis(0)).ok()? {
        if let Event::Key(key) = event::read().ok()? {
            // Only process the press event (ignore release/repeat)
            if key.kind == KeyEventKind::Press {
                return match key.code {
                    KeyCode::Left => Some(Actions::Left),
                    KeyCode::Right => Some(Actions::Right),
                    KeyCode::Down => Some(Actions::Down),
                    KeyCode::Char('z') | KeyCode::Up => Some(Actions::Rotate),
                    KeyCode::Esc | KeyCode::Char('q') => Some(Actions::Quit),
                    KeyCode::Char('p') => Some(Actions::Pause),
                    _ => None,
                };
            }
        }
    }
    None
}
