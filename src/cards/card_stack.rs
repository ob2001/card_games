use crate::{
    cards::card_stack::StackError::InvalidCardSelection,
    lib_prelude::*
};

#[derive(Clone)]
pub struct CardStack<CardSet: Card> {
    stack: Vec<CardSet>,
    stack_variant: StackVariant,
    lim: Option<usize>,
    hovered_card: Option<usize>,
}

#[derive(Clone, Debug)]
pub enum StackVariant {
    HorizontalLtR,
    HorizontalRtL,
    VerticalTtB,
    VerticalBtT,
    Flush,
}

#[derive(Clone, Debug)]
pub enum StackError {
    InvalidCardSelection,
}

impl<CardSet: Card> CardStack<CardSet> {
    pub fn new(stack_variant: StackVariant, lim: Option<usize>) -> Self {
        CardStack {
            stack: vec![],
            stack_variant,
            lim,
            hovered_card: None,
        }
    }

    pub fn new_from(stack: Vec<CardSet>, stack_variant: StackVariant, lim: Option<usize>) -> Self {
        CardStack {
            stack,
            stack_variant,
            lim,
            hovered_card: None,
        }
    }

    pub fn len(&self) -> usize {
        self.stack.len()
    }

    pub fn gather_cards(&mut self) -> Vec<CardSet> {
        let ret = self.stack.drain(0..self.stack.len()).collect();
        ret
    }
    
    pub fn stack_variant(&self) -> &StackVariant {
        &self.stack_variant
    }

    pub fn set_hovered_card(&mut self, stack: usize) -> Result<(), StackError> {
        if stack <= self.stack.len() {
            self.hovered_card = Some(stack);
            Ok(())
        } else {
            Err(InvalidCardSelection)
        }
    }

    pub fn inc_hovered_card(&mut self) {
        if let Some(c) = self.hovered_card {
            self.hovered_card = Some((c + 1) % self.stack.len());
        }
    }

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

    pub fn get_hovered_card_idx(&self) -> Option<usize> {
        self.hovered_card
    }

    pub fn take_hovered_card(&mut self) ->  Option<CardSet> {
        if let Some(c) = self.hovered_card {
            Some(self.stack.remove(c))
        } else {
            None
        }
    }

    pub fn take_hovered_stack(&mut self) -> Option<CardStack<CardSet>> {
        if let Some(c) = self.hovered_card {
            Some(self.stack.split_off(c).into())
        } else {
            None
        }
    }

    pub fn hover(&mut self) {
        self.hovered_card = Some(self.stack.len().saturating_sub(1));
    }

    pub fn unhover(&mut self) {
        self.hovered_card = None;
    }

    pub fn set_lim(&mut self, lim: usize) {
        self.lim = Some(lim);
    }

    pub fn unset_lim(&mut self) {
        self.lim = None;
    }

    pub fn get_lim(&self) -> Option<usize> {
        self.lim
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
