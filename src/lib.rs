pub mod cards;
pub mod game;
pub mod ui;

#[allow(unused)]
pub(crate) mod lib_prelude {
    pub use std::fmt::Debug;
    pub use crossterm::{ execute, queue, cursor, event, style, terminal };
    pub use crate::cards::{
        Card, Rank,
        deck::{ DeckError, DeckToggle },
        card_stack::{ CardStackError, CardStackVariant },
        tableau::{ TableauError, TableauVariant },
        french_card::FrenchCard, flippable_card::FlippableCard,
        italian_card::ItalianCard, five_crowns_card::FiveCrownsCard,
        tarocchi_card::TarocchiCard, tarot_card::TarotCard,
    };
}

pub mod prelude {
    pub use crate::cards::{
        Card, Rank,
        french_card::FrenchCard, flippable_card::FlippableCard,
        italian_card::ItalianCard, five_crowns_card::FiveCrownsCard,
        tarocchi_card::TarocchiCard, tarot_card::TarotCard,
    };
}
