use std::fmt::Display;

#[derive(Clone, Copy, Debug)]
pub enum FrenchSuit {
    Spades,
    Hearts,
    Clubs,
    Diamonds,
}

#[derive(Clone, Copy, Debug)]
pub enum FrenchRank {
    Pip(u32),
    Jack,
    Queen,
    King,
}

#[derive(Clone, Copy, Debug)]
pub struct CardSuitRank {
    suit: FrenchSuit,
    rank: FrenchRank,
    is_face_up: bool,
}

impl CardSuitRank {
    pub fn new(suit: FrenchSuit, rank: FrenchRank) -> CardSuitRank {
        CardSuitRank { suit, rank, is_face_up: true }
    }

    pub fn flip(&mut self) {
        self.is_face_up = !self.is_face_up;
    }

    pub fn flip_face_up(&mut self) {
        self.is_face_up = true;
    }

    pub fn flip_face_down(&mut self) {
        self.is_face_up = false;
    }
}

impl Display for CardSuitRank {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use FrenchRank::*;
        use FrenchSuit::*;
        match self.rank {
            Jack => {write!(f, "J")?}
            Queen => {write!(f, "Q")?},
            King => {write!(f, "K")?},
            Pip(i) => {write!(f, "{}", i)?}
        }
        match self.suit {
            Spades => {write!(f, "♠")},
            Hearts => {write!(f, "♥")},
            Clubs => {write!(f, "♣")},
            Diamonds => {write!(f, "♦")},
        }
    }
}