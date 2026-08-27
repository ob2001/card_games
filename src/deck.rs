use crate::{
    prelude::*,
    card::SuitRankCard
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

impl Deck<FrenchCard> {
    pub fn new_standard_french_deck() -> Deck<FrenchCard> {
        use crate::card::{SuitRankCard, FrenchRank::{self, *}, FrenchSuit::*};
        let mut deck = vec![];
        for suit in [Spades, Hearts, Clubs, Diamonds] {
            for i in 1..11 {
                deck.push(SuitRankCard::new(suit.clone(), FrenchRank::Pip(i)));
            }

            for rank in [Jack, Queen, King] {
                deck.push(SuitRankCard::new(suit.clone(), rank));
            }
        }

        Deck { deck, default_discard: Some(vec![]) }
    }
}

impl<S: crate::card::Suit, R: crate::card::Rank> Display for Deck<SuitRankCard<S, R>> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for card in &self.deck {
            writeln!(f, "{}", card)?;
        }
        Ok(())
    }
}