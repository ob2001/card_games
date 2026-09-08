use crate::lib_prelude::*;

#[derive(Clone)]
pub struct CardStack<CardSet: Card> {
    stack: Vec<CardSet>,
    stack_variant: StackVariant,
    lim: Option<usize>,
}

#[derive(Clone)]
pub enum StackVariant {
    HorizontalLtR,
    HorizontalRtL,
    VerticalTtB,
    VerticalBtT,
    Flush,
}

impl<CardSet: Card> CardStack<CardSet> {
    pub fn new(stack_variant: StackVariant, lim: Option<usize>) -> Self {
        CardStack {
            stack: vec![],
            stack_variant,
            lim,
        }
    }

    pub fn gather_cards(&mut self) -> Vec<CardSet> {
        let ret = self.stack.drain(0..self.stack.len()).collect();
        ret
    }

    pub(crate) fn set_stack_variant(&mut self, stack_variant: StackVariant) {
        self.stack_variant = stack_variant;
    }

    pub fn stack_variant(&self) -> &StackVariant {
        &self.stack_variant
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
        for c in &self.stack {
            write!(f, "{} ", c)?;
        }
        Ok(())
    }
}
