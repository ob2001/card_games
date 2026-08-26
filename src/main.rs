pub mod card {
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
}

pub mod deck {
    use std::fmt::Display;
    use rand::seq::IteratorRandom;
    use crate::card::CardSuitRank;

    #[derive(Clone, Debug)]
    pub struct Deck<CardSet> {
        deck: Vec<CardSet>,
        default_discard: Option<Vec<CardSet>>,
    }

    impl<CardSet: Clone> Deck<CardSet> {
        pub fn new_empty() -> Self {
            Deck { deck: vec![], default_discard: None }
        }

        pub fn draw(&mut self) -> Result<CardSet, &str> {
            self.deck.pop().ok_or("")
        }

        pub fn shuffle(&mut self) {
            for i in (1..self.deck.len()).rev() {
                let j = rand::random_range(0..i + 1);
                self.deck.swap(i, j)
            }
        }

        pub fn replenish(&mut self, other: Option<&mut Deck<CardSet>>, force_other: bool) {
            self.deck.reverse();

            if let Some(discard) = &mut self.default_discard && !force_other {
                self.deck.append(discard);
            } else if let Some(other) = other {
                self.deck.append(&mut other.deck);
            } else {
                panic!();
            }
            
            self.deck.reverse();
        }

        pub fn push(&mut self, card: CardSet) {
            self.deck.push(card);
        }

        pub fn get_inner_deck(&self) -> &Vec<CardSet> {
            &self.deck
        }

        pub fn get_inner_deck_mut(&mut self) -> &mut Vec<CardSet> {
            &mut self.deck
        }
    }

    impl Deck<CardSuitRank> {
        pub fn new_standard_french_deck() -> Deck<CardSuitRank> {
            use crate::card::{CardSuitRank, FrenchRank::{self, *}, FrenchSuit::*};
            let mut deck = vec![];
            for suit in [Spades, Hearts, Clubs, Diamonds] {
                for i in 1..11 {
                    deck.push(CardSuitRank::new(suit.clone(), FrenchRank::Pip(i)));
                }

                for rank in [Jack, Queen, King] {
                    deck.push(CardSuitRank::new(suit.clone(), rank));
                }
            }

            Deck { deck, default_discard: Some(vec![]) }
        }
    }

    impl Display for Deck<CardSuitRank> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            for card in &self.deck {
                writeln!(f, "{}", card)?;
            }
            Ok(())
        }
    }
}

pub mod hand {
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
}

pub mod stack {
    #[derive(Clone, Debug)]
    pub struct Stack<T: Clone> {
        stack: Vec<T>,
        lim: Option<usize>
    }

    impl<T: Clone> Stack<T> {
        pub fn new(lim: Option<usize>) -> Self {
            Stack { stack: vec![], lim }
        }
    }
}

pub mod tableau {
    use crate::stack::Stack;

    #[derive(Debug)]
    pub struct Tableau<T: Clone> {
        tableau: Vec<Stack<T>>,
    }

    impl<T: Clone> Tableau<T> {
        pub fn new(cols: usize) -> Self {
            Tableau { tableau: vec![Stack::new(None); cols] }
        }
    }
}

pub mod player {
    use crate::{
        PlayableTo, card::CardSuitRank, deck::Deck, hand::Hand,
    };

    #[derive(Debug)]
    pub struct Player {
        name: String,
        hand: Hand,
    }

    impl Player {
        pub fn new(name: String) -> Self {
            Player { name, hand: Hand::new_empty() }
        }

        pub fn draw_from(&mut self, deck: &mut Deck<CardSuitRank>) {
            if let Ok(card) = deck.draw() {
                self.hand.hand.push(card);
            } else {
                deck.replenish(None, false);
                deck.shuffle();
                self.hand.hand.push(deck.draw().expect("Empty deck"));
            }
        }

        pub fn play_card_to(&mut self, card: CardSuitRank, playable: &mut impl PlayableTo) {
            playable.play_to(card);
        }

        pub fn discard_to(&mut self, card: CardSuitRank, discard: &mut Deck<CardSuitRank>) {
            discard.push(card);
        }
    }
}

pub mod klondike {
    use crate::{
        card::CardSuitRank,
        deck::Deck,
        player::Player,
        tableau::Tableau,
    };

    #[derive(Debug)]
    pub struct Klondike {
        player: Player,
        talon: Deck<CardSuitRank>,
        talon_discard: Deck<CardSuitRank>,
        tableau: Tableau<CardSuitRank>,
        foundation: Tableau<CardSuitRank>,
    }

    impl Klondike {
        pub fn new_game_default() -> Self {
            Klondike { 
                player: Player::new(String::from("Player1")),
                talon: Deck::new_standard_french_deck(),
                talon_discard: Deck::new_empty(),
                tableau: Tableau::new(7),
                foundation: Tableau::new(4),
            }
        }

        pub fn init_game(&mut self) {
            for c in self.talon.get_inner_deck_mut() {
                c.flip_face_down();
            }
            self.talon.shuffle();
        }

        pub fn run_game(&mut self) {
            todo!();
        }

    }

    impl std::fmt::Display for Klondike {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            todo!("Display not yet implemented for Klondike");
        }    
    }
}

pub trait PlayableTo {
    fn play_to(&mut self, card: card::CardSuitRank);
}

fn main() {
    use crate::deck::Deck;

    let mut deck = Deck::new_standard_french_deck();
    println!("{}", deck);
    deck.shuffle();
    println!("{}", deck);
}