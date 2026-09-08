use crate::lib_prelude::*;

pub mod card_stack;
pub mod deck;
pub mod tableau;

pub trait Card: Debug + Display + Clone {}
pub trait Rank: Debug + Clone + Copy + PartialEq + Eq + PartialOrd + Ord + Display {}

pub mod french_card {
    use super::{Card, Display, Rank};

    #[derive(Clone, Copy, Debug)]
    pub enum FrenchCard {
        Spades(FrenchRank),
        Hearts(FrenchRank),
        Clubs(FrenchRank),
        Diamonds(FrenchRank),
        Joker,
    }

    impl Card for FrenchCard {}

    impl Display for FrenchCard {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Self::Spades(r) => {
                    write!(f, "{}♠", r)
                }
                Self::Hearts(r) => {
                    write!(f, "{}♥", r)
                }
                Self::Clubs(r) => {
                    write!(f, "{}♣", r)
                }
                Self::Diamonds(r) => {
                    write!(f, "{}♦", r)
                }
                Self::Joker => {
                    write!(f, "Jk")
                }
            }
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
    pub enum FrenchRank {
        Pip(u32),
        Jack,
        Queen,
        King,
    }

    impl Rank for FrenchRank {}

    impl Display for FrenchRank {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Self::Pip(1) => {
                    write!(f, "A")
                }
                Self::Pip(i) => write!(f, "{}", i),
                Self::Jack => {
                    write!(f, "J")
                }
                Self::Queen => {
                    write!(f, "Q")
                }
                Self::King => {
                    write!(f, "K")
                }
            }
        }
    }
}

pub mod italian_card {
    use super::{Card, Display, Rank};

    #[derive(Clone, Copy, Debug)]
    pub enum ItalianCard {
        Swords(ItalianRank),
        Cups(ItalianRank),
        Batons(ItalianRank),
        Coins(ItalianRank),
    }

    impl Display for ItalianCard {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Self::Swords(r) => {
                    write!(f, "{}♠", r)
                }
                Self::Cups(r) => {
                    write!(f, "{}♥", r)
                }
                Self::Batons(r) => {
                    write!(f, "{}♣", r)
                }
                Self::Coins(r) => {
                    write!(f, "{}♦", r)
                }
            }
        }
    }

    impl Card for ItalianCard {}

    #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
    pub enum ItalianRank {
        Pip(u32),
        Jack,
        Knight,
        King,
    }

    impl Rank for ItalianRank {}

    impl Display for ItalianRank {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Self::Pip(1) => {
                    write!(f, "A")
                }
                Self::Pip(i) => write!(f, "{}", i),
                Self::Jack => {
                    write!(f, "J")
                }
                Self::Knight => {
                    write!(f, "k")
                }
                Self::King => {
                    write!(f, "K")
                }
            }
        }
    }
}

pub mod tarocchi_card {
    use super::{Card, Display, Rank};

    #[derive(Clone, Debug)]
    pub enum TarocchiCard {
        Swords(TarocchiRank),
        Cups(TarocchiRank),
        Batons(TarocchiRank),
        Coins(TarocchiRank),
    }

    impl Card for TarocchiCard {}

    impl Display for TarocchiCard {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Self::Swords(r) => {
                    write!(f, "{}♠", r)
                }
                Self::Cups(r) => {
                    write!(f, "{}♥", r)
                }
                Self::Batons(r) => {
                    write!(f, "{}♣", r)
                }
                Self::Coins(r) => {
                    write!(f, "{}♦", r)
                }
            }
        }
    }

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
                Self::Pip(1) => {
                    write!(f, "A")
                }
                Self::Pip(i) => write!(f, "{}", i),
                Self::Jack => {
                    write!(f, "J")
                }
                Self::Knight => {
                    write!(f, "k")
                }
                Self::Queen => {
                    write!(f, "Q")
                }
                Self::King => {
                    write!(f, "K")
                }
            }
        }
    }

    impl Rank for TarocchiRank {}
}

pub mod tarot_card {
    use super::{Card, Display, Rank, tarocchi_card::TarocchiRank};

    #[derive(Clone, Copy, Debug)]
    pub enum TarotCard {
        Swords(TarocchiRank),
        Cups(TarocchiRank),
        Batons(TarocchiRank),
        Coins(TarocchiRank),
        Trump(TarotTrumps),
    }

    impl Display for TarotCard {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                TarotCard::Swords(r) => write!(f, "{}♠", r),
                TarotCard::Cups(r) => write!(f, "{}♥", r),
                TarotCard::Batons(r) => write!(f, "{}♣", r),
                TarotCard::Coins(r) => write!(f, "{}♦", r),
                TarotCard::Trump(r) => write!(f, "{}", r),
            }
        }
    }

    impl Card for TarotCard {}

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

    impl Rank for TarotTrumps {}

    impl Display for TarotTrumps {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                TarotTrumps::Fool => write!(f, "0"),
                TarotTrumps::Magician => write!(f, "I"),
                TarotTrumps::HighPriestess => write!(f, "II"),
                TarotTrumps::Empress => write!(f, "III"),
                TarotTrumps::Emperor => write!(f, "IV"),
                TarotTrumps::Hierophant => write!(f, "V"),
                TarotTrumps::Lovers => write!(f, "VI"),
                TarotTrumps::Chariot => write!(f, "VII"),
                TarotTrumps::Strength => write!(f, "VIII"),
                TarotTrumps::Hermit => write!(f, "IX"),
                TarotTrumps::WheelOfFortune => write!(f, "X"),
                TarotTrumps::Justice => write!(f, "XI"),
                TarotTrumps::HangedMan => write!(f, "XII"),
                TarotTrumps::Death => write!(f, "XIII"),
                TarotTrumps::Temperance => write!(f, "XIV"),
                TarotTrumps::Devil => write!(f, "XV"),
                TarotTrumps::Tower => write!(f, "XVI"),
                TarotTrumps::Star => write!(f, "XVII"),
                TarotTrumps::Moon => write!(f, "XVIII"),
                TarotTrumps::Sun => write!(f, "XIX"),
                TarotTrumps::Judgement => write!(f, "XX"),
                TarotTrumps::World => write!(f, "XXI"),
            }
        }
    }
}

pub mod five_crowns_card {
    use super::{Card, Display, Rank};

    #[derive(Clone, Debug)]
    pub enum FiveCrownsCard {
        Spades(FiveCrownsRank),
        Hearts(FiveCrownsRank),
        Clubs(FiveCrownsRank),
        Diamonds(FiveCrownsRank),
        Stars(FiveCrownsRank),
        Joker,
    }

    impl Display for FiveCrownsCard {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Self::Spades(r) => {
                    write!(f, "{}♠", r)
                }
                Self::Hearts(r) => {
                    write!(f, "{}♥", r)
                }
                Self::Clubs(r) => {
                    write!(f, "{}♣", r)
                }
                Self::Diamonds(r) => {
                    write!(f, "{}♦", r)
                }
                Self::Stars(r) => {
                    write!(f, "{}★", r)
                }
                Self::Joker => {
                    write!(f, "Jk")
                }
            }
        }
    }

    impl Card for FiveCrownsCard {}

    #[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord)]
    pub enum FiveCrownsRank {
        Pip(u32),
        Jack,
        Queen,
        King,
    }

    impl Display for FiveCrownsRank {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Self::Pip(1) => {
                    write!(f, "A")
                }
                Self::Pip(i) => write!(f, "{}", i),
                Self::Jack => {
                    write!(f, "J")
                }
                Self::Queen => {
                    write!(f, "Q")
                }
                Self::King => {
                    write!(f, "K")
                }
            }
        }
    }

    impl Rank for FiveCrownsRank {}
}

pub mod magic_card;

#[derive(Clone, Debug)]
pub struct FlippableCard<C: Card>(C, pub bool);

impl<C: Card> FlippableCard<C> {
    pub fn new(card: C) -> FlippableCard<C> {
        FlippableCard(card, true)
    }

    pub fn flip(&mut self) {
        self.1 = !self.1;
    }

    pub fn flip_face_up(&mut self) {
        self.1 = true;
    }

    pub fn flip_face_down(&mut self) {
        self.1 = false;
    }

    pub fn into_inner(self) -> C {
        self.0
    }

    pub fn into_inner_checked(self) -> Result<C, Self> {
        if self.1 { Ok(self.0) } else { Err(self) }
    }

    pub fn peek_inner_checked(&self) -> Option<&C> {
        if self.1 { Some(&self.0) } else { None }
    }
}

impl<C: Card> Card for FlippableCard<C> {}

impl<C: Card> Display for FlippableCard<C> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.1 {
            write!(f, "{}", self.0)
        } else {
            write!(f, "XX")
        }
    }
}
