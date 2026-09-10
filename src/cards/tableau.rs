use super::card_stack::CardStack;
use crate::lib_prelude::*;

#[derive(Clone)]
pub struct Tableau<CardSet: Card> {
    stacks: Vec<CardStack<CardSet>>,
    variant: TableauVariant,
    hovered_stack: Option<usize>,
}

#[derive(Clone, Debug)]
pub enum TableauVariant {
    HorizontalLtR,
    HorizontalRtL,
    VerticalTtB,
    VerticalBtT,
}

pub enum TableauError {
    InvalidStackSelection,
}

impl<CardSet: Card> Tableau<CardSet> {
    pub fn new(stack_variant: StackVariant, cols: usize) -> Self {
        Tableau {
            stacks: vec![CardStack::new(stack_variant, None); cols],
            variant: TableauVariant::HorizontalLtR,
            hovered_stack: None,
        }
    }

    pub fn num_stacks(&self) -> usize {
        self.stacks.len()
    }

    pub fn set_hovered_stack(&mut self, stack: usize) -> Result<(), TableauError> {
        if stack <= self.stacks.len() {
            self.hovered_stack = Some(stack);
            Ok(())
        } else {
            Err(TableauError::InvalidStackSelection)
        }
    }

    pub fn inc_hovered_stack(&mut self) {
        if let Some(c) = self.hovered_stack {
            self.hovered_stack = Some((c + 1) % self.stacks.len());
        }
    }

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

    pub fn hovered_stack(&self) -> Option<usize> {
        self.hovered_stack
    }

    pub fn stacks_mut(&mut self, range: std::ops::Range<usize>) -> &mut [CardStack<CardSet>] {
        &mut self.stacks[range]
    }

    pub fn play_to_stack(&mut self, card: CardSet, i: usize) -> Result<(), CardSet> {
        if i < self.stacks.len() {
            self.stacks[i].play_to(card)
        } else {
            Err(card)
        }
    }

    pub fn gather_all(&mut self) -> Vec<CardSet> {
        let mut ret = vec![];

        for stack in self.stacks.iter_mut() {
            ret.append(&mut stack.gather_cards());
        }

        ret
    }

    pub fn hover(&mut self) {
        self.hovered_stack = Some(0);
        self.update_hovered_stack();
    }

    pub fn unhover(&mut self) {
        self.unhover_stack();
        self.hovered_stack = None;
    }

    pub fn hover_stack(&mut self) {
        if let Some(c) = self.hovered_stack {
            self.stacks[c].hover();
        }
    }

    pub fn unhover_stack(&mut self) {
        if let Some(c) = self.hovered_stack {
            self.stacks[c].unhover();
        }
    }

    pub fn update_hovered_stack(&mut self) {
        for s in &mut self.stacks { s.unhover(); }
        if let Some(c) = self.hovered_stack {
            self.stacks[c].hover();
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
