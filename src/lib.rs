pub mod cards;
pub mod game;
pub mod ui;

pub trait PlayTo<CardSet: cards::Card> {
    fn play_to(&mut self, card: CardSet) -> Result<(), CardSet>;
}

pub trait Flip {
    fn flip(&mut self);
    fn flip_face_up(&mut self);
    fn flip_face_down(&mut self);
}

pub mod prelude {
    pub use std::fmt::{Debug, Display};
    pub use crate::{
        PlayTo, Flip,
        cards::{
            Card,
            french_card::FrenchCard,
            italian_card::ItalianCard,
            tarocchi_card::TarocchiCard,
            tarot_card::TarotCard,
            magic_card::MagicCard,
            card_stack::StackVariant,
        },
    };
}