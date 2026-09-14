pub mod cards;
pub mod game;
pub mod ui;

pub trait PlayTo<C: cards::Card> {
    fn play_to(&mut self, card: C) -> Result<(), C>;
}

#[allow(unused)]
pub(crate) mod lib_prelude {
    pub use std::fmt::{ Debug, Display };
    pub use crossterm::{ execute, queue, cursor, event::{ self, Event, KeyEvent, KeyEventKind, KeyCode, KeyModifiers },
        style::{ self, Stylize }, terminal };
    pub use crate::{
        PlayTo,
        cards::{
            Card, card_stack::CardStackVariant, five_crowns_card::FiveCrownsCard,
            french_card::FrenchCard, italian_card::ItalianCard, magic_card::MagicCardEntry,
            tarocchi_card::TarocchiCard, tarot_card::TarotCard, flippable_card::FlippableCard,
        },
    };
}

pub mod prelude {
    pub use crate::{
        PlayTo,
        cards::{
            Card, five_crowns_card::FiveCrownsCard, french_card::FrenchCard,
            italian_card::ItalianCard, magic_card::MagicCardEntry, tarocchi_card::TarocchiCard,
            tarot_card::TarotCard,
        },
    };
}
