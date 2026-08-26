use crate::card::CardSuitRank;

#[derive(Debug)]
pub struct Hand {
    pub hand: Vec<CardSuitRank>,
}

impl Hand {
    pub fn new_empty() -> Self {
        Hand { hand: vec![] }
    }
}