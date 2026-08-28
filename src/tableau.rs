use crate::{
    prelude::*,
    stack::{Stack, StackVariant}
};

#[derive(Debug)]
pub struct Tableau<CardSet: Card> {
    tableau: Vec<Stack<CardSet>>,
}

impl<CardSet: Card> Tableau<CardSet> {
impl<CardSet: Card> PlayableTo<CardSet> for Tableau<CardSet> {
    fn play_to(&mut self, card: CardSet) -> Result<(), CardSet> {
        self.tableau[self.selected_stack].play_to(card)
    }
}