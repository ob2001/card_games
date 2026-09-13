pub use std::io::{ Write, Stdout };
use crate::lib_prelude::*;

pub fn enter_game_screen(stdout: &mut Stdout) -> Result<(), std::io::Error> {
    execute!(stdout,
        terminal::EnterAlternateScreen,
        cursor::Hide,
    )
}

pub fn leave_game_screen(stdout: &mut Stdout) -> Result<(), std::io::Error> {
    execute!(stdout,
        cursor::Show,
        terminal::LeaveAlternateScreen
    )
}

pub fn clear_screen(stdout: &mut Stdout) -> Result<(), std::io::Error> {
    execute!(stdout,
        terminal::Clear(terminal::ClearType::All),
        cursor::MoveTo(0, 0),
    )
}

pub fn format_box_message(str: &str, w: usize, text_centred: bool) -> String {
    let w = w.max(str.len());
    if text_centred {
        format!("{}\n{}{}\n{}", "-".repeat(w), " ".repeat((w.saturating_sub(str.len()))/2), str, "-".repeat(w))
    } else {
        format!("{}\n{}\n{}", "-".repeat(w), str, "-".repeat(w))
    }
}

pub fn format_centred_message(str: &str, w: usize) -> String {
    let w = w.max(str.len());
    format!("{}{}{}", " ".repeat((w.saturating_sub(str.len()))/2), str, " ".repeat((w.saturating_sub(str.len()))/2))
}
