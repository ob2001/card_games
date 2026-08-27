use crate::prelude::*;

pub trait Card: Flippable + Debug + Clone {}
pub trait Suit: Debug + Clone + Copy + Display {}
pub trait Rank: Debug + Clone + Copy + PartialEq + Eq + PartialOrd + Ord + Display {}

#[derive(Clone, Copy, Debug)]
pub enum FrenchSuit {
    Spades,
    Hearts,
    Clubs,
    Diamonds,
}

impl Display for FrenchSuit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Spades => {write!(f, "♠")},
            Self::Hearts => {write!(f, "♥")},
            Self::Clubs => {write!(f, "♣")},
            Self::Diamonds => {write!(f, "♦")},
        }
    }
}

impl Suit for FrenchSuit {}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum FrenchRank {
    Pip(u32),
    Jack,
    Queen,
    King,
}

impl Display for FrenchRank {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Pip(0) => {write!(f, "A")},
            Self::Pip(i) => write!(f, "{}", i),
            Self::Jack => {write!(f, "J")},
            Self::Queen => {write!(f, "Q")},
            Self::King => {write!(f, "K")},
        }
    }
}

impl Rank for FrenchRank {}

#[derive(Clone, Copy, Debug)]
pub enum ItalianSuit {
    Swords,
    Cups,
    Batons,
    Coins
}

impl Display for ItalianSuit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Swords => {write!(f, "♠")},
            Self::Cups => {write!(f, "♥")},
            Self::Batons => {write!(f, "♣")},
            Self::Coins => {write!(f, "♦")},
        }
    }
}

impl Suit for ItalianSuit {}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum ItalianRank {
    Pip(u32),
    Jack,
    Knight,
    King,
}

impl Display for ItalianRank {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Pip(0) => {write!(f, "A")},
            Self::Pip(i) => write!(f, "{}", i),
            Self::Jack => {write!(f, "J")},
            Self::Knight => {write!(f, "k")},
            Self::King => {write!(f, "K")},
        }
    }
}

impl Rank for ItalianRank {}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum TarocchiRank {
    Pip(u32),
    Jack,
    Knight,
    Queen,
    King,
}

impl Display for TarocchiRank {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Pip(0) => {write!(f, "A")},
            Self::Pip(i) => write!(f, "{}", i),
            Self::Jack => {write!(f, "J")},
            Self::Knight => {write!(f, "k")},
            Self::Queen => {write!(f, "Q")},
            Self::King => {write!(f, "K")},
        }
    }
}

impl Rank for TarocchiRank {}

#[derive(Clone, Copy, Debug)]
pub enum TarotSuit {
    Pip(u32),
    Jack,
    Knight,
    Queen,
    King,
    Trump(TarotTrumps)
}

impl Display for TarotSuit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!();
    }
}

impl Suit for TarotSuit {}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum TarotTrumps {
    Fool,
    Magician,
    HighPriestess,
    Empress,
    Emperor,
    Hierophant,
    Lovers,
    Chariot,
    Strength,
    Hermit,
    WheelOfFortune,
    Justice,
    HangedMan,
    Death,
    Temperance,
    Devil,
    Tower,
    Star,
    Moon,
    Sun,
    Judgement,
    World,
}

impl Display for TarotTrumps {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!();
    }
}

impl Rank for TarotTrumps {}

#[derive(Clone, Debug)]
pub struct SuitRankCard<S: Suit, R: Rank> {
    suit: S,
    rank: R,
    is_face_up: bool,
}

pub type FrenchCard = SuitRankCard<FrenchSuit, FrenchRank>;
pub type ItalianCard = SuitRankCard<ItalianSuit, ItalianRank>;
pub type TarocchiCard = SuitRankCard<ItalianSuit, TarocchiRank>;
pub type TarotCard = SuitRankCard<TarotSuit, TarocchiRank>;

impl<S: Suit, R: Rank> SuitRankCard<S, R> {
    pub fn new(suit: S, rank: R) -> SuitRankCard<S, R> {
        SuitRankCard { suit, rank, is_face_up: true }
    }
}

impl<S: Suit, R: Rank> Card for SuitRankCard<S, R> {}

impl<S: Suit, R: Rank> Flippable for SuitRankCard<S, R> {
    fn flip(&mut self) {
        self.is_face_up = !self.is_face_up;
    }

    fn flip_face_up(&mut self) {
        self.is_face_up = true;
    }

    fn flip_face_down(&mut self) {
        self.is_face_up = false;
    }
}

impl<S: Suit, R: Rank> Display for SuitRankCard<S, R> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.rank, self.suit)
    }
}