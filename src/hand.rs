use crate::prelude::*;

#[derive(Debug)]
pub struct Hand<CardSet: Card> {
    pub hand: Vec<CardSet>,
}

impl<CardSet: Card> Hand<CardSet> {
    pub fn new_empty() -> Self {
        Hand { hand: vec![] }
    }
}