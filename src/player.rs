use crate::{
    prelude::*,
    card::Card,
    deck::Deck,
};

#[derive(Debug)]
pub struct Player<CardSet: Card> {
    pub name: String,
    hand: Vec<CardSet>,
}

impl<CardSet: Card> Player<CardSet> {
    pub fn new(name: String) -> Self {
        Player { name, hand: vec!() }
    }

    pub fn draw_from(&mut self, deck: &mut Deck<CardSet>) {
        if let Ok(card) = deck.draw() {
            self.hand.push(card);
        } else {
            deck.replenish(None, false);
            deck.shuffle();
            self.hand.push(deck.draw().expect("Empty deck"));
        }
    }

    pub fn play_card_to(&mut self, card: CardSet, playable: &mut impl PlayableTo<CardSet>) {
        playable.play_to(card);
    }

    pub fn discard_to(&mut self, card: CardSet, discard: &mut Deck<CardSet>) {
        discard.push(card);
    }
}