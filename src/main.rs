use card_games::{
    // lib_prelude::*,
    game::klondike::Klondike,
};

fn main() {
    french_cards_functionality_test();
}

fn french_cards_functionality_test() {
    let mut game = Klondike::new_game_default();
    card_games::ui::highlight_print_string("Created Klondike Game");
    println!("{:?}", game);
    card_games::ui::highlight_print_string("Initialized Klondike Game");
    let _ = game.init_game();
    println!("{:?}", game);
    let _ = game.run_game();
}
