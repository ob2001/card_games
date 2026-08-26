use crate::{
    card::CardSuitRank,
    deck::Deck,
    player::Player,
    tableau::Tableau,
};

#[derive(Debug)]
pub struct Klondike {
    player: Player,
    talon: Deck<CardSuitRank>,
    talon_discard: Deck<CardSuitRank>,
    tableau: Tableau<CardSuitRank>,
    foundation: Tableau<CardSuitRank>,
}

impl Klondike {
    pub fn new_game_default() -> Self {
        Klondike { 
            player: Player::new(String::from("Player1")),
            talon: Deck::new_standard_french_deck(),
            talon_discard: Deck::new_empty(),
            tableau: Tableau::new(7),
            foundation: Tableau::new(4),
        }
    }

    pub fn init_game(&mut self) {
        for c in self.talon.get_inner_deck_mut() {
            c.flip_face_down();
        }
        self.talon.shuffle();
    }

    pub fn run_game(&mut self) {
        todo!();
    }

}

impl std::fmt::Display for Klondike {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!("Display not yet implemented for Klondike");
    }    
}