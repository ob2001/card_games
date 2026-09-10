use crate::{
    cards::card_stack::StackError::InvalidStackSelection,
    lib_prelude::*
};

#[derive(Clone)]
pub struct CardStack<CardSet: Card> {
    stack: Vec<CardSet>,
    stack_variant: StackVariant,
    lim: Option<usize>,
    selected_card: usize,
    active: bool,
}

#[derive(Clone)]
pub enum StackVariant {
    HorizontalLtR,
    HorizontalRtL,
    VerticalTtB,
    VerticalBtT,
    Flush,
}

#[derive(Clone, Debug)]
pub enum StackError {
    InvalidStackSelection
}

impl<CardSet: Card> CardStack<CardSet> {
    pub fn new(stack_variant: StackVariant, lim: Option<usize>) -> Self {
        CardStack {
            stack: vec![],
            stack_variant,
            lim,
            selected_card: 0,
            active: false,
        }
    }

    pub fn new_from(stack: Vec<CardSet>, stack_variant: StackVariant, lim: Option<usize>) -> Self {
        CardStack {
            stack,
            stack_variant,
            lim,
            selected_card: 0,
            active: false,
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

    pub fn set_selected_card(&mut self, stack: usize) -> Result<(), StackError> {
        if stack <= self.stack.len() {
            self.selected_card = stack;
            Ok(())
        } else {
            Err(InvalidStackSelection)
        }
    }

    pub fn inc_selected_card(&mut self) {
        self.selected_card = (self.selected_card + 1) % self.stack.len();
    }

    pub fn dec_selected_card(&mut self) {
        let mut tmp = false;

        if self.selected_card > isize::MAX as usize {
            self.selected_card = self.selected_card - isize::MAX as usize;
            tmp = true;
        }

        self.selected_card = self.selected_card
            .checked_sub(1)
            .unwrap_or(self.stack.len().saturating_sub(1));

        if tmp {
            self.selected_card = self.selected_card + isize::MAX as usize;
        }
    }

    pub fn get_selected_card(&self) -> usize {
        self.selected_card
    }

    pub fn take_selected_card(&mut self) -> CardSet {
        self.stack.remove(self.selected_card)
    }

    pub fn take_selected_stack(&mut self) -> CardStack<CardSet> {
        self.stack.split_off(self.selected_card).into()
    }

    pub fn activate(&mut self) {
        self.active = true;
        if self.stack.len() > 0 {
            self.selected_card = self.stack.len() - 1;
        }
    }

    pub fn deactivate(&mut self) {
        self.active = false;
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
        for (i, c) in self.stack.iter().enumerate() {
            if self.active && self.selected_card == i {
                write!(f, "\x1b[100m{}\x1b[40m ", c)?;
            } else {
                write!(f, "{} ", c)?;
            }
        }
        Ok(())
    }
}

impl<CardSet: Card> Display for CardStack<CardSet> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.stack.len() == 0 && self.active {
            write!(f, "\x1b[100m  \x1b[40m")?;
        } else {
            for (i, c) in self.stack.iter().enumerate() {
                if self.active && self.selected_card == i {
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
            selected_card: 0,
            active: false,
        }
    }
}
