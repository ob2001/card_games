#[allow(unused)]
use card_games::game::{klondike::KlondikeGame, canfield::Canfield};

fn main() {
    unsafe { std::env::set_var("RUST_BACKTRACE", "1"); }
    let mut game = KlondikeGame::new_game_default();
    println!("{:?}", game);
    game.init_game();
    println!("{:?}", game);
    let _ = game.run_game();
}
