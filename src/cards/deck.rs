use crate::{
    cards::{FlippableCard, french_card::FrenchCard},
    lib_prelude::*,
};

#[derive(Clone, Debug)]
pub enum DeckError {
    NoValidCard,
    DrawOnEmptyDeck,
    NoDefaultDiscard,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DeckToggle {
    Deck,
    Discard,
}

impl DeckToggle {
    pub fn toggle(&mut self) {
        *self = match self {
            Self::Deck => Self::Discard,
            Self::Discard => Self::Deck,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Deck<CardSet: Card> {
    deck: Vec<CardSet>,
    default_discard: Option<Vec<CardSet>>,
    display_discard: bool,
    hovered: Option<DeckToggle>,
    // top_deck_hovered: bool,
    // top_discard_hovered: bool,
}

impl<CardSet: Card> Deck<CardSet> {
    pub fn new_empty() -> Self {
        Deck {
            deck: vec![],
            default_discard: None,
            display_discard: false,
            hovered: None,
        }
    }

    pub fn new_with_default_discard(discard: Vec<CardSet>, display_discard: bool) -> Self {
        let mut ret = Self::new_empty();
        ret.default_discard = Some(discard);
        ret.display_discard = display_discard;
        ret
    }

    pub fn draw(&mut self) -> Option<CardSet> {
        self.deck.pop()
    }

    pub fn draw_n(&mut self, n: usize) -> Result<Vec<CardSet>, Vec<CardSet>> {
        if self.deck.len() >= n {
            Ok(self.deck.split_off(self.deck.len() - n))
        } else {
            Err(self.deck.split_off(0))
        }
    }

    pub fn discard_default(&mut self) -> Result<(), DeckError> {
        match &mut self.default_discard {
            None => Err(DeckError::NoDefaultDiscard),
            Some(d) => {
                if let Some(c) = self.deck.pop() {
                    d.push(c);
                    Ok(())
                } else {
                    Err(DeckError::DrawOnEmptyDeck)
                }
            }
        }
    }

    pub fn discard_to(&mut self, other: &mut Deck<CardSet>) -> Result<(), DeckError> {
        if let Some(c) = self.draw() {
            other.take_cards(&mut vec![c]);
            Ok(())
        } else {
            Err(DeckError::DrawOnEmptyDeck)
        }
    }

    pub fn peek_top(&self) -> Option<&CardSet> {
        self.deck.last()
    }

    pub fn peek_bottom(&self) -> Option<&CardSet> {
        self.deck.first()
    }

    /// Randomize the positions of `Card`s in the `Deck`
    pub fn shuffle(&mut self) {
        // Fisher-Yates shuffling algorithm
        for i in (1..self.deck.len()).rev() {
            let j = rand::random_range(0..i + 1);
            self.deck.swap(i, j)
        }
    }

    pub fn replenish_default(&mut self) -> Result<(), DeckError> {
        if let Some(default_discard) = &mut self.default_discard {
            default_discard.append(&mut self.deck);
            self.deck = default_discard.split_off(0);
            Ok(())
        } else {
            Err(DeckError::NoDefaultDiscard)
        }
    }

    pub fn replenish_from(&mut self, other: &mut Deck<CardSet>) {
        other.deck.append(&mut self.deck);
        self.deck = other.deck.split_off(0);
    }

    pub fn reverse(&mut self) {
        self.deck.reverse();
    }

    pub fn take_cards(&mut self, cards: &mut Vec<CardSet>) {
        self.deck.append(cards);
    }

    pub fn push(&mut self, card: CardSet) {
        self.deck.push(card);
    }

    pub fn inner_deck(&self) -> &Vec<CardSet> {
        &self.deck
    }

    pub fn inner_deck_mut(&mut self) -> &mut Vec<CardSet> {
        &mut self.deck
    }

    pub fn is_hovered(&self) -> Option<DeckToggle> {
        self.hovered
    }

    pub fn toggle_deck_hover(&mut self) {
        if let Some(h) = &mut self.hovered {
            h.toggle()
        }
    }

    pub fn hover_discard(&mut self) {
        if self.default_discard.is_some() {
            self.hovered = Some(DeckToggle::Discard);
        }
    }
    
    pub fn hover_deck(&mut self) {
        self.hovered = Some(DeckToggle::Deck);
    }

    pub fn unhover(&mut self) {
        self.hovered = None;
    }

    pub fn get_top_discard(&mut self) -> Option<CardSet> {
        if let Some(dc) = &mut self.default_discard {
            dc.pop()
        } else {
            None
        }
    }

    pub fn play_card_to_discard(&mut self, card: CardSet) -> Result<(), CardSet> {
        if let Some(dc) = &mut self.default_discard {
            dc.push(card);
            Ok(())
        } else {
            Err(card)
        }
    }

    pub fn play_cards_to_discard(&mut self, cards: &mut Vec<CardSet>) -> Result<(), DeckError> {
        if let Some(dc) = &mut self.default_discard {
            dc.append(cards);
            Ok(())
        } else {
            Err(DeckError::NoDefaultDiscard)
        }
    }
}

impl<CardSet: Card> Deck<FlippableCard<CardSet>> {
    pub fn discard_default_flip(&mut self) -> Result<(), DeckError> {
        match &mut self.default_discard {
            None => Err(DeckError::NoDefaultDiscard),
            Some(d) => {
                if let Some(mut c) = self.deck.pop() {
                    c.flip_face_up();
                    d.push(c);
                    Ok(())
                } else {
                    Err(DeckError::DrawOnEmptyDeck)
                }
            }
        }
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

    pub fn top_face_up(&mut self) -> Result<(), DeckError> {
        if let Some(c) = self.deck.last_mut() {
            c.flip_face_up();
            Ok(())
        } else {
            Err(DeckError::NoValidCard)
        }
    }

    pub fn top_face_down(&mut self) -> Result<(), DeckError> {
        if let Some(c) = self.deck.last_mut() {
            c.flip_face_down();
            Ok(())
        } else {
            Err(DeckError::NoValidCard)
        }
    }

    pub fn flip_top(&mut self) -> Result<(), DeckError> {
        if let Some(c) = self.deck.last_mut() {
            c.flip();
            Ok(())
        } else {
            Err(DeckError::NoValidCard)
        }
    }
}

impl<CardSet: Card> std::iter::Iterator for Deck<CardSet> {
    type Item = CardSet;
    fn next(&mut self) -> Option<Self::Item> {
        self.draw()
    }
}

impl Deck<FlippableCard<FrenchCard>> {
    pub fn new_standard_french_deck(default_discard: bool, display_discard: bool) -> Deck<FlippableCard<FrenchCard>> {
        use crate::cards::FlippableCard;
        use crate::cards::french_card::FrenchRank::*;
        use FrenchCard::*;

        let mut deck = vec![];
        for i in 1..11 {
            for card in [
                Spades(Pip(i)),
                Hearts(Pip(i)),
                Clubs(Pip(i)),
                Diamonds(Pip(i)),
            ] {
                deck.push(FlippableCard::new(card.clone()));
            }
        }

        for rank in [Jack, Queen, King] {
            for card in [Spades(rank), Hearts(rank), Clubs(rank), Diamonds(rank)] {
                deck.push(FlippableCard::new(card.clone()));
            }
        }

        let default_discard = if default_discard {
            Some(vec![])
        } else {
            None
        };

        Deck {
            deck,
            default_discard,
            display_discard,
            hovered: None,
        }
    }

    pub fn new_joker_french_deck(
        default_discard: Option<Vec<FlippableCard<FrenchCard>>>,
        display_discard: bool
    ) -> Deck<FlippableCard<FrenchCard>> {
        use crate::cards::FlippableCard;
        use crate::cards::french_card::FrenchRank::*;
        use FrenchCard::*;

        let mut deck = vec![];
        for i in 1..11 {
            for card in [
                Spades(Pip(i)),
                Hearts(Pip(i)),
                Clubs(Pip(i)),
                Diamonds(Pip(i)),
            ] {
                deck.push(FlippableCard::new(card.clone()));
            }
        }

        for rank in [Jack, Queen, King] {
            for card in [Spades(rank), Hearts(rank), Clubs(rank), Diamonds(rank)] {
                deck.push(FlippableCard::new(card.clone()));
            }
        }

        deck.push(FlippableCard::new(FrenchCard::Joker));
        deck.push(FlippableCard::new(FrenchCard::Joker));

        Deck {
            deck,
            default_discard,
            display_discard,
            hovered: None,
        }
    }
}

impl<C: Card> Display for Deck<C> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for card in self.deck.iter().take(self.deck.len().saturating_sub(1)) {
            write!(f, "{} ", card)?;
        }
        if let Some(c) = self.deck.last() {
            if self.hovered == Some(DeckToggle::Deck) {
                write!(f, "\x1b[100m{}\x1b[40m", c)?;
            } else {
                write!(f, "{}", c)?;
            }
        }

        if self.display_discard && let Some(d_d) = &self.default_discard {
            write!(f, "\n=> ")?;
            for card in d_d.iter().take(d_d.len().saturating_sub(1)) {
                write!(f, "{} ", card)?;
            }
            if let Some(c) = d_d.last() {
                if self.hovered == Some(DeckToggle::Discard) {
                    write!(f, "\x1b[100m{}\x1b[40m", c)?;
                } else {
                    write!(f, "{}", c)?;
                }
            }
        }

        Ok(())
    }
}
