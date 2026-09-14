use crate::{
    cards::{ deck::Deck, tableau::Tableau },
    lib_prelude::*,
};

pub struct Canfield {
    talon: Deck<FlippableCard<FrenchCard>>,
    stock: Deck<FlippableCard<FrenchCard>>,
    tableau: Tableau<FlippableCard<FrenchCard>>,
    foundation: Tableau<FlippableCard<FrenchCard>>,
}

impl Canfield {
    pub fn new_game_default() -> Self {
        Canfield {
            talon: Deck::new_standard_french_deck(true, true),
            stock: Deck::new_empty(),
            tableau: Tableau::new(CardStackVariant::VerticalTtB, 4),
            foundation: Tableau::new(CardStackVariant::Flush, 4),
        }
    }

    pub fn init_game(&mut self) {
        // Gather all cards from other regions into talon for shuffling and redistribution
        self.talon.replenish_default().expect("Talon is initialized with default discard");
        self.talon.add_cards(&mut self.tableau.gather_all());
        self.talon.add_cards(&mut self.foundation.gather_all());
        self.talon.replenish_from(&mut self.stock);

        // Ensure all cards are face-down before shuffling
        self.talon.all_face_down();

        // Shuffle talon (deck)
        self.talon.shuffle();

        // Draw 13 cards from talon to become the new stock pile. Flip the top card face-up
        self.stock.add_cards(&mut self.talon.draw_n(13).expect("Talon should not be emptied in initial setup"));
        self.stock.top_face_up().expect("Stock should not be empty, it was just dealt 13 cards");

        // Draw one card from talon to be first foundation card
        let mut c = self.talon.draw_card().expect("Talon should not be emptied in initial setup");
        c.flip_face_up();
        self.foundation.play_card_to_stack(c, 0).expect("Foundation stack 0 should exist");

        // Play one card from talon to each tableau stack
        for i in 0..4 {
            let mut c = self.talon.draw_card().expect("Talon should not be emptied in initial setup");
            c.flip_face_up();
            self.tableau.play_card_to_stack(c, i).expect(&format!("Tableau stack {} should exist", i));
        }
    }

    pub fn draw_talon(&mut self) -> Result<(), crate::cards::deck::DeckError> {
        if self.talon.view_inner_deck().len() > 0 {
            for _ in 0..3 {
                self.talon.top_deck_discard_default_flip()?;
            }
            Ok(())
        } else {
            self.talon.replenish_default()?;
            if self.talon.view_inner_deck().len() > 0 {
                self.talon.all_face_down();
                self.talon.reverse();
                self.draw_talon()
            } else {
                Ok(())
            }
        }
    }

    pub fn run_game(&mut self) {
        todo!();
    }
}

impl std::fmt::Debug for Canfield {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}\n\n{}\n\n{:?}\n{:?}",
            self.talon, self.stock, self.tableau, self.foundation
        )
    }
}