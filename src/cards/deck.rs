use std::io::{ Stdout, Write };

use crate::{
    cards::{ FlippableCard, french_card::FrenchCard },
    lib_prelude::*,
};

/// Errors originating from Deck functionality
#[derive(Clone, Debug)]
pub enum DeckError {
    NoValidCard,
    DrawOnEmptyDeck,
    NoDefaultDiscard,
}

/// Enum for tracking whether the deck or default discard pile
/// is curently hovered
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DeckToggle {
    Deck,
    Discard,
}

impl DeckToggle {
    /// Convenience function to toggle between the two DeckToggle options
    pub fn toggle(&mut self) {
        *self = match self {
            Self::Deck => Self::Discard,
            Self::Discard => Self::Deck,
        }
    }
}

/// A structure containing a collection of cards which may be shuffled, drawn from, and discarded from/to
#[derive(Clone, Debug)]
pub struct Deck<C: Card> {
    deck: Vec<C>,
    default_discard: Option<Vec<C>>,
    display_discard: bool,
    hovered: Option<DeckToggle>,

    /// Only set if this element has no parent
    pos: Option<(u16, u16)>,
}

impl<C: Card> Deck<C> {
    /// Create a new deck containing no cards
    pub fn new_empty(pos: Option<(u16, u16)>) -> Self {
        Deck {
            deck: vec![],
            default_discard: None,
            display_discard: false,
            hovered: None,
            pos,
        }
    }

    /// Create a new empty deck with a default discard pile,
    /// and with an option to either display or hide the discard pile
    pub fn new_with_default_discard(discard: Vec<C>, display_discard: bool, pos: Option<(u16, u16)>) -> Self {
        let mut ret = Self::new_empty(pos);
        ret.default_discard = Some(discard);
        ret.display_discard = display_discard;
        ret
    }

    pub fn draw_imm(&self, stdout: &mut Stdout) -> Result<(), std::io::Error> {
        self.draw_que(stdout)?;
        stdout.flush()
    }

    pub fn set_pos(&mut self, pos: Option<(u16, u16)>) {
        self.pos = pos
    }

    pub fn get_pos(&self) -> Option<(u16, u16)> {
        self.pos
    }

    pub fn get_pos_mut(&mut self) -> Option<&mut (u16, u16)> {
        self.pos.as_mut()
    }

    pub fn draw_que(&self, stdout: &mut Stdout) -> Result<(), std::io::Error> {
        if self.deck.len() > 0 {
            for c in self.deck.iter().take(self.deck.len().saturating_sub(1)) {
                c.draw_que(stdout)?;
                queue!(stdout, cursor::MoveRight(1))?;
            }

            if self.hovered == Some(DeckToggle::Deck) {
                queue!(stdout, style::PrintStyledContent(format!("{}", self.deck.last().expect("Deck is guaranteed to have at least one card at this point")).on_dark_grey()))?;
            } else {
                self.deck.last().expect("Deck is guaranteed to have at least one card at this point").draw_que(stdout)?;
            }

            queue!(stdout, terminal::Clear(terminal::ClearType::UntilNewLine))?;
        } else if self.hovered == Some(DeckToggle::Deck) {
            queue!(stdout, style::PrintStyledContent("  ".on_dark_grey()), terminal::Clear(terminal::ClearType::UntilNewLine))?;
        } else {
            queue!(stdout, terminal::Clear(terminal::ClearType::UntilNewLine))?;
        }

        if self.display_discard {
            if let Some(dc) = &self.default_discard {
                if dc.len() > 0 {
                    queue!(stdout, cursor::MoveToNextLine(1), style::Print("=> "))?;
                    for c in dc.iter().take(dc.len().saturating_sub(1)) {
                        c.draw_que(stdout)?;
                        queue!(stdout, cursor::MoveRight(1))?;
                    }

                    if self.hovered == Some(DeckToggle::Discard) {
                        queue!(stdout, style::PrintStyledContent(format!("{}", dc.last().expect("Discard is guaranteed to have at least one card at this point")).on_dark_grey()))?;
                    } else {
                        dc.last().expect("Discard is guaranteed to have at least one card at this point").draw_que(stdout)?;
                    }

                    queue!(stdout, terminal::Clear(terminal::ClearType::UntilNewLine))?;
                }
            } else {
                if self.hovered == Some(DeckToggle::Discard) {
                    queue!(stdout, style::PrintStyledContent("  ".on_dark_grey()), terminal::Clear(terminal::ClearType::UntilNewLine))?;
                } else {
                    queue!(stdout, terminal::Clear(terminal::ClearType::UntilNewLine))?;
                }
            }
        }

        Ok(())
    }

    /// Draw a single card from the top (end) of the deck and return it
    /// to the function caller if it exists
    pub fn draw_card(&mut self) -> Option<C> {
        self.deck.pop()
    }

    /// Draw `n` cards from the top (end) of the deck and return them to the
    /// function caller. If the deck contains fewer than `n` cards before drawing,
    /// return the remainder of the Deck wrapped in an `Err` and allow the caller
    /// to decide how to proceed.
    pub fn draw_n(&mut self, n: usize) -> Result<Vec<C>, Vec<C>> {
        if self.deck.len() >= n {
            Ok(self.deck.split_off(self.deck.len() - n))
        } else {
            Err(self.deck.split_off(0))
        }
    }

    /// Place the top card of the deck immediately onto the top of
    /// its discard pile (if it has one).
    pub fn top_deck_discard_default(&mut self) -> Result<(), DeckError> {
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

    /// Place the top card of the deck immediately onto the top of the provided
    /// deck.
    pub fn top_deck_discard_to(&mut self, other: &mut Deck<C>) -> Result<(), DeckError> {
        if let Some(c) = self.draw_card() {
            other.add_cards(&mut vec![c]);
            Ok(())
        } else {
            Err(DeckError::DrawOnEmptyDeck)
        }
    }

    /// Borrow the top card of the deck, if it exists
    pub fn peek_top(&self) -> Option<&C> {
        self.deck.last()
    }

    /// Borrow the bottom card of the deck, if it exists
    pub fn peek_bottom(&self) -> Option<&C> {
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

    /// Return the contents of the deck's default discard pile to the bottom (beginning)
    /// of the deck.
    pub fn replenish_default(&mut self) -> Result<(), DeckError> {
        if let Some(default_discard) = &mut self.default_discard {
            default_discard.append(&mut self.deck);
            self.deck = default_discard.split_off(0);
            Ok(())
        } else {
            Err(DeckError::NoDefaultDiscard)
        }
    }

    /// Return the contents of the provided deck to the bottom (beginning) of
    /// the deck
    pub fn replenish_from(&mut self, other: &mut Deck<C>) {
        other.deck.append(&mut self.deck);
        self.deck = other.deck.split_off(0);
    }

    /// Reverse the order of the deck's contents
    pub fn reverse(&mut self) {
        self.deck.reverse();
    }

    /// Add the provided cards to the top of the deck
    pub fn add_cards(&mut self, cards: &mut Vec<C>) {
        self.deck.append(cards);
    }

    /// Push a single card onto the top of the deck
    pub fn push(&mut self, card: C) {
        self.deck.push(card);
    }

    /// Remove the entire contents of the deck and return them to the caller
    pub fn get_inner_deck(&mut self) -> Vec<C> {
        self.deck.split_off(0)
    }

    /// Borrow the deck
    pub fn inner_deck(&self) -> &Vec<C> {
        &self.deck
    }

    /// Mutably borrow the deck
    pub fn inner_deck_mut(&mut self) -> &mut Vec<C> {
        &mut self.deck
    }

    /// If the deck is hovered, return whether it is the deck or discard pile
    /// currently hovered over. If the deck is not hovered, return None
    pub fn is_hovered(&self) -> Option<DeckToggle> {
        self.hovered
    }

    /// Toggle between hovering over the deck and its the default discard (if it exists)
    pub fn toggle_deck_hover(&mut self) {
        if let Some(h) = &mut self.hovered {
            h.toggle()
        }
    }

    /// Hover over the discard pile (if it exists)
    pub fn hover_discard(&mut self) {
        if self.default_discard.is_some() {
            self.hovered = Some(DeckToggle::Discard);
        }
    }
    
    /// Hover over the deck
    pub fn hover_deck(&mut self) {
        self.hovered = Some(DeckToggle::Deck);
    }

    /// Completely unhover the Deck
    pub fn unhover(&mut self) {
        self.hovered = None;
    }

    /// Draw a single card from the top (end) of the default discard
    /// (if it exists)
    pub fn draw_discard(&mut self) -> Option<C> {
        if let Some(dc) = &mut self.default_discard {
            dc.pop()
        } else {
            None
        }
    }

    /// Place passed cards onto the top (end) of the default discard (if it exists)
    pub fn play_cards_to_discard(&mut self, cards: &mut Vec<C>) -> Result<(), DeckError> {
        if let Some(dc) = &mut self.default_discard {
            dc.append(cards);
            Ok(())
        } else {
            Err(DeckError::NoDefaultDiscard)
        }
    }
}

impl<C: Card> Deck<FlippableCard<C>> {
    /// Place the top card of the deck immediately onto the top of
    /// its discard pile (if it has one) flipped face-up
    pub fn top_deck_discard_default_flip(&mut self) -> Result<(), DeckError> {
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

    /// Flip all cards in the deck face-up
    pub fn all_face_up(&mut self) {
        for c in self.deck.iter_mut() {
            c.flip_face_up();
        }
    }

    /// Flip all cards in the deck face-down
    pub fn all_face_down(&mut self) {
        for c in self.deck.iter_mut() {
            c.flip_face_down();
        }
    }

    /// Flip only the top card of the deck face-up
    pub fn top_face_up(&mut self) -> Result<(), DeckError> {
        if let Some(c) = self.deck.last_mut() {
            c.flip_face_up();
            Ok(())
        } else {
            Err(DeckError::NoValidCard)
        }
    }

    /// Flip only the top card of the deck face-down
    pub fn top_face_down(&mut self) -> Result<(), DeckError> {
        if let Some(c) = self.deck.last_mut() {
            c.flip_face_down();
            Ok(())
        } else {
            Err(DeckError::NoValidCard)
        }
    }

    /// Toggle the flip state of the top card of the deck
    pub fn flip_top(&mut self) -> Result<(), DeckError> {
        if let Some(c) = self.deck.last_mut() {
            c.flip();
            Ok(())
        } else {
            Err(DeckError::NoValidCard)
        }
    }
}

impl<C: Card> std::iter::Iterator for Deck<C> {
    type Item = C;
    fn next(&mut self) -> Option<Self::Item> {
        self.draw_card()
    }
}

impl Deck<FlippableCard<FrenchCard>> {
    /// Generate and return a standard 52-card flippable French-style deck (no Jokers)
    pub fn new_standard_french_deck(default_discard: bool, display_discard: bool, pos: Option<(u16, u16)>) -> Deck<FlippableCard<FrenchCard>> {
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
            pos,
        }
    }

    /// Generate and return a 54-card French style deck with 2 Jokers
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
            pos: None,
        }
    }
}

impl<C: Card> Display for Deck<C> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.deck.len() == 0 && let Some(DeckToggle::Deck) = self.hovered {
            write!(f, "\x1b[100m  \x1b[40m")?;
        }
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
