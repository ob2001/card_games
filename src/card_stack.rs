use crate::prelude::*;

#[derive(Copy, Clone, Debug)]
pub enum StackVariant {
    Flush,
    FanHorizontal,
    FanVertical,
}

#[derive(Clone, Debug)]
pub struct CardStack<CardSet: Card> {
    stack: Vec<CardSet>,
    lim: Option<usize>,
    variant: StackVariant,
}

impl<CardSet: Card> CardStack<CardSet> {
    pub fn new(variant: Option<StackVariant>, lim: Option<usize>) -> Self {
        CardStack { variant: variant.unwrap_or(StackVariant::Flush), stack: vec![], lim }
    }
}

impl<CardSet: Card> PlayTo<CardSet> for CardStack<CardSet> {
    fn play_to(&mut self, card: CardSet) -> Result<(), CardSet>
    {
        if self.lim.is_some() && self.stack.len() < self.lim.unwrap() {
            self.stack.push(card);
            Ok(())
        } else {
            Err(card)
        }
    }
}