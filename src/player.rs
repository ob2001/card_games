use std::ops::Index;

use crate::{
    prelude::*,
    card::Card,
    deck::Deck,
};

#[derive(Debug)]
pub struct Player<CardSet: Card> {
    pub name: String,
    hand: Vec<CardSet>,
    selected_card: usize,
}

impl<CardSet: Card> Player<CardSet> {
    pub fn new(name: String) -> Self {
        Player { name, hand: vec!(), selected_card: 0 }
    }

    pub fn set_selected_card(&mut self, i: usize) {
        if i < self.hand.len() {
            self.selected_card = i;
        }
    }

    pub fn inc_selected_card(&mut self) {
        self.selected_card = (self.selected_card + 1) % self.hand.len();
    }

    pub fn dec_selected_card(&mut self) {
        let mut tmp = false;
        if self.selected_card > isize::MAX as usize {
            self.selected_card -= isize::MAX as usize;
            tmp = true;
        }

        self.selected_card = self.selected_card.checked_sub(1).unwrap_or(self.hand.len().saturating_sub(1));

        if tmp {
            self.selected_card += isize::MAX as usize;
        }
    }

    pub fn select_hand_card(&mut self) -> CardSet {
        self.hand.remove(self.selected_card)
    }

    pub fn play_hand_card(&mut self, playable: &mut impl PlayTo<CardSet>) {
        match playable.play_to(self.select_hand_card()) {
            Err(err_card) => {
                self.hand.push(err_card);
            },
            _ => {},
        }
    }

    pub fn discard_to_deck(&mut self, card: CardSet, discard: &mut Deck<CardSet>) {
        discard.push(card);
    }
}