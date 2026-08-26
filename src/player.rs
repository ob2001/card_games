use crate::{
    PlayableTo, card::CardSuitRank, deck::Deck, hand::Hand,
};

#[derive(Debug)]
pub struct Player {
    name: String,
    hand: Hand,
}

impl Player {
    pub fn new(name: String) -> Self {
        Player { name, hand: Hand::new_empty() }
    }

    pub fn draw_from(&mut self, deck: &mut Deck<CardSuitRank>) {
        if let Ok(card) = deck.draw() {
            self.hand.hand.push(card);
        } else {
            deck.replenish(None, false);
            deck.shuffle();
            self.hand.hand.push(deck.draw().expect("Empty deck"));
        }
    }

    pub fn play_card_to(&mut self, card: CardSuitRank, playable: &mut impl PlayableTo) {
        playable.play_to(card);
    }

    pub fn discard_to(&mut self, card: CardSuitRank, discard: &mut Deck<CardSuitRank>) {
        discard.push(card);
    }
}