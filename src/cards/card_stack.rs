use crate::{
    cards::card_stack::CardStackError::InvalidCardSelection,
    lib_prelude::*
};

/// A representation of a stack of cards which may be played to, without
/// the special functionality associated with a deck (e.g. drawing, shuffling)
/// Has an optional stack limit which allows the stack to reject being played
/// to if the played card would exceed its limit
#[derive(Clone)]
pub struct CardStack<CardSet: Card> {
    stack: Vec<CardSet>,
    stack_variant: StackVariant,
    lim: Option<usize>,
    hovered_card: Option<usize>,
}

/// Variant enum for use when displaying the CardStack. Indicates the order and direction
/// that the cards in the stack should be displayed.
#[derive(Clone, Debug)]
pub enum StackVariant {
    HorizontalLtR,
    HorizontalRtL,
    VerticalTtB,
    VerticalBtT,
    Flush,
}

/// Errors originating from CardStack functionality
#[derive(Clone, Debug)]
pub enum CardStackError {
    InvalidCardSelection,
    CardStackOverflow
}

impl<CardSet: Card> CardStack<CardSet> {
    /// Return a new, empty, unhovered, unlimited CardStack
    pub fn new(stack_variant: StackVariant, lim: Option<usize>) -> Self {
        CardStack {
            stack: vec![],
            stack_variant,
            lim,
            hovered_card: None,
        }
    }

    /// Return a new, unhovered, unlimited Cardstack containing the passed cards (in order)
    pub fn new_from(stack: Vec<CardSet>, stack_variant: StackVariant, lim: Option<usize>) -> Self {
        CardStack {
            stack,
            stack_variant,
            lim,
            hovered_card: None,
        }
    }

    /// Return the current length of the CardStack
    pub fn len(&self) -> usize {
        self.stack.len()
    }

    /// Collect all cards in the stack and return them to the caller
    pub fn gather_cards(&mut self) -> Vec<CardSet> {
        let ret = self.stack.drain(0..self.stack.len()).collect();
        ret
    }
    
    /// Return the display variant of the CardStack
    pub fn stack_variant(&self) -> &StackVariant {
        &self.stack_variant
    }

    /// Directly set which card in the stack is currently hovered over by index,
    /// if the given index exists
    pub fn set_hovered_card(&mut self, stack: usize) -> Result<(), CardStackError> {
        if stack <= self.stack.len() {
            self.hovered_card = Some(stack);
            Ok(())
        } else {
            Err(InvalidCardSelection)
        }
    }

    /// Borrow the last card on the CardStack (if it exists)
    pub fn last(&self) -> Option<&CardSet> {
        self.stack.last()
    }

    /// Mutably borrow the last card on the CardStack (if it exists)
    pub fn last_mut(&mut self) -> Option<&mut CardSet> {
        self.stack.last_mut()
    }

    /// Hover over the next card in the stack
    pub fn inc_hovered_card(&mut self) {
        if let Some(c) = self.hovered_card {
            self.hovered_card = Some((c + 1) % self.stack.len());
        }
    }

    /// Hover over the previous card in the stack
    pub fn dec_hovered_card(&mut self) {
        let mut tmp = false;

        if let Some(c) = self.hovered_card && c > isize::MAX as usize {
            self.hovered_card = Some(c - isize::MAX as usize);
            tmp = true;
        }
        
        if self.hovered_card != None {
            self.hovered_card = Some(self.hovered_card.unwrap()
                .checked_sub(1)
                .unwrap_or(self.stack.len().saturating_sub(1)));
        }

        if tmp && self.hovered_card != None {
            self.hovered_card = Some(self.hovered_card.unwrap() + isize::MAX as usize);
        }
    }

    /// Return the index of the currently hovered card
    pub fn get_hovered_card_idx(&self) -> Option<usize> {
        self.hovered_card
    }

    /// Borrow the currently hovered card
    pub fn get_hovered_card(&self) -> Option<&CardSet> {
        if let Some(c) = self.hovered_card {
            self.stack.get(c)
        } else {
            None
        }
    }

    /// Mutably borrow the currently hovered card
    pub fn get_hovered_card_mut(&mut self) -> Option<&mut CardSet> {
        if let Some(c) = self.hovered_card {
            Some(&mut self.stack[c])
        } else {
            None
        }
    }

    /// Remove the currently hovered card from the CardStack and return
    /// it to the caller
    pub fn take_hovered_card(&mut self) ->  Option<CardSet> {
        if let Some(c) = self.hovered_card {
            Some(self.stack.remove(c))
        } else {
            None
        }
    }

    /// Remove all cards from the currently hovered card (if it exists) to the end of the stack
    /// and return them (in order) to the caller
    pub fn take_hovered_stack(&mut self) -> Option<CardStack<CardSet>> {
        if let Some(c) = self.hovered_card {
            Some(self.stack.split_off(c).into())
        } else {
            None
        }
    }

    /// Hover over the last card in the CardStack
    pub fn hover(&mut self) {
        self.hovered_card = Some(self.stack.len().saturating_sub(1));
    }

    /// Unhover the CardStack
    pub fn unhover(&mut self) {
        self.hovered_card = None;
    }

    /// Set the limit of the CardStack
    pub fn set_lim(&mut self, lim: usize) {
        self.lim = Some(lim);
    }

    /// Remove the limit of the CardStack
    pub fn unset_lim(&mut self) {
        self.lim = None;
    }

    /// Return the limit of the CardStack, if it exists
    pub fn get_lim(&self) -> Option<usize> {
        self.lim
    }

    /// Borrow the card before the currently hovered card, if it exists
    pub fn peek_prev_card(&self) -> Option<&CardSet> {
        if let Some(c) = self.hovered_card && c > 0 {
            self.stack.get(c - 1)
        } else {
            None
        }
    }

    /// Mutably borrow the card before the currently hovered card, if it exists
    pub fn prev_card_mut(&mut self) -> Option<&mut CardSet> {
        if let Some(c) = self.hovered_card && c > 0 {
            self.stack.get_mut(c - 1)
        } else {
            None
        }
    }

    /// Borrow the card after the currently hovered card, if it exists
    pub fn peek_next_card(&self) -> Option<&CardSet> {
        if let Some(c) = self.hovered_card && c < self.stack.len().saturating_sub(1) {
            self.stack.get(c + 1)
        } else {
            None
        }
    }

    /// Mutably borrow the card after the currently hovered card, if it exists
    pub fn next_card_mut(&mut self) -> Option<&mut CardSet> {
        if let Some(c) = self.hovered_card && c > 0 {
            self.stack.get_mut(c + 1)
        } else {
            None
        }
    }

    /// Takes ownership of `cards` passed in
    pub fn play_cards(&mut self, cards: &mut Vec<CardSet>) -> Result<(), CardStackError> {
        if self.len() + cards.len() <= self.lim.unwrap_or(usize::MAX) {
            self.stack.append(cards);
            Ok(())
        } else {
            Err(CardStackError::CardStackOverflow)
        }
    }
}

impl<CardSet: Card> PlayTo<CardSet> for CardStack<CardSet> {
    fn play_to(&mut self, card: CardSet) -> Result<(), CardSet> {
        if self.lim == None || self.lim.unwrap_or(0) < self.stack.len() {
            self.stack.push(card);
            Ok(())
        } else {
            Err(card)
        }
    }
}

impl<CardSet: Card> Debug for CardStack<CardSet> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.stack.len() == 0 && self.hovered_card != None {
            write!(f, "\x1b[100m  \x1b[40m")?;
        } else {
            for (i, c) in self.stack.iter().enumerate() {
                if let Some(sel_c) = self.hovered_card && sel_c == i {
                    write!(f, "\x1b[100m{}\x1b[40m ", c)?;
                } else {
                    write!(f, "{} ", c)?;
                }
            }
        }
        Ok(())
    }
}

impl<CardSet: Card> Display for CardStack<CardSet> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.stack_variant {
            StackVariant::Flush => {
                if self.hovered_card == None {
                    if self.stack.len() == 0 {
                        write!(f, "  ")?;
                    } else {
                        write!(f, "{} ", self.stack.last().unwrap())?;
                    }
                } else if self.stack.len() == 0 {
                    write!(f, "\x1b[100m  \x1b[40m")?;
                } else {
                    write!(f, "\x1b[100m{}\x1b[40m ", self.stack.last().unwrap())?;
                }
            }
            _ => {
                if self.stack.len() == 0 && self.hovered_card != None {
                    write!(f, "\x1b[100m  \x1b[40m")?;
                } else {
                    for (i, c) in self.stack.iter().enumerate() {
                        if let Some(sel_c) = self.hovered_card && sel_c == i {
                            write!(f, "\x1b[100m{}\x1b[40m ", c)?;
                        } else {
                            write!(f, "{} ", c)?;
                        }
                    }
                }
            }
        }
        Ok(())
    }
}

impl<CardSet: Card> From<Vec<CardSet>> for CardStack<CardSet> {
    fn from(value: Vec<CardSet>) -> Self {
        CardStack { 
            stack: value,
            stack_variant: StackVariant::Flush,
            lim: None,
            hovered_card: None,
        }
    }
}

impl<CardSet: Card> Into<Vec<CardSet>> for CardStack<CardSet> {
    fn into(self) -> Vec<CardSet> {
        self.stack
    }
}
