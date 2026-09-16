pub use std::io::{ Write, Stdout };
use crossterm::{ terminal::{ enable_raw_mode, disable_raw_mode }, };
use crate::lib_prelude::*;

pub fn enter_game_screen_imm(stdout: &mut Stdout) -> Result<(), std::io::Error> {
    enable_raw_mode()?;
    execute!(stdout, terminal::EnterAlternateScreen, terminal::DisableLineWrap, cursor::Hide)
}

pub fn leave_game_screen_imm(stdout: &mut Stdout) -> Result<(), std::io::Error> {
    execute!(stdout, cursor::Show, terminal::EnableLineWrap, terminal::LeaveAlternateScreen)?;
    disable_raw_mode()
}

pub fn clear_screen_imm(stdout: &mut Stdout) -> Result<(), std::io::Error> {
    execute!(stdout, terminal::Clear(terminal::ClearType::All), cursor::MoveTo(0, 0))
}

pub fn format_box_message(str: &str, w: usize, text_centred: bool) -> String {
    if text_centred {
        format!("{}\n{}{}\n{}\n", "-".repeat(w), " ".repeat(w.saturating_sub(str.len())/2), str, "-".repeat(w))
    } else {
        format!("{}\n{}\n{}\n", "-".repeat(w), str, "-".repeat(w))
    }
}

pub fn draw_box_message_que(stdout: &mut Stdout, str: &str, w: usize, text_centred: bool) -> Result<(), std::io::Error> {
    let w = w.max(str.len());
    if text_centred {
        queue!(stdout,
            style::Print(format!("{}", "-".repeat(w))),
            cursor::MoveToNextLine(1),
            style::Print(format!("{}{}", " ".repeat(w.saturating_sub(str.len())/2), str)),
            terminal::Clear(terminal::ClearType::UntilNewLine),
            cursor::MoveToNextLine(1),
            style::Print(format!("{}", "-".repeat(w))),
        )
    } else {
        queue!(stdout,
            style::Print(format!("{}", "-".repeat(w))),
            cursor::MoveToNextLine(1),
            style::Print(format!("{}", str)),
            terminal::Clear(terminal::ClearType::UntilNewLine),
            cursor::MoveToNextLine(1),
            style::Print(format!("{}", "-".repeat(w))),
        )
    }
}

pub fn draw_box_message_imm(stdout: &mut Stdout, str: &str, w: usize, text_centred: bool) -> Result<(), std::io::Error> {
    draw_box_message_que(stdout, str, w, text_centred)?;
    stdout.flush()
}

pub fn format_centred_message(str: &str, w: usize) -> String {
    let w = w.max(str.len());
    format!("{}{}{}", " ".repeat((w.saturating_sub(str.len()))/2), str, " ".repeat((w.saturating_sub(str.len()))/2))
}
