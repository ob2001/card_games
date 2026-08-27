use crate::prelude::*;

#[derive(Copy, Clone, Debug)]
pub enum StackVariant {
    Flush,
    FanHorizontal,
    FanVertical,
}

#[derive(Clone, Debug)]
pub struct Stack<CardSet: Card> {
    stack: Vec<CardSet>,
    variant: StackVariant,
    lim: Option<usize>
}

impl<CardSet: Card> Stack<CardSet> {
    pub fn new(variant: Option<StackVariant>, lim: Option<usize>) -> Self {
        Stack { variant: variant.unwrap_or(StackVariant::Flush), stack: vec![], lim }
    }
}

impl<CardSet: Card> PlayableTo<CardSet> for Stack<CardSet> {
    fn play_to(&mut self, card: CardSet)
    {
        self.stack.push(card);
    }
}