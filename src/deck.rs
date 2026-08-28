use crate::{
    prelude::*,
    card::{FlippableCard, FrenchCard}
};

#[derive(Clone, Debug)]
pub struct Deck<CardSet: Card> {
    deck: Vec<CardSet>,
    default_discard: Option<Vec<CardSet>>,
}

impl<CardSet: Card> Deck<CardSet> {
    pub fn new_empty() -> Self {
        Deck { deck: vec![], default_discard: None }
    }

    pub fn draw(&mut self) -> Result<CardSet, &str> {
        self.deck.pop().ok_or("")
    }

    pub fn shuffle(&mut self) {
        for i in (1..self.deck.len()).rev() {
            let j = rand::random_range(0..i + 1);
            self.deck.swap(i, j)
        }
    }

    pub fn replenish(&mut self, other: Option<&mut Deck<CardSet>>, force_other: bool) {
        if let Some(discard) = &mut self.default_discard && !force_other {
            discard.append(&mut self.deck);
            self.deck = discard.to_vec();
        } else if let Some(other) = other {
            other.deck.append(&mut self.deck);
            self.deck = other.deck.to_vec();
        } else {
            panic!("Deck empty with no source to replenish");
        }
    }

    pub fn push(&mut self, card: CardSet) {
        self.deck.push(card);
    }

    pub fn get_inner_deck(&self) -> &Vec<CardSet> {
        &self.deck
    }

    pub fn get_inner_deck_mut(&mut self) -> &mut Vec<CardSet> {
        &mut self.deck
    }
}

impl<C: Card + Flippable> Deck<C> {
    pub fn all_face_up(&mut self) {
        for c in self.deck.iter_mut() {
            c.flip_face_up();
        }
    }

    pub fn all_face_down(&mut self) {
        for c in self.deck.iter_mut() {
            c.flip_face_down();
        }
    }
}

impl Deck<FlippableCard<FrenchCard>> {
    pub fn new_standard_french_deck() -> Deck<FlippableCard<FrenchCard>> {
        use crate::card::{FlippableCard, FrenchCard::*, FrenchRank::*};
        let mut deck = vec![];
        for i in 1..11 {
            for card in [Spades(Pip(i)), Hearts(Pip(i)), Clubs(Pip(i)), Diamonds(Pip(i))] {
                deck.push(FlippableCard::new(card.clone()));
            }
        }

        for rank in [Jack, Queen, King] {
            for card in [Spades(rank), Hearts(rank), Clubs(rank), Diamonds(rank)] {
                deck.push(FlippableCard::new(card.clone()));
            }
        }

        Deck { deck, default_discard: Some(vec![]) }
    }
}

impl<C: Card> Display for Deck<C> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for card in &self.deck {
            writeln!(f, "{}", card)?;
        }
        Ok(())
    }
}