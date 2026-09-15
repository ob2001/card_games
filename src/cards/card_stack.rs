use std::io::{ Stdout, Write };

use crate::{
    cards::card_stack::CardStackError::InvalidCardSelection,
    lib_prelude::*,
};

/// A representation of a stack of cards which may be played to, without
/// the special functionality associated with a deck (e.g. drawing, shuffling)
/// Has an optional stack limit which allows the stack to reject being played
/// to if the played card would exceed its limit
#[derive(Clone)]
pub struct CardStack<C: Card> {
    stack: Vec<C>,
    variant: CardStackVariant,
    lim: Option<usize>,
    hovered_card: Option<usize>,
    
    /// Only set if this element has no parent
    pos: Option<(u16, u16)>,
}

/// Variant enum for use when displaying the CardStack. Indicates the order and direction
/// that the cards in the stack should be displayed.
#[derive(Clone, Debug)]
pub enum CardStackVariant {
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

impl<C: Card> CardStack<C> {
    /// Return a new, empty, unhovered, unlimited CardStack
    pub fn new(variant: CardStackVariant, lim: Option<usize>, pos: Option<(u16, u16)>) -> Self {
        CardStack {
            stack: vec![],
            variant,
            lim,
            hovered_card: None,
            pos
        }
    }

    /// Return a new, unhovered, unlimited Cardstack containing the passed cards (in order)
    pub fn new_from(stack: Vec<C>, variant: CardStackVariant, lim: Option<usize>) -> Self {
        CardStack {
            stack,
            variant,
            lim,
            hovered_card: None,
            pos: None
        }
    }

    pub fn set_pos(&mut self, pos: Option<(u16, u16)>) {
        self.pos = pos;
    }

    pub fn get_pos(&self) -> Option<(u16, u16)> {
        self.pos
    }

    pub fn get_pos_mut(&mut self) -> Option<&mut (u16, u16)> {
        self.pos.as_mut()
    }

    pub fn draw_imm(&self, stdout: &mut Stdout) -> Result<(), std::io::Error> {
        self.draw_que(stdout)?;
        stdout.flush()
    }

    // TODO: Finish the Horizontal LtR arm
    pub fn draw_que(&self, stdout: &mut Stdout) -> Result<(), std::io::Error> {
        match &self.variant {
            CardStackVariant::Flush => {
                if self.stack.len() > 0 {
                    if self.hovered_card != None {
                        queue!(stdout, style::PrintStyledContent(format!("{}", self.stack.last().unwrap()).on_dark_grey()))
                    } else {
                        queue!(stdout, style::Print(format!("{}", self.stack.last().unwrap())))
                    }
                } else {
                    if self.hovered_card != None {
                        queue!(stdout, style::PrintStyledContent("  ".on_dark_grey()))
                    } else {
                        queue!(stdout, style::Print("  "))
                    }
                }
            },
            CardStackVariant::HorizontalLtR => {
                if self.stack.len() > 0 {
                    for (i, card) in self.stack.iter().enumerate() {
                        if let Some(hovered_card_idx) = self.hovered_card && hovered_card_idx == i {
                            queue!(stdout,
                                style::PrintStyledContent(format!("{}", card).on_dark_grey()),
                                cursor::MoveRight(1)
                            )?;
                        } else {
                            card.draw_que(stdout)?;
                            queue!(stdout, cursor::MoveRight(1))?;
                        }
                    }
                } else {
                    if let Some(_) = self.hovered_card {
                        queue!(stdout, style::PrintStyledContent("  ".on_dark_grey()))?;
                    } else {
                        queue!(stdout, style::Print("  "))?;
                    }
                }
            
                if self.pos.is_some() {
                    queue!(stdout, terminal::Clear(terminal::ClearType::UntilNewLine))
                } else {
                    Ok(())
                }
            },
            var => todo!("Drawing a {:?} not yet implemented", var),
        }
    }

    /// Return the current length of the CardStack
    pub fn len(&self) -> usize {
        self.stack.len()
    }

    pub fn widest_card_str(&self) -> usize {
        self.stack.iter().fold(0, |acc, c| acc.max(c.str_len()))
    }

    pub fn str_len(&self) -> usize {
        match self.variant {
            CardStackVariant::Flush => { self.stack.last().map_or(0, |c| c.str_len()) }
            CardStackVariant::HorizontalLtR => { format!("{}", self).len() },
            CardStackVariant::HorizontalRtL => { todo!() },
            CardStackVariant::VerticalTtB => { todo!() },
            CardStackVariant::VerticalBtT => { todo!() },
        }
    }

    /// Collect all cards in the stack and return them to the caller
    pub fn gather_cards(&mut self) -> Vec<C> {
        let ret = self.stack.drain(0..self.stack.len()).collect();
        ret
    }
    
    /// Return the display variant of the CardStack
    pub fn stack_variant(&self) -> &CardStackVariant {
        &self.variant
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
    pub fn last(&self) -> Option<&C> {
        self.stack.last()
    }

    /// Mutably borrow the last card on the CardStack (if it exists)
    pub fn last_mut(&mut self) -> Option<&mut C> {
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
    pub fn get_hovered_card(&self) -> Option<&C> {
        if let Some(c) = self.hovered_card {
            self.stack.get(c)
        } else {
            None
        }
    }

    /// Mutably borrow the currently hovered card
    pub fn get_hovered_card_mut(&mut self) -> Option<&mut C> {
        if let Some(c) = self.hovered_card {
            Some(&mut self.stack[c])
        } else {
            None
        }
    }

    /// Remove the currently hovered card from the CardStack and return
    /// it to the caller
    pub fn take_hovered_card(&mut self) ->  Option<C> {
        if let Some(c) = self.hovered_card {
            Some(self.stack.remove(c))
        } else {
            None
        }
    }

    /// Remove all cards from the currently hovered card (if it exists) to the end of the stack
    /// and return them (in order) to the caller
    pub fn take_hovered_stack(&mut self) -> Option<CardStack<C>> {
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
    pub fn peek_prev_card(&self) -> Option<&C> {
        if let Some(c) = self.hovered_card && c > 0 {
            self.stack.get(c - 1)
        } else {
            None
        }
    }

    /// Mutably borrow the card before the currently hovered card, if it exists
    pub fn prev_card_mut(&mut self) -> Option<&mut C> {
        if let Some(c) = self.hovered_card && c > 0 {
            self.stack.get_mut(c - 1)
        } else {
            None
        }
    }

    /// Borrow the card after the currently hovered card, if it exists
    pub fn peek_next_card(&self) -> Option<&C> {
        if let Some(c) = self.hovered_card && c < self.stack.len().saturating_sub(1) {
            self.stack.get(c + 1)
        } else {
            None
        }
    }

    /// Mutably borrow the card after the currently hovered card, if it exists
    pub fn next_card_mut(&mut self) -> Option<&mut C> {
        if let Some(c) = self.hovered_card && c > 0 {
            self.stack.get_mut(c + 1)
        } else {
            None
        }
    }

    /// Takes ownership of `cards` passed in
    pub fn play_cards(&mut self, cards: &mut Vec<C>) -> Result<(), CardStackError> {
        if self.len() + cards.len() <= self.lim.unwrap_or(usize::MAX) {
            self.stack.append(cards);
            Ok(())
        } else {
            Err(CardStackError::CardStackOverflow)
        }
    }
}

impl<C: Card> PlayTo<C> for CardStack<C> {
    fn play_to(&mut self, card: C) -> Result<(), C> {
        if self.lim == None || self.lim.unwrap_or(0) < self.stack.len() {
            self.stack.push(card);
            Ok(())
        } else {
            Err(card)
        }
    }
}

impl<C: Card> Debug for CardStack<C> {
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

impl<C: Card> Display for CardStack<C> {
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

impl<C: Card> From<Vec<C>> for CardStack<C> {
    fn from(value: Vec<C>) -> Self {
        CardStack { 
            stack: value,
            variant: CardStackVariant::Flush,
            lim: None,
            hovered_card: None,
            pos: None
        }
    }
}

impl<C: Card> Into<Vec<C>> for CardStack<C> {
    fn into(self) -> Vec<C> {
        self.stack
    }
}
