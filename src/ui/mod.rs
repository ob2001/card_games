pub mod prelude {
    pub use crossterm::{self, cursor, event, execute, queue, style, terminal};
    pub use ratatui::{Terminal, backend::CrosstermBackend, prelude::*};
    pub use std::io::{stderr, stdin, stdout};
}

pub fn highlight_print_string(str: &str) {
    let l = str.len();
    print!("{}\n{}\n{}\n", "-".repeat(l), str, "-".repeat(l));
}
