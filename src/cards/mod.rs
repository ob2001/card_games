use std::io::{ Stdout, Write };
use crate::lib_prelude::*;

pub mod card_stack;
pub mod deck;
pub mod tableau;

/// Trait required to be implemented by any object to act as a Card throughout the library
pub trait Card: Debug + Display + Clone {
    /// Queues the card to be drawn to passed `stdout`
    fn draw_que(&self, stdout: &mut Stdout) -> Result<(), std::io::Error>;

    /// Immediately draws the card to passed `stdout`
    fn draw_imm(&self, stdout: &mut Stdout) -> Result<(), std::io::Error> {
        self.draw_que(stdout)?;
        stdout.flush()
    }

    /// Returns the length of the string representation of the card
    fn str_len(&self) -> usize {
        format!("{}", self).chars().count()
    }
}

pub trait Rank: Debug + Clone + Copy + PartialEq + Eq + PartialOrd + Ord + Display {
    /// Returns the next-highest rank if it exists
    fn next(&self) -> Option<Self>;

    /// Returns the next lowest rank if it exists
    fn prev(&self) -> Option<Self>;
}

/// A module for French type cards
pub mod french_card {
    use super::{ Card, Display, Rank, Stdout, queue, style::{self, Color}, Stylize };

    #[derive(Clone, Copy, Debug)]
    pub enum FrenchCard {
        Spades(FrenchRank),
        Hearts(FrenchRank),
        Clubs(FrenchRank),
        Diamonds(FrenchRank),
        Joker,
    }

    impl FrenchCard {
        pub fn rank(&self) -> Option<FrenchRank> {
            use FrenchCard::*;
            match self {
                &Spades(r) | &Hearts(r) | &Clubs(r) | &Diamonds(r) => Some(r),
                Joker => None,
            }
        }

        pub fn suit_equals(&self, other: &FrenchCard) -> bool {
            match (self, other) {
                (Self::Spades(_), Self::Spades(_)) | (Self::Hearts(_), Self::Hearts(_)) | (Self::Clubs(_), Self::Clubs(_)) | (Self::Diamonds(_), Self::Diamonds(_)) => true,
                _ => false
            }
        }

        pub fn is_black(&self) -> bool {
            match self {
                Self::Spades(_) | Self::Clubs(_) | Self::Joker => true,
                _ => false,
            }
        }

        pub fn is_red(&self) -> bool {
            match self {
                Self::Hearts(_) | Self::Diamonds(_) | Self::Joker => true,
                _ => false,
            }
        }
    }

    impl Card for FrenchCard {
        fn draw_que(&self, stdout: &mut Stdout) -> Result<(), std::io::Error> {
            style::PrintStyledContent("H".with(style::Color::Red));
            match self {
                Self::Spades(r) => queue!(stdout, style::Print(format!("{}♠", r))),
                Self::Hearts(r) => queue!(stdout, style::PrintStyledContent(format!("{}♥", r).with(Color::Red))),
                Self::Clubs(r) => queue!(stdout, style::Print(format!("{}♣", r))),
                Self::Diamonds(r) => queue!(stdout, style::PrintStyledContent(format!("{}♦", r).with(Color::Red))),
                Self::Joker => queue!(stdout, style::Print("Jk")),
            }
        }

        fn str_len(&self) -> usize {
            match self {
                &Self::Spades(r) | &Self::Hearts(r) | &Self::Clubs(r) | &Self::Diamonds(r) => {
                    match r {
                        FrenchRank::Pip(10) => 3,
                        _ => 2
                    }
                },
                _ => 2,
            }
        }
    }

    impl Display for FrenchCard {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Self::Spades(r) => write!(f, "{}♠", r),
                Self::Hearts(r) => write!(f, "\x1b[91m{}♥\x1b[37m", r),
                Self::Clubs(r) => write!(f, "{}♣", r),
                Self::Diamonds(r) => write!(f, "\x1b[91m{}♦\x1b[37m", r),
                Self::Joker => write!(f, "Jk"),
            }
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum FrenchRank {
        Pip(u32),
        Jack,
        Queen,
        King,
    }

    impl Rank for FrenchRank {
        fn next(&self) -> Option<Self> {
            match self {
                Self::Pip(10) => Some(Self::Jack),
                Self::Pip(n) => Some(Self::Pip(n + 1)),
                Self::Jack => Some(Self::Queen),
                Self::Queen => Some(Self::King),
                Self::King => None,
            }
        }

        fn prev(&self) -> Option<Self> {
            match self {
                Self::Pip(1) => None,
                Self::Pip(n) => Some(Self::Pip(n - 1)),
                Self::Jack => Some(Self::Pip(10)),
                Self::Queen => Some(Self::Jack),
                Self::King => Some(Self::Queen),
            }
        }
    }

    impl PartialOrd for FrenchRank {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            Some(self.cmp(other))
        }
    }

    impl Ord for FrenchRank {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            match self {
                Self::Pip(n) => {
                    match other {
                        Self::Pip(m) => n.cmp(m),
                        _ => std::cmp::Ordering::Less
                    }
                },
                Self::Jack => {
                    match other {
                        Self::Pip(_) => std::cmp::Ordering::Greater,
                        Self::Jack => std::cmp::Ordering::Equal,
                        _ => std::cmp::Ordering::Less,
                    }
                },
                Self::Queen => {
                    match other {
                        Self::Pip(_) => std::cmp::Ordering::Greater,
                        Self::Jack => std::cmp::Ordering::Greater,
                        Self::Queen => std::cmp::Ordering::Equal,
                        Self::King => std::cmp::Ordering::Less
                    }
                },
                Self::King => {
                    match other {
                        Self::King => std::cmp::Ordering::Equal,
                        _ => std::cmp::Ordering::Greater,
                    }
                },
            }
        }
    }

    impl Display for FrenchRank {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Self::Pip(1) => write!(f, "A"),
                Self::Pip(i) => write!(f, "{}", i),
                Self::Jack => write!(f, "J"),
                Self::Queen => write!(f, "Q"),
                Self::King => write!(f, "K"),
            }
        }
    }
}

/// A module for Italian type cards
pub mod italian_card {
    use super::{ Card, Display, Rank, queue, style };

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
                Self::Swords(r) => write!(f, "{}♠", r),
                Self::Cups(r) => write!(f, "{}♥", r),
                Self::Batons(r) => write!(f, "{}♣", r),
                Self::Coins(r) => write!(f, "{}♦", r),
            }
        }
    }

    impl Card for ItalianCard {
        fn draw_que(&self, stdout: &mut std::io::Stdout) -> Result<(), std::io::Error> {
            match self {
                Self::Swords(r) => queue!(stdout, style::Print(format!("{}♠", r))),
                Self::Cups(r) => queue!(stdout, style::Print(format!("{}♥", r))),
                Self::Batons(r) => queue!(stdout, style::Print(format!("{}♣", r))),
                Self::Coins(r) => queue!(stdout, style::Print(format!("{}♦", r))),
            }
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
    pub enum ItalianRank {
        Pip(u32),
        Jack,
        Knight,
        King,
    }

    impl Rank for ItalianRank {
        fn next(&self) -> Option<Self> {
            match self {
                Self::Pip(10) => Some(Self::Jack),
                Self::Pip(n) => Some(Self::Pip(n + 1)),
                Self::Jack => Some(Self::Knight),
                Self::Knight => Some(Self::King),
                Self::King => None,
            }
        }

        fn prev(&self) -> Option<Self> {
            match self {
                Self::Pip(1) => None,
                Self::Pip(n) => Some(Self::Pip(n - 1)),
                Self::Jack => Some(Self::Pip(10)),
                Self::Knight => Some(Self::Jack),
                Self::King => Some(Self::Knight),
            }
        }
    }

    impl Display for ItalianRank {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Self::Pip(1) => write!(f, "A"),
                Self::Pip(i) => write!(f, "{}", i),
                Self::Jack => write!(f, "J"),
                Self::Knight => write!(f, "k"),
                Self::King => write!(f, "K"),
            }
        }
    }
}

/// A module for Tarocchi type cards (Italian plus the Queen rank)
pub mod tarocchi_card {
    use super::{ Card, Display, Rank, queue, style };

    #[derive(Clone, Debug)]
    pub enum TarocchiCard {
        Swords(TarocchiRank),
        Cups(TarocchiRank),
        Batons(TarocchiRank),
        Coins(TarocchiRank),
    }

    impl Card for TarocchiCard {
        fn draw_que(&self, stdout: &mut std::io::Stdout) -> Result<(), std::io::Error> {
            match self {
                Self::Swords(r) => queue!(stdout, style::Print(format!("{}♠", r))),
                Self::Cups(r) => queue!(stdout, style::Print(format!("{}♥", r))),
                Self::Batons(r) => queue!(stdout, style::Print(format!("{}♣", r))),
                Self::Coins(r) => queue!(stdout, style::Print(format!("{}♦", r))),
            }
        }
    }

    impl Display for TarocchiCard {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Self::Swords(r) => write!(f, "{}♠", r),
                Self::Cups(r) => write!(f, "{}♥", r),
                Self::Batons(r) => write!(f, "{}♣", r),
                Self::Coins(r) => write!(f, "{}♦", r),
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
                Self::Pip(1) => write!(f, "A"),
                Self::Pip(i) => write!(f, "{}", i),
                Self::Jack => write!(f, "J"),
                Self::Knight => write!(f, "k"),
                Self::Queen => write!(f, "Q"),
                Self::King => write!(f, "K"),
            }
        }
    }

    impl Rank for TarocchiRank {
        fn next(&self) -> Option<Self> {
            match self {
                Self::Pip(10) => Some(Self::Jack),
                Self::Pip(n) => Some(Self::Pip(n + 1)),
                Self::Jack => Some(Self::Knight),
                Self::Knight => Some(Self::Queen),
                Self::Queen => Some(Self::King),
                Self::King => None,
            }
        }

        fn prev(&self) -> Option<Self> {
            match self {
                Self::Pip(1) => None,
                Self::Pip(n) => Some(Self::Pip(n - 1)),
                Self::Jack => Some(Self::Pip(10)),
                Self::Knight => Some(Self::Jack),
                Self::Queen => Some(Self::Knight),
                Self::King => Some(Self::Queen),
            }
        }
    }
}

/// A module for Tarot type cards (Tarocchi plus 21 trump-suited cards)
pub mod tarot_card {
    use super::{ Card, Display, Rank, tarocchi_card::TarocchiRank, queue, style };

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

    impl Card for TarotCard {
        fn draw_que(&self, stdout: &mut std::io::Stdout) -> Result<(), std::io::Error> {
            match self {
                TarotCard::Swords(r) => queue!(stdout, style::Print(format!("{}♠", r))),
                TarotCard::Cups(r) => queue!(stdout, style::Print(format!("{}♥", r))),
                TarotCard::Batons(r) => queue!(stdout, style::Print(format!("{}♣", r))),
                TarotCard::Coins(r) => queue!(stdout, style::Print(format!("{}♦", r))),
                TarotCard::Trump(r) => queue!(stdout, style::Print(format!("{}", r))),
            }
        }
    }

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

    impl Rank for TarotTrumps {
        fn next(&self) -> Option<Self> {
            use TarotTrumps::*;
            match self {
                Fool => Some(Magician),
                Magician => Some(HighPriestess),
                HighPriestess => Some(Empress),
                Empress => Some(Emperor),
                Emperor => Some(Hierophant),
                Hierophant => Some(Lovers),
                Lovers => Some(Chariot),
                Chariot => Some(Strength),
                Strength => Some(Hermit),
                Hermit => Some(WheelOfFortune),
                WheelOfFortune => Some(Justice),
                Justice => Some(HangedMan),
                HangedMan => Some(Death),
                Death => Some(Temperance),
                Temperance => Some(Devil),
                Devil => Some(Tower),
                Tower => Some(Star),
                Star => Some(Moon),
                Moon => Some(Sun),
                Sun => Some(Judgement),
                Judgement => Some(World),
                World => None,
            }
        }

        fn prev(&self) -> Option<Self> {
            use TarotTrumps::*;
            match self {
                Fool => None,
                Magician => Some(Fool),
                HighPriestess => Some(Magician),
                Empress => Some(HighPriestess),
                Emperor => Some(Empress),
                Hierophant => Some(Emperor),
                Lovers => Some(Hierophant),
                Chariot => Some(Lovers),
                Strength => Some(Chariot),
                Hermit => Some(Strength),
                WheelOfFortune => Some(Hermit),
                Justice => Some(WheelOfFortune),
                HangedMan => Some(Justice),
                Death => Some(HangedMan),
                Temperance => Some(Death),
                Devil => Some(Temperance),
                Tower => Some(Devil),
                Star => Some(Tower),
                Moon => Some(Star),
                Sun => Some(Moon),
                Judgement => Some(Sun),
                World => Some(Judgement),
            }
        }
    }

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

/// A module for Five Crowns type cards
pub mod five_crowns_card {
    use crate::cards::french_card::FrenchRank;

    use super::{ Card, Display, queue, style, Stylize };

    #[derive(Clone, Debug)]
    pub enum FiveCrownsCard {
        Spades(FrenchRank),
        Hearts(FrenchRank),
        Clubs(FrenchRank),
        Diamonds(FrenchRank),
        Stars(FrenchRank),
        Joker,
    }

    impl Display for FiveCrownsCard {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Self::Spades(r) => write!(f, "{}♠", r),
                Self::Hearts(r) => write!(f, "\x1b[91m{}♥\x1b[37m", r),
                Self::Clubs(r) => write!(f, "\x1b[92m{}♣\x1b[37m", r),
                Self::Diamonds(r) => write!(f, "\x1b[36m{}♦\x1b[37m", r),
                Self::Stars(r) => write!(f, "\x1b[93m{}★\x1b[37m", r),
                Self::Joker => write!(f, "\x1b[91mJ\x1b[92mk\x1b[93mr\x1b[37m"),
            }
        }
    }

    impl Card for FiveCrownsCard {
        fn draw_que(&self, stdout: &mut std::io::Stdout) -> Result<(), std::io::Error> {
            match self {
                Self::Spades(r) => queue!(stdout, style::Print(format!("{}♠", r))),
                Self::Hearts(r) => queue!(stdout, style::PrintStyledContent(format!("{}♥", r).with(style::Color::Red))),
                Self::Clubs(r) => queue!(stdout, style::PrintStyledContent(format!("{}♣", r).with(style::Color::Green))),
                Self::Diamonds(r) => queue!(stdout, style::PrintStyledContent(format!("{}♦", r).with(style::Color::Blue))),
                Self::Stars(r) => queue!(stdout, style::PrintStyledContent(format!("{}★", r).with(style::Color::Yellow))),
                Self::Joker => queue!(stdout, style::Print("Jk")),
            }
        }
    }
}

/// A module for flippable cards
pub mod flippable_card {
    use super::{ Card, Display, Stdout, queue, style };
    /// A struct which adds functionality for concealing and revealing the card contained within it.
    #[derive(Clone, Debug)]
    pub struct FlippableCard<C: Card>(C, pub bool);

    impl<C: Card> FlippableCard<C> {
        pub fn new(card: C) -> FlippableCard<C> {
            FlippableCard(card, true)
        }

        /// Invert the current flip state of the card
        pub fn flip(&mut self) {
            self.1 = !self.1;
        }

        /// Set the card's flip state to face-up
        pub fn flip_face_up(&mut self) {
            self.1 = true;
        }

        /// Set the card's flip state to face-down
        pub fn flip_face_down(&mut self) {
            self.1 = false;
        }

        /// Consume the flippable card, returning the card it contained
        pub fn into_inner(self) -> C {
            self.0
        }

        /// Return a reference to the contained card
        pub fn peek_inner(&self) -> &C {
            &self.0
        }

        /// Return a mutable reference to the contained card
        pub fn inner_mut(&mut self) -> &mut C {
            &mut self.0
        }

        /// Consume the flippable card, returning the card it contained only if the flippable card was face-up
        pub fn into_inner_checked(self) -> Result<C, Self> {
            if self.1 { Ok(self.0) } else { Err(self) }
        }

        /// Return a reference to the contained card only if the flippable card is face-up
        pub fn peek_inner_checked(&self) -> Option<&C> {
            if self.1 { Some(&self.0) } else { None }
        }

        /// Return a mutable reference to the contained card only if the flippable card is face-up
        pub fn inner_mut_checked(&mut self) -> Option<&mut C > {
            if self.1 { Some(&mut self.0 )} else { None }
        }
    }

    impl<C: Card> Card for FlippableCard<C> {
        fn draw_que(&self, stdout: &mut Stdout) -> Result<(), std::io::Error> {
            if self.1 {
                self.0.draw_que(stdout)
            } else {
                queue!(stdout, style::Print("XX"))
            }
        }

        fn str_len(&self) -> usize {
            match self.1 {
                false => 2,
                true => self.0.str_len(),
            }
        }
    }

    impl<C: Card> Display for FlippableCard<C> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            if self.1 {
                write!(f, "{}", self.0)
            } else {
                write!(f, "XX")
            }
        }
    }
}

/// A module for Magic the Gathering type cards.
pub mod magic_card;