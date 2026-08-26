pub mod card;
pub mod deck;
pub mod hand;
pub mod game;
pub mod player;
pub mod stack;
pub mod tableau;

pub trait PlayableTo {
    fn play_to(&mut self, card: card::CardSuitRank);
}