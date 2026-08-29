use crate::prelude::*;
use super::card_stack::{CardStack, StackVariant};


#[derive(Clone, Copy, Debug)]
pub enum TableauVariant {
    Horizontal,
    Vertical,
    Circle
}

#[derive(Clone, Debug)]
pub struct Tableau<CardSet: Card>{
    tableau: Vec<CardStack<CardSet>>,
    selected_stack: usize,
    variant: TableauVariant,
}

impl<CardSet: Card> Tableau<CardSet> {
    pub fn new(tableau_variant: Option<TableauVariant>, stack_variant: Option<StackVariant>, cols: usize) -> Self {
        Tableau { 
            tableau: vec![CardStack::new(stack_variant, None); cols],
            selected_stack: 0,
            variant: tableau_variant.unwrap_or(TableauVariant::Horizontal)
        }
    }

    pub fn set_selected_stack(&mut self, stack: usize) -> Result<(), &str> {
        if stack <= self.tableau.len() {
            self.selected_stack = stack;
            Ok(())
        } else {
            Err("Invalid stack selection")
        }
    }

    pub fn inc_selected_stack(&mut self) {
        self.selected_stack = (self.selected_stack + 1) % self.tableau.len();
    }

    pub fn dec_selected_stack(&mut self) {
        let mut tmp = false;
        if self.selected_stack > isize::MAX as usize {
            self.selected_stack -= isize::MAX as usize;
            tmp = true;
        }

        self.selected_stack = self.selected_stack.checked_sub(1).unwrap_or(self.tableau.len().saturating_sub(1));

        if tmp {
            self.selected_stack += isize::MAX as usize;
        }
    }

    pub fn get_selected_stack(&self) -> usize {
        self.selected_stack
    }
}

impl<CardSet: Card> std::ops::Index<usize> for Tableau<CardSet> {
    type Output = CardStack<CardSet>;
    fn index(&self, index: usize) -> &Self::Output {
        &self.tableau[index]
    }
}

impl<CardSet: Card> std::ops::IndexMut<usize> for Tableau<CardSet> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.tableau[index]
    }
}

impl<CardSet: Card> PlayTo<CardSet> for Tableau<CardSet> {
    fn play_to(&mut self, card: CardSet) -> Result<(), CardSet> {
        self.tableau[self.selected_stack].play_to(card)
    }
}