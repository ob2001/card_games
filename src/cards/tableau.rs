use super::card_stack::CardStack;
use crate::{cards::card_stack, lib_prelude::*};

/// A representation of an array of card stacks which may each be played to.
#[derive(Clone)]
pub struct Tableau<CardSet: Card> {
    stacks: Vec<CardStack<CardSet>>,
    variant: TableauVariant,
    hovered_stack: Option<usize>,
}

/// Variant enum for use when displaying the Tableau. Indicates the order and direction
/// that the stacks in the tableau should be displayed.
#[derive(Clone, Debug)]
pub enum TableauVariant {
    HorizontalLtR,
    HorizontalRtL,
    VerticalTtB,
    VerticalBtT,
}

/// Errors originating from Tableau functionality
#[derive(Clone, Debug)]
pub enum TableauError {
    CardStackError(card_stack::CardStackError),
    InvalidStackSelection,
    InvalidHoveredStack
}

impl<CardSet: Card> Tableau<CardSet> {
    /// Return a new Tableau with `n_stacks` empty stacks of variant `stack_variant`
    pub fn new(stack_variant: StackVariant, n_stacks: usize) -> Self {
        Tableau {
            stacks: vec![CardStack::new(stack_variant, None); n_stacks],
            variant: TableauVariant::HorizontalLtR,
            hovered_stack: None,
        }
    }

    /// Set the display variant of the Tableau
    pub fn set_tableau_variant(&mut self, variant: TableauVariant) {
        self.variant = variant;
    }

    /// Borrow the display variant of the Tableau
    pub fn get_tableau_variant(&self) -> &TableauVariant {
        &self.variant
    }

    /// Return the number of stacks in the Tableau
    pub fn num_stacks(&self) -> usize {
        self.stacks.len()
    }

    /// Directly set the hovered stack to the passed index, if it exists
    pub fn set_hovered_stack(&mut self, idx: usize) -> Result<(), TableauError> {
        if idx <= self.stacks.len() {
            self.hovered_stack = Some(idx);
            Ok(())
        } else {
            Err(TableauError::InvalidStackSelection)
        }
    }

    /// Hover over the next stack in the Tableau, wrapping from the end to the beginning
    pub fn inc_hovered_stack(&mut self) {
        if let Some(c) = self.hovered_stack {
            self.hovered_stack = Some((c + 1) % self.stacks.len());
        }
    }

    /// Hover over the previous stack in the Tableau, wrapping from the beginning to the end
    pub fn dec_hovered_stack(&mut self) {
        let mut tmp = false;
        if let Some(c) = self.hovered_stack && c > isize::MAX as usize {
            self.hovered_stack = Some(c - isize::MAX as usize);
            tmp = true;
        }

        if self.hovered_stack != None {
            self.hovered_stack = Some(self.hovered_stack.unwrap()
                .checked_sub(1)
                .unwrap_or(self.stacks.len().saturating_sub(1)));
        }

        if tmp {
            self.hovered_stack = Some(self.hovered_stack.unwrap() + isize::MAX as usize);
        }
    }

    /// Return the currently hovered stack of the Tableau, if any
    pub fn hovered_stack(&self) -> Option<usize> {
        self.hovered_stack
    }

    /// Mutably borrow the currently hovered CardStack, if any
    pub fn get_hovered_stack_mut(&mut self) -> Option<&mut CardStack<CardSet>> {
        if let Some(c) = self.hovered_stack {
            Some(&mut self.stacks[c])
        } else {
            None
        }
    }

    /// Mutably borrow all stacks in provided range provided the range is valid
    pub fn stacks_mut(&mut self, range: std::ops::Range<usize>) -> Option<&mut [CardStack<CardSet>]> {
        self.stacks.get_mut(range)
    }

    /// Borrow all stacks in provided range provided the range is valid
    pub fn stacks(&self, range: std::ops::Range<usize>) -> Option<&[CardStack<CardSet>]> {
        self.stacks.get(range)
    }

    /// Play provided card to the indicated stack, if it exists
    pub fn play_card_to_stack(&mut self, card: CardSet, i: usize) -> Result<(), CardSet> {
        if i < self.stacks.len() {
            self.stacks[i].play_to(card)
        } else {
            Err(card)
        }
    }

    /// Play all provided cards to the indicated stack, if it exists
    pub fn play_cards_to_stack(&mut self, cards: &mut Vec<CardSet>, i: usize) -> Result<(), TableauError> {
        if i < self.stacks.len() {
            self.stacks[i].play_cards(cards).map_err(|e| TableauError::CardStackError(e))
        } else {
            Err(TableauError::InvalidStackSelection)
        }
    }

    /// Play all provided cards to the currently hovered stack, if it exists
    pub fn play_cards_to_hovered_stack(&mut self, cards: &mut Vec<CardSet>) -> Result<(), TableauError> {
        if let Some(i) = self.hovered_stack && i < self.stacks.len() {
            self.stacks[i].play_cards(cards).map_err(|e| TableauError::CardStackError(e))
        } else {
            Err(TableauError::InvalidHoveredStack)
        }
    }

    /// Gather all cards from each stack into a single Vec and return them to the caller
    pub fn gather_all(&mut self) -> Vec<CardSet> {
        let mut ret = vec![];

        for stack in self.stacks.iter_mut() {
            ret.append(&mut stack.gather_cards());
        }

        ret
    }

    /// Hover over the Tableau. Default to hovering over the first stack of the Tableau
    pub fn hover(&mut self) {
        self.hovered_stack = Some(0);
        self.update_hovered_stack();
    }

    /// Unhover the Tableau. Also unhover any stack of the Tableau currently hovered
    pub fn unhover(&mut self) {
        self.unhover_stack();
        self.hovered_stack = None;
    }

    /// Notify the currently hovered stack that it is being hovered
    pub fn hover_stack(&mut self) {
        if let Some(s) = self.hovered_stack {
            self.stacks[s].hover();
        }
    }

    /// Notify the currently hovered stack that it is being unhovered
    pub fn unhover_stack(&mut self) {
        if let Some(s) = self.hovered_stack {
            self.stacks[s].unhover();
        }
    }

    /// Unhover all stacks, then only hover the currently hovered stack
    pub fn update_hovered_stack(&mut self) {
        for s in &mut self.stacks { s.unhover(); }
        if let Some(c) = self.hovered_stack {
            self.stacks[c].hover();
        }
    }

    /// Borrow the card hovered in the currently hovered stack
    pub fn get_hovered_card(&self) -> Option<&CardSet> {
        if let Some(s) = self.hovered_stack {
            self.stacks[s].get_hovered_card()
        } else {
            None
        }
    }

    /// Mutably borrow the card hovered in the currently hovered stack
    pub fn get_hovered_card_mut(&mut self) -> Option<&mut CardSet> {
        if let Some(s) = self.hovered_stack {
            self.stacks[s].get_hovered_card_mut()
        } else {
            None
        }
    }
}

impl<CardSet: Card> std::ops::Index<usize> for Tableau<CardSet> {
    type Output = CardStack<CardSet>;
    fn index(&self, index: usize) -> &Self::Output {
        &self.stacks[index]
    }
}

impl<CardSet: Card> std::ops::IndexMut<usize> for Tableau<CardSet> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.stacks[index]
    }
}

impl<CardSet: Card> PlayTo<CardSet> for Tableau<CardSet> {
    fn play_to(&mut self, card: CardSet) -> Result<(), CardSet> {
        if let Some(c) = self.hovered_stack {
            self.stacks[c].play_to(card)
        } else {
            Err(card)
        }
    }
}

impl<CardSet: Card> Debug for Tableau<CardSet> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for s in &self.stacks {
            write!(f, "||")?;
            writeln!(f, "{:?}", s)?;
        }
        Ok(())
    }
}

impl<CardSet: Card> Display for Tableau<CardSet> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for s in &self.stacks {
            write!(f, "||")?;
            writeln!(f, "{}", s)?;
        }
        Ok(())
    }
}
