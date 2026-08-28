pub mod card;
pub mod deck;
pub mod player;
pub mod stack;
pub mod tableau;

pub mod game;
pub mod ui;

pub trait PlayableTo<CardSet: card::Card> {
    fn play_to(&mut self, card: CardSet) -> Result<(), CardSet>;
}

pub trait Flippable {
    fn flip(&mut self);
    fn flip_face_up(&mut self);
    fn flip_face_down(&mut self);
}

pub mod prelude {
    pub use std::fmt::{Debug, Display};
    pub use crate::{
        PlayableTo, Flippable,
        card::{Card, FrenchCard, ItalianCard},
        stack::StackVariant,
    };
}