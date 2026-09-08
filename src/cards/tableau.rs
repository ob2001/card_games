use super::card_stack::CardStack;
use crate::lib_prelude::*;

#[derive(Clone)]
pub struct Tableau<CardSet: Card> {
    stacks: Vec<CardStack<CardSet>>,
    selected_stack: usize,
}

impl<CardSet: Card> Tableau<CardSet> {
    pub fn new(stack_variant: StackVariant, cols: usize) -> Self {
        Tableau {
            stacks: vec![CardStack::new(stack_variant, None); cols],
            selected_stack: 0,
        }
    }

    pub fn num_stacks(&self) -> usize {
        self.stacks.len()
    }

    pub fn set_selected_stack(&mut self, stack: usize) -> Result<(), &str> {
        if stack <= self.stacks.len() {
            self.selected_stack = stack;
            Ok(())
        } else {
            Err("Invalid stack selection")
        }
    }

    pub fn inc_selected_stack(&mut self) {
        self.selected_stack = (self.selected_stack + 1) % self.stacks.len();
    }

    pub fn dec_selected_stack(&mut self) {
        let mut tmp = false;
        if self.selected_stack > isize::MAX as usize {
            self.selected_stack -= isize::MAX as usize;
            tmp = true;
        }

        self.selected_stack = self
            .selected_stack
            .checked_sub(1)
            .unwrap_or(self.stacks.len().saturating_sub(1));

        if tmp {
            self.selected_stack += isize::MAX as usize;
        }
    }

    pub fn selected_stack(&self) -> usize {
        self.selected_stack
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
        self.stacks[self.selected_stack].play_to(card)
    }
}

impl<CardSet: Card> std::fmt::Debug for Tableau<CardSet> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for s in &self.stacks {
            write!(f, "||")?;
            writeln!(f, "{:?}", s)?;
        }
        Ok(())
    }
}
