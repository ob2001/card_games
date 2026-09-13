use std::io::{Stdout, Write};

use crate::{
    cards::{
        FlippableCard, card_stack::CardStackError, deck::{Deck, DeckError, DeckToggle}, tableau::{Tableau, TableauError}
    }, lib_prelude::*,
};

type KlondikeCard = FlippableCard<FrenchCard>;
type KlondikeDeck = Deck<KlondikeCard>;
type KlondikeTableau = Tableau<KlondikeCard>;
type KlondikeFoundation = Tableau<KlondikeCard>;

#[derive(Clone, Debug)]
pub enum KlondikeGameError {
    CardError(KlondikeCard),
    DeckError(DeckError),
    TableauError(TableauError),
    CardStackError(CardStackError),
    PlayError,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum KlondikeGameElement {
    Talon,
    Tableau,
    Foundation,
}

impl KlondikeGameElement {
    /// Cycles through Klondike Game elements in a specified order
    pub fn next(&mut self) {
        *self = match self {
            KlondikeGameElement::Talon => KlondikeGameElement::Tableau,
            KlondikeGameElement::Tableau =>  KlondikeGameElement::Foundation,
            KlondikeGameElement::Foundation => KlondikeGameElement::Talon,
        }
    }
}

pub struct KlondikeGame {
    talon: KlondikeDeck,
    tableau: KlondikeTableau,
    foundation: KlondikeFoundation,
    hovered_element: KlondikeGameElement,
    selected_cards: (Vec<KlondikeCard>, KlondikeGameElement, Option<usize>),
    win: bool,
    history: Vec<KlondikeGameWeak>,
}

struct KlondikeGameWeak {
    talon: KlondikeDeck,
    tableau: KlondikeTableau,
    foundation: KlondikeFoundation,
    hovered_element: KlondikeGameElement,
    selected_cards: (Vec<KlondikeCard>, KlondikeGameElement, Option<usize>),
}

impl From<&KlondikeGame> for KlondikeGameWeak {
    fn from(value: &KlondikeGame) -> Self {
        KlondikeGameWeak {
            talon: value.talon.clone(),
            tableau: value.tableau.clone(),
            foundation: value.foundation.clone(),
            hovered_element: value.hovered_element.clone(),
            selected_cards: value.selected_cards.clone(),
        }
    }
}

impl KlondikeGame {
    pub fn new() -> Self {
        KlondikeGame {
            talon: KlondikeDeck::new_standard_french_deck(true, true),
            tableau: KlondikeTableau::new(StackVariant::HorizontalLtR, 7),
            foundation: KlondikeFoundation::new(StackVariant::Flush, 4),
            hovered_element: KlondikeGameElement::Talon,
            selected_cards: (vec![], KlondikeGameElement::Talon, None),
            win: false,
            history: vec![],
        }
    }

    // 
    pub fn init(&mut self) {
        // Gather all cards from other regions into talon for shuffling and redistribution
        self.talon.replenish_default().expect("Talon is initialized with default discard");
        self.talon.take_cards(&mut self.tableau.gather_all());
        self.talon.take_cards(&mut self.foundation.gather_all());

        // Ensure all cards are face-down before shuffling
        self.talon.all_face_down();

        // Shuffle talon (deck)
        self.talon.shuffle();

        // Draw cards from talon and play to tableau stacks 
        for i in 0..self.tableau.num_stacks() {
            let mut face_up_card = self.talon.draw().unwrap();
            face_up_card.flip_face_up();
            self.tableau.play_card_to_stack(face_up_card, i).expect("Talon should not be emptied in initial setup");

            for s in self.tableau.stacks_mut((i + 1)..7) {
                s.play_to(
                    self.talon
                        .draw()
                        .expect("Talon should not be emptied in initial setup"))
                    .expect("There should be no issues in initial deal to tableau");
            }
        }

        // Deal first 3 cards from talon
        self.draw_talon().expect("Talon should not be emptied in initial setup");

        // Deselect all elements 
        self.talon.unhover();
        self.tableau.unhover();
        self.foundation.unhover();

        // Start game with talon selected
        self.hovered_element = KlondikeGameElement::Talon;
        self.talon.hover_deck();

        // Set win state to false as this is the start of a new game
        self.win = false;

        self.history = vec![];

        self.selected_cards = (vec![], KlondikeGameElement::Talon, None);
    }

    // Draw 3 cards from talon into talon discard
    pub fn draw_talon(&mut self) -> Result<(), KlondikeGameError> {
        if self.talon.inner_deck().len() > 0 {
            for _ in 0..3 {
                self.talon.discard_default_flip().map_err(|e| KlondikeGameError::DeckError(e))?;
            }
            Ok(())
        } else {
            self.talon.replenish_default().map_err(|e | KlondikeGameError::DeckError(e))?;
            if self.talon.inner_deck().len() > 0 {
                self.talon.all_face_down();
                self.talon.reverse();
                self.draw_talon()
            } else {
                Ok(())
            }
        }
    }

    fn enter_game_screen(&mut self, stdout: &mut Stdout) -> Result<(), std::io::Error> {
        execute!(stdout,
            terminal::EnterAlternateScreen,
            cursor::Hide,
        )
    }

    fn leave_game_screen(&mut self, stdout: &mut Stdout) -> Result<(), std::io::Error> {
        execute!(stdout,
            cursor::Show,
            terminal::LeaveAlternateScreen
        )
    }

    fn clear_screen(&mut self, stdout: &mut Stdout) -> Result<(), std::io::Error> {
        execute!(stdout,
            terminal::Clear(terminal::ClearType::All),
            cursor::MoveTo(0, 0),
        )
    }

    pub fn run_game(&mut self) -> Result<(), std::io::Error> {
        let mut stdout = std::io::stdout();

        // Setup game environment in terminal, refresh screen and print initial game state
        self.enter_game_screen(&mut stdout)?;
        self.clear_screen(&mut stdout)?;
        print!("{}", self);

        // Enter interactive loop
        loop {
            if self.win {
                self.win_screen();
            } else {
                // Refresh screen and print current game state
                self.clear_screen(&mut stdout)?;
                print!("{}", self);

                // Check for any terminal events, capture any error
                if let Err(err) = match event::read() {
                    Ok(ev) => {
                        match ev {
                            // Match on keyboard events
                            Event::Key(ke) => {
                                match (ke.kind, ke.code, ke.modifiers) {
                                    // Exit interactive loop
                                    (KeyEventKind::Press, KeyCode::Esc, _) => break,

                                    // Perform selection action
                                    (KeyEventKind::Press, KeyCode::Enter, _) => self.perform_selection(),

                                    // Cycle selected game element
                                    (KeyEventKind::Press, KeyCode::Tab, _) => self.cycle_selected_game_element(),

                                    // Navigate within selected game element
                                    (KeyEventKind::Press, KeyCode::Down, _) => self.navigate_game_element_down(),
                                    (KeyEventKind::Press, KeyCode::Up, _) => self.navigate_game_element_up(),

                                    // Navigate within hovered game sub-element
                                    (KeyEventKind::Press, KeyCode::Left, _) => {
                                        if self.hovered_element == KlondikeGameElement::Tableau {
                                            if let Some(curr_stack) = self.tableau.get_hovered_stack_mut() {
                                                if let Some(c) = curr_stack.peek_prev_card() && c.1 {
                                                    curr_stack.dec_hovered_card();
                                                }
                                            }
                                        }
                                        Ok(())
                                    },
                                    (KeyEventKind::Press, KeyCode::Right, _) => {
                                        if self.hovered_element == KlondikeGameElement::Tableau {
                                            if let Some(curr_stack) = self.tableau.get_hovered_stack_mut() {
                                                if let Some(c) = curr_stack.peek_next_card() && c.1 {
                                                    curr_stack.inc_hovered_card();
                                                }
                                            }
                                        }
                                        Ok(())
                                    },
                                    (KeyEventKind::Press, KeyCode::Char('z'), KeyModifiers::NONE) => self.replace_selected_cards(),
                                    (KeyEventKind::Press, KeyCode::Char('z'), KeyModifiers::CONTROL) => { self.undo_move(); Ok(()) },
                                    (KeyEventKind::Press, KeyCode::Char('n'), _) => {
                                        self.init();
                                        continue
                                    }
                                    _ => { Ok(()) },
                                }
                            },
                            _ => { Ok(()) }
                        }
                    },
                    Err(_) => { Ok(()) }
                } {
                    // Act on any errors
                    match err {
                        KlondikeGameError::DeckError(DeckError::DrawOnEmptyDeck) => {},
                        KlondikeGameError::DeckError(DeckError::NoDefaultDiscard) => {},
                        KlondikeGameError::DeckError(DeckError::NoValidCard) => {},
                        _ => {}
                    }
                }
            }
        };

        // Clean up and restore terminal
        self.leave_game_screen(&mut stdout)
    }

    fn perform_selection(&mut self) -> Result<(), KlondikeGameError> {
        self.history.push(KlondikeGameWeak::from(&*self));
        match self.hovered_element {
            KlondikeGameElement::Talon => {
                if self.talon.is_hovered().unwrap() == DeckToggle::Deck {
                    if !self.selected_cards.0.is_empty() {
                        self.replace_selected_cards().expect("Replacing a non-empty curr_selection should be infallible");
                    }

                    self.draw_talon()
                } else if self.selected_cards.0.is_empty() {
                    if let Some(c) =  self.talon.get_top_discard() {
                        self.selected_cards = (vec![c], KlondikeGameElement::Talon, None);
                    }
                    Ok(())
                } else {
                    self.replace_selected_cards()
                }
            },
            KlondikeGameElement::Tableau => {
                // Player is attempting to pick up a card/cards from the tableau
                if self.selected_cards.0.is_empty() {
                    if let Some(sel_stack) = self.tableau.get_hovered_stack_mut() {
                        if let Some(take_stack) = sel_stack.take_hovered_stack() {
                            self.selected_cards = (take_stack.into(), KlondikeGameElement::Tableau, self.tableau.hovered_stack());
                        }
                    }
                    Ok(())
                }

                // Player is attempting to place their picked-up card/cards onto the tableau
                else if self.check_move() {
                    // Play selected cards to currently hovered stack
                    let _ = self.tableau
                        .play_cards_to_hovered_stack(&mut self.selected_cards.0)
                        .map_err(|_| KlondikeGameError::PlayError);
                    
                    // Flip any uncovered tableau card face-up
                    for stack in self.tableau.stacks_mut(0..7) {
                        if let Some(c) = stack.last_mut() {
                            c.flip_face_up();
                        }
                    }

                    // Fixes the case that the card selection gets stuck on a face-down card
                    if let Some(c) = self.tableau.get_hovered_card() && !c.1 {
                        self.tableau.get_hovered_stack_mut().unwrap().inc_hovered_card();
                    }

                    // Reset picked_up_cards
                    (self.selected_cards.1, self.selected_cards.2) = (KlondikeGameElement::Tableau, None);
                    Ok(())
                } 
                
                // Player's attempted move failed; clean up
                else {
                    self.replace_selected_cards()
                }
            },
            KlondikeGameElement::Foundation => {
                if self.selected_cards.0.is_empty() {
                    if let Some(sel_stack) = self.foundation.get_hovered_stack_mut() && sel_stack.len() > 0 {
                        if let Some(c) = sel_stack.take_hovered_card() {
                            self.selected_cards = (vec![c], KlondikeGameElement::Foundation, self.foundation.hovered_stack())
                        }
                    }
                    Ok(())
                } else if self.selected_cards.0.len() == 1 && self.check_move() {
                    self.foundation.play_cards_to_hovered_stack(&mut self.selected_cards.0).map_err(|e| KlondikeGameError::TableauError(e))?;
                    self.foundation.get_hovered_stack_mut().unwrap().inc_hovered_card();

                    // Flip any uncovered tableau card face-up
                    for stack in self.tableau.stacks_mut(0..7) {
                        if let Some(c) = stack.last_mut() {
                            c.flip_face_up();
                        }
                    }

                    Ok(())
                } else {
                    self.replace_selected_cards()
                }
            },
        }
    }

    fn replace_selected_cards(&mut self) -> Result<(), KlondikeGameError> {
        let (cs, elem, pos) = &mut self.selected_cards;
        if cs.len() > 0 {
            match elem {
                KlondikeGameElement::Talon => {
                    self.talon.play_cards_to_discard(cs).map_err(|e| KlondikeGameError::DeckError(e))?;
                },
                KlondikeGameElement::Tableau => {
                    self.tableau.play_cards_to_stack(cs, pos.expect("A selection originating from the Tableau must have an associated index")).map_err(|e| KlondikeGameError::TableauError(e))?;
                },
                KlondikeGameElement::Foundation => {
                    self.foundation.play_cards_to_stack(cs, pos.expect("A selection originating from the Foundation must have an associated index")).map_err(|e| KlondikeGameError::TableauError(e))?;
                },
            }

            self.selected_cards = (vec![], KlondikeGameElement::Talon, None);
        }
        Ok(())
    }

    fn undo_move(&mut self) {
        let state_res = self.history.pop();
        if let Some(state) = state_res {
            self.talon = state.talon;
            self.tableau = state.tableau;
            self.foundation = state.foundation;
            self.hovered_element = state.hovered_element;
            self.selected_cards = state.selected_cards;
        }
    }

    fn cycle_selected_game_element(&mut self) -> Result<(), KlondikeGameError> {
        match self.hovered_element {
            KlondikeGameElement::Talon => {
                self.talon.unhover();
                self.hovered_element.next();
                self.tableau.hover();
            },
            KlondikeGameElement::Tableau => {
                self.tableau.unhover();
                self.hovered_element.next();
                self.foundation.hover();
                let hs = self.foundation.get_hovered_stack_mut().unwrap();
                hs.set_hovered_card(hs.len().saturating_sub(1)).expect("");
            },
            KlondikeGameElement::Foundation => {
                self.foundation.unhover();
                self.hovered_element.next();
                self.talon.hover_deck();
            },
        }
        Ok(())
    }

    fn navigate_game_element_down(&mut self) -> Result<(), KlondikeGameError> {
        match self.hovered_element {
            KlondikeGameElement::Talon => {
                self.talon.toggle_deck_hover();
            },
            KlondikeGameElement::Tableau => {
                self.tableau.inc_hovered_stack();
                self.tableau.update_hovered_stack();
            },
            KlondikeGameElement::Foundation => {
                self.foundation.inc_hovered_stack();
                self.foundation.update_hovered_stack();
            },
        }
        Ok(())
    }

    fn navigate_game_element_up(&mut self) -> Result<(), KlondikeGameError> {
        match self.hovered_element {
            KlondikeGameElement::Talon => {
                self.talon.toggle_deck_hover();
            },
            KlondikeGameElement::Tableau => {
                self.tableau.dec_hovered_stack();
                self.tableau.update_hovered_stack();
            },
            KlondikeGameElement::Foundation => {
                self.foundation.dec_hovered_stack();
                self.foundation.update_hovered_stack();
            },
        }
        Ok(())
    }

    // Ensure a user-selected move is valid before performing the move
    fn check_move(&mut self) -> bool {
        use crate::cards::french_card::{FrenchCard::*, FrenchRank::*};

        if self.selected_cards.0.is_empty() {
            false
        } else {
            match self.hovered_element {

                // Rules for playing to Tableau
                KlondikeGameElement::Tableau => {
                    if let Some(c) = self.tableau.get_hovered_card() {
                        match (self.selected_cards.0.first().unwrap().peek_inner(), c.peek_inner()) {
                            (Spades(m) | Clubs(m), Hearts(n) | Diamonds(n)) => {
                                match (m, n) {
                                    (&Pip(m), &Pip(n)) => m == (n - 1),
                                    (Pip(10), Jack) => true,
                                    (Jack, Queen) => true,
                                    (Queen, King) => true,
                                    _ => false,
                                }
                            },
                            (Hearts(m) | Diamonds(m), Spades(n) | Clubs(n)) => {
                                match (m, n) {
                                    (&Pip(m), &Pip(n)) => m == (n - 1),
                                    (Pip(10), Jack) => true,
                                    (Jack, Queen) => true,
                                    (Queen, King) => true,
                                    _ => false,
                                }
                            },
                            _ => false,
                        }
                    } else {
                        match self.selected_cards.0.first() {
                            Some(fc) => {
                                match fc.peek_inner() {
                                    &Spades(r) | &Hearts(r) | &Clubs(r) | &Diamonds(r) => r == King,
                                    _ => false,
                                }
                            },
                            _ => false
                        }
                    }
                },

                // Rules for playing to Foundation
                KlondikeGameElement::Foundation => {
                    if self.selected_cards.0.len() != 1 {
                        false
                    } else {
                        if let Some(c) = self.foundation.get_hovered_card() {
                            match (self.selected_cards.0.get(0).unwrap().peek_inner(), c.peek_inner()) {
                                (Spades(m), Spades(n)) | (Hearts(m), Hearts(n)) | (Clubs(m), Clubs(n)) | (Diamonds(m), Diamonds(n)) => {
                                    match (m, n) {
                                        (&Pip(m), &Pip(n)) => { m == n + 1 },
                                        (Jack, Pip(10)) => true,
                                        (Queen, Jack) => true,
                                        (King, Queen) => {
                                            for s in self.foundation.stacks(0..4) {
                                                match s.last() {
                                                    Some(c) => {
                                                        match c.peek_inner() {
                                                            &Spades(r) | &Hearts(r) | &Clubs(r) | &Diamonds(r) => {
                                                                if r == King {
                                                                    self.win = true;
                                                                }
                                                            }
                                                            _ => {
                                                                self.win = false;
                                                                break;
                                                            }
                                                        }
                                                    },
                                                    None => {
                                                        self.win = false;
                                                        break;
                                                    }
                                                }
                                            }
                                            true
                                        },
                                        _ => false
                                    }
                                },
                                _ => false,
                            }
                        } else { 
                            match self.selected_cards.0.get(0).unwrap().peek_inner() {
                                Spades(Pip(1)) | Hearts(Pip(1)) | Clubs(Pip(1)) | Diamonds(Pip(1)) => true,
                                _ => false
                            }
                        }
                    }
                },
                _ => false,
            }
        }
    }

    fn win_screen(&mut self) {
        self.init();
        self.win = false;
    }
}

impl Debug for KlondikeGame {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,
            "{}\n\n{:?}\n{:?}",
            self.talon, self.tableau, self.foundation
        )
    }
}

// Display format for Klondike game
impl Display for KlondikeGame {
    fn fmt(&self, _: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut stdout = std::io::stdout();
        let (w, _) = terminal::size().unwrap_or((50, 50));

        let _ = queue!(stdout,
                style::Print(format!("{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n",
                crate::ui::format_box_message("Keybindings", w as usize, true),
                    " <Esc> - Exit game",
                    " <Tab> - Cycle game element selection [Talon > Tableau > Foundation]",
                    " <Up>/<Down> - Toggle Talon draw/play or select Tablueau/Foundation column",
                    " <Left>/<Right> - Select card in Tableau column",
                    " <z> - Deselect current selection",
                    " <Ctrl> + <z> - Undo move",
                    " <n> - Start new game",
                    "-".repeat(w as usize))
                ),
                style::Print(format!("{}\n\n", self.talon)),
                style::Print(format!("{}\n", self.tableau)),
                style::Print(format!("{}\n", self.foundation)),
                style::Print(format!("Current Selection: ")),
        );

        for c in &self.selected_cards.0 {
            let _ = queue!(stdout,
                style::Print(format!("{} ", c)),
            );
        }

        stdout.flush().map_err(|_| std::fmt::Error)
    }
}