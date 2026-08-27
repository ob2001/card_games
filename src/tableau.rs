use crate::{
    prelude::*,
    stack::{Stack, StackVariant}
};

#[derive(Debug)]
pub struct Tableau<CardSet: Card> {
    tableau: Vec<Stack<CardSet>>,
}

impl<CardSet: Card> Tableau<CardSet> {
    pub fn new(stack_variant: Option<StackVariant>, cols: usize) -> Self {
        Tableau { tableau: vec![Stack::new(stack_variant, None); cols] }
    }
}