use crate::{
    prelude::*,
    card::FlippableCard,
    deck::Deck,
    player::Player,
    tableau::{Tableau, TableauVariant},
};

#[derive(Debug)]
pub struct Klondike {
    player: Player<FlippableCard<FrenchCard>>,
    talon: Deck<FlippableCard<FrenchCard>>,
    talon_discard: Deck<FlippableCard<FrenchCard>>,
    tableau: Tableau<FlippableCard<FrenchCard>>,
    foundation: Tableau<FlippableCard<FrenchCard>>,
}

impl Klondike {
    pub fn new_game_default() -> Self {
        Klondike { 
            player: Player::new(String::from("Player1")),
            talon: Deck::new_standard_french_deck(),
            talon_discard: Deck::new_empty(),
            tableau: Tableau::new(Some(TableauVariant::Horizontal), Some(StackVariant::FanVertical), 7),
            foundation: Tableau::new(Some(TableauVariant::Horizontal), Some(StackVariant::Flush), 4),
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