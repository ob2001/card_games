#[allow(unused)]
use card_games::game::{klondike::KlondikeGame, canfield::Canfield};

fn main() {
    unsafe { std::env::set_var("RUST_BACKTRACE", "1"); }
    let mut game = KlondikeGame::new();
    game.init();
    let _ = game.run_game();
}
