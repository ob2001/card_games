use crate::{
    cards::{
        Rank, deck::{ Deck, DeckError, DeckToggle }, french_card::{ FrenchCard, FrenchRank }, tableau::{ Tableau, TableauError, TableauVariant },
    }, lib_prelude::*, ui::*,
};

type KlondikeCard = FlippableCard<FrenchCard>;
type KlondikeDeck = Deck<KlondikeCard>;
type KlondikeTableau = Tableau<KlondikeCard>;
type KlondikeFoundation = Tableau<KlondikeCard>;

#[derive(Clone, Debug)]
pub enum KlondikeGameError {
    CardError(KlondikeCard),
    TalonError(DeckError),
    TableauError(TableauError),
    FoundationError(TableauError),
    PlayError,
    HistoryError
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum KlondikeGameElement {
    Talon,
    Tableau,
    Foundation,
}

impl KlondikeGameElement {
    /// Cycles through Klondike Game elements in a specified order
    pub fn next_element(&mut self) {
        *self = match self {
            KlondikeGameElement::Talon => KlondikeGameElement::Tableau,
            KlondikeGameElement::Tableau =>  KlondikeGameElement::Foundation,
            KlondikeGameElement::Foundation => KlondikeGameElement::Talon,
        }
    }
}

#[derive(Debug)]
pub struct KlondikeGame {
    talon: KlondikeDeck,
    tableau: KlondikeTableau,
    foundation: KlondikeFoundation,
    hovered_element: KlondikeGameElement,
    selected_cards: (Vec<KlondikeCard>, KlondikeGameElement, Option<usize>),
    win: bool,
    history: Vec<KlondikeGameWeak>,
}

#[derive(Clone, Debug)]
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
            talon: KlondikeDeck::new_standard_french_deck(true, true, Some((0, 12))),
            tableau: KlondikeTableau::new(TableauVariant::VerticalTtB, CardStackVariant::HorizontalLtR, 7, Some((0, 15))),
            foundation: KlondikeFoundation::new(TableauVariant::VerticalTtB, CardStackVariant::Flush, 4, Some((0, 23))),
            hovered_element: KlondikeGameElement::Talon,
            selected_cards: (vec![], KlondikeGameElement::Talon, None),
            win: false,
            history: vec![],
        }
    }

    pub fn init(&mut self) {
        self.talon.set_num_display_deck(1);
        self.talon.set_num_display_discard(3);

        // Gather all cards from other regions into talon for shuffling and redistribution
        self.talon.replenish_default().expect("Talon is initialized with default discard");
        self.talon.add_cards(&mut self.tableau.gather_all());
        self.talon.add_cards(&mut self.foundation.gather_all());
        self.talon.add_cards(&mut self.selected_cards.0);

        // Ensure all cards are face-down before shuffling
        self.talon.all_face_down();

        // Shuffle talon (deck)
        self.talon.shuffle();

        // Draw cards from talon and play to tableau stacks 
        for i in 0..self.tableau.num_stacks() {
            let mut face_up_card = self.talon.draw_card().unwrap();
            face_up_card.flip_face_up();
            self.tableau.play_card_to_stack(face_up_card, i).expect("Talon should not be emptied in initial setup");

            for s in self.tableau.stacks_mut((i + 1)..7).expect("There are 7 stacks in the Tableau") {
                s.play_to(
                    self.talon
                        .draw_card()
                        .expect("Talon should not be emptied in initial setup"))
                    .expect("There should be no issues in initial deal to tableau");
            }
        }

        // Deal first 3 cards from talon
        self.draw_from_talon().expect("Talon should not be emptied in initial setup");

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

    pub fn draw_imm(&self, stdout: &mut Stdout) -> Result<(), std::io::Error> {
        self.draw_que(stdout)?;
        stdout.flush()
    }

    /// Queue drawing the game to the passed `stdout`.
    /// Drawing will be performed the next time `stdout` is `flush()`ed
    pub fn draw_que(&self, stdout: &mut Stdout) -> Result<(), std::io::Error> {
        let (w, _h) = terminal::size().unwrap_or((50, 50));

        // Draw game instructions
        queue!(stdout,
            cursor::MoveTo(0, 0),
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
        )?;
        
        // Draw each dedicated game element using its designated draw() function
        self.talon.draw_que(stdout)?;

        queue!(stdout, cursor::MoveToNextLine(2))?;

        self.tableau.draw_que(stdout)?;

        queue!(stdout, cursor::MoveToNextLine(1))?;

        self.foundation.draw_que(stdout)?;

        // Draw the selected cards
        queue!(stdout, cursor::MoveToColumn(0), cursor::MoveDown(0), style::Print("Current selection: "))?;
        for c in self.selected_cards.0.iter().take(self.selected_cards.0.len().saturating_sub(1)) {
            c.draw_que(stdout)?;
            queue!(stdout, cursor::MoveRight(1))?;
        }
        if let Some(c) = self.selected_cards.0.last() {
            c.draw_que(stdout)?;
        }
        queue!(stdout, terminal::Clear(terminal::ClearType::UntilNewLine))?;

        Ok(())
    }

    /// Draw 3 cards from talon into talon discard
    pub fn draw_from_talon(&mut self) -> Result<(), KlondikeGameError> {
        if self.talon.inner_deck().len() > 0 {
            for _ in 0..3 {
                self.talon.top_deck_discard_default_flip().map_err(|e| KlondikeGameError::TalonError(e))?;
            }
            Ok(())
        } else {
            self.talon.replenish_default().map_err(|e | KlondikeGameError::TalonError(e))?;
            if self.talon.inner_deck().len() > 0 {
                self.talon.all_face_down();
                self.talon.reverse();
                self.draw_from_talon()
            } else {
                Ok(())
            }
        }
    }

    /// Handles setup and tear down of terminal environment as well as
    /// receiving and processing terminal events and updating internal game
    /// logic and terminal display as required
    pub fn run_game(&mut self) -> Result<(), std::io::Error> {
        let mut stdout = std::io::stdout();

        // Set up terminal environment, draw initial game state
        enter_game_screen(&mut stdout)?;
        self.draw_imm(&mut stdout)?;

        // Enter interactive loop
        loop { match self.win {
            // Print the winning screen if the win condition has been achieved
            true => { execute!(stdout, terminal::Clear(terminal::ClearType::All))?; self.win_screen();},
            // Otherwise, continue with interactive gameplay 
            false => {
                // Refresh screen and draw current game state
                self.draw_imm(&mut stdout)?;

                // Wait for any terminal event and act on it, capture any errors that may arise
                if let Err(err) = match event::read() {
                    Ok(ev) => {
                        match ev {
                            // Match on keyboard events
                            Event::Key(ke) => {
                                match (ke.kind, ke.code, ke.modifiers) {
                                    // Break out of interactive loop
                                    (KeyEventKind::Press, KeyCode::Esc, _) => break,

                                    // Perform selection action
                                    (KeyEventKind::Press, KeyCode::Enter, _) => self.perform_selection(),

                                    // Cycle game element selection
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
                                    (KeyEventKind::Press, KeyCode::Char('z'), KeyModifiers::CONTROL) => self.undo_move(),
                                    (KeyEventKind::Press, KeyCode::Char('n'), _) => {
                                        self.init();
                                        execute!(stdout, terminal::Clear(terminal::ClearType::All))?;
                                        continue
                                    }
                                    _ => { Ok(()) },
                                }
                            },
                            _ => { Ok(()) }
                        }
                    },
                    Err(_) => { Ok(()) }
                } { /* Act on any captured errors */ match err { _ => {} } }
            } };
        }
        
        // Clean up and restore terminal
        leave_game_screen(&mut stdout)
    }

    // ? Documented
    /// Analyzes the context in which the player made their selection and performs the correct
    /// game action, checking the legality of any card moves before performing them.
    fn perform_selection(&mut self) -> Result<(), KlondikeGameError> {
        self.history.push(KlondikeGameWeak::from(&*self));
        match self.hovered_element {
            KlondikeGameElement::Talon => {
                // Player is attempting to deal cards from the Talon
                if self.talon.is_hovered().expect("Talon (as a whole) is guaranteed to be hovered") == DeckToggle::Deck {
                    // Selecting Talon with cards selected is considered a mismove.
                    // Replace selected cards before dealing from Talon.
                    if !self.selected_cards.0.is_empty() {
                        self.replace_selected_cards().expect("Replacing a non-empty curr_selection should be infallible");
                    }

                    self.draw_from_talon()
                } 
                // Player is attempting to take the top card from the Talon
                 else if self.selected_cards.0.is_empty() {
                    if let Some(c) =  self.talon.draw_discard() {
                        self.selected_cards = (vec![c], KlondikeGameElement::Talon, None);
                    }
                    Ok(())
                }
                // Any other circumstance that the player may select the Talon is a mismove.
                // Replace any currently selected cards as a default.
                else {
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
                // Player is attempting to place their currently selected card/cards onto the tableau
                else if self.check_move_legality() {
                    // Play selected cards to currently hovered stack
                    self.tableau
                        .play_cards_to_hovered_stack(&mut self.selected_cards.0)
                        .map_err(|e| KlondikeGameError::TableauError(e))?;
                    
                    // Flip any uncovered tableau card face-up
                    for stack in self.tableau.stacks_mut(0..7).expect("There are 7 stacks in the Tableau") {
                        if let Some(c) = stack.last_mut() {
                            c.flip_face_up();
                        }
                    }

                    // Fixes the case that the card selection gets stuck on a face-down card
                    while let Some(c) = self.tableau.get_hovered_card() && !c.1 {
                        self.tableau.get_hovered_stack_mut().unwrap().inc_hovered_card();
                    }

                    // Reset selected_cards
                    self.selected_cards = (vec![], KlondikeGameElement::Tableau, None);
                    Ok(())
                } 
                // Player's attempted move failed; replace any selected cards as a default
                else {
                    self.replace_selected_cards()
                }
            },
            KlondikeGameElement::Foundation => {
                // Player is attempting to take a cardfrom the Foundation
                if self.selected_cards.0.is_empty() {
                    if let Some(sel_stack) = self.foundation.get_hovered_stack_mut() && sel_stack.len() > 0 {
                        if let Some(c) = sel_stack.take_hovered_card() {
                            self.selected_cards = (vec![c], KlondikeGameElement::Foundation, self.foundation.hovered_stack())
                        }
                    }
                    Ok(())
                }
                // Player is attempting to play a card to the Foundation
                // It is only allowed to play a single card to the Foundation at a time
                else if self.selected_cards.0.len() == 1 && self.check_move_legality() {
                    self.foundation.play_cards_to_hovered_stack(&mut self.selected_cards.0).map_err(|e| KlondikeGameError::TableauError(e))?;
                    self.foundation.get_hovered_stack_mut().unwrap().inc_hovered_card();

                    // Flip any uncovered tableau card face-up
                    for stack in self.tableau.stacks_mut(0..7).expect("There are 7 stacks in the Tableau") {
                        if let Some(c) = stack.last_mut() {
                            c.flip_face_up();
                        }
                    }

                    Ok(())
                }
                // Any other circumstance that the player may select the Foundation is a mismove.
                // Replace any currently selected cards as a default.
                else {
                    self.replace_selected_cards()
                }
            },
        }
    }

    // ? Documented
    /// Moves the cards stores in `selected_cards` back to where they came from
    fn replace_selected_cards(&mut self) -> Result<(), KlondikeGameError> {
        // Unpack the information from selected_cards
        let (cards, elem, idx) = &mut self.selected_cards;

        // Ensure that there really were cards selected
        if cards.len() > 0 {
            match elem {
                KlondikeGameElement::Talon => {
                    self.talon.play_cards_to_discard(cards).map_err(|e| KlondikeGameError::TalonError(e))?;
                },
                KlondikeGameElement::Tableau => {
                    self.tableau.play_cards_to_stack(cards, idx.expect("A selection originating from the Tableau must have an associated index")).map_err(|e| KlondikeGameError::TableauError(e))?;
                },
                KlondikeGameElement::Foundation => {
                    self.foundation.play_cards_to_stack(cards, idx.expect("A selection originating from the Foundation must have an associated index")).map_err(|e| KlondikeGameError::FoundationError(e))?;
                },
            }

            // Reset `selected_cards`
            self.selected_cards = (vec![], KlondikeGameElement::Talon, None);
        }
        Ok(())
    }

    // ? Documented
    /// Restore the game state to the immediately preceeding game state
    fn undo_move(&mut self) -> Result<(), KlondikeGameError> {
        let state_res = self.history.pop();

        // Ensure that there is a previous game state before restoring it
        if let Some(state) = state_res {
            self.talon = state.talon;
            self.tableau = state.tableau;
            self.foundation = state.foundation;
            self.hovered_element = state.hovered_element;
            self.selected_cards = state.selected_cards;
            Ok(())
        } else {
            Err(KlondikeGameError::HistoryError)
        }
    }

    // ? Documented
    /// Cycle through game elements in a predictable order.
    /// Upon cycling, ensure that any subelements are "hovered" and "unhovered" as required
    fn cycle_selected_game_element(&mut self) -> Result<(), KlondikeGameError> {
        match self.hovered_element {
            // Cycle from the Talon to the Tableau
            // Unhover Talon and hover Tableau
            KlondikeGameElement::Talon => {
                self.talon.unhover();
                self.hovered_element.next_element();
                self.tableau.hover();
            },
            // Cycle from the Tableau to the Foundation
            // Unhover Tableau and hover Foundation
            KlondikeGameElement::Tableau => {
                self.tableau.unhover();
                self.hovered_element.next_element();
                self.foundation.hover();
                // let hs = self.foundation.get_hovered_stack_mut().expect("The Foundation was just hovered over");
                // hs.set_hovered_card(hs.len().saturating_sub(1)).expect("");
            },
            // Cycle from Foundation back to the Talon
            // Unhover Foundation and hover Talon
            KlondikeGameElement::Foundation => {
                self.foundation.unhover();
                self.hovered_element.next_element();
                self.talon.hover_deck();
            },
        }
        Ok(())
    }

    // ? Documented
    /// Shift the hovered position within the currently hovered game element corresponding with the
    /// <Down> key.
    /// Due to wrapping/toggling behaviour within game elements, this action cannot produce an error.
    fn navigate_game_element_down(&mut self) -> Result<(), KlondikeGameError> {
        match self.hovered_element {
            // Toggle between hovering the Talon deck and the Talon discard
            KlondikeGameElement::Talon => {
                self.talon.toggle_deck_hover();
            },
            // Step to next Tableau stack, wrapping from the end to the beginning
            KlondikeGameElement::Tableau => {
                self.tableau.inc_hovered_stack();
                self.tableau.update_hovered_stack();
            },
            // Step to next Foundation stack, wrapping from the end to the beginning
            KlondikeGameElement::Foundation => {
                self.foundation.inc_hovered_stack();
                self.foundation.update_hovered_stack();
            },
        }
        Ok(())
    }

    // ? Documented
    /// Shift the hovered position within the currently hovered game element corresponding with the
    /// <Up> key.
    /// Due to wrapping/toggling behaviour within game elements, this action cannot produce an error.
    fn navigate_game_element_up(&mut self) -> Result<(), KlondikeGameError> {
        match self.hovered_element {
            // Toggle between hovering the Talon deck and the Talon discard
            KlondikeGameElement::Talon => {
                self.talon.toggle_deck_hover();
            },
            // Step to the previous Tableau stack, wrapping from the beginning to the end
            KlondikeGameElement::Tableau => {
                self.tableau.dec_hovered_stack();
                self.tableau.update_hovered_stack();
            },
            // Step to the previous Foundation stack, wrapping from the beginning to the end
            KlondikeGameElement::Foundation => {
                self.foundation.dec_hovered_stack();
                self.foundation.update_hovered_stack();
            },
        }
        Ok(())
    }

    // ? Documented
    // Ensure the legality of the currently attempted move
    fn check_move_legality(&mut self) -> bool {
        // If no cards are currently selected, no attempted move is possible
        if self.selected_cards.0.first().is_none() { return false }

        let first_selected_card = self.selected_cards.0.first().expect("Already checked for case that current selection is empty").peek_inner();
        match self.hovered_element {
            // Rules for playing one or more cards to Tableau
            KlondikeGameElement::Tableau => {
                // For the case that the currently hovered Tableau stack contains at least
                // one card. In this case, we must check that the colour-alternating and 
                // rank-descending Tableau stack rules are met
                if let Some(hovered_card) = self.tableau.get_hovered_card() {
                    let hovered_card = hovered_card.peek_inner();
                    // Assuming that all cards in current selection are already legally ordered
                    //      (this is guaranteed inductively from the initial state of the game
                    //      and the checking that each Tableau movement is legal)
                    // we only need to check that the "top" card of the selection may legally be
                    // played onto the "bottom" card of the currently hovered Tableau stack
                    //
                    // A "Black" card may only be played on a "Red" card or vice versa and...
                    first_selected_card.is_black() == hovered_card.is_red() &&
                    // ... a card may only be played onto a card one rank higher than it
                    first_selected_card.rank() == hovered_card.rank().expect("No Jokers in card set").prev()
                }
                // For the case that the currently hovered Tableau stack contains no cards. In this
                // case we need only check that the "top" card of the current selection is a King
                // (of any Suit).
                else { first_selected_card.rank().expect("No Jokers in card set") == FrenchRank::King }
            },
            // Rules for playing to Foundation
            // May only play one card to the Foundation at a time
            KlondikeGameElement::Foundation => {
                if let Some(hovered_card) = self.foundation.get_hovered_card() {
                    let hovered_card = hovered_card.peek_inner();
                    // First check whether move is legal. Predicate validity of a win check on legality of move
                    let ret = first_selected_card.suit_equals(&hovered_card) && hovered_card.rank().expect("No Jokers in card set").next() == first_selected_card.rank();

                    // Only check win conditions if the card being played to the Foundation is a King
                    if ret && first_selected_card.rank() == Some(FrenchRank::King) { self.check_win_conds(); }

                    // Return the legality of the move
                    ret
                } 
                // For the case that the currently hovered Foundation stack contains no cards. In this
                // case we need only check that the currently selected card is an Ace (of any Suit).
                else { first_selected_card.rank() == Some(FrenchRank::Pip(1)) }
            },
            // No card/s may be moved to the Talon
            KlondikeGameElement::Talon => false,
        }
    }

    // ? Documented
    /// Check that there is a King at the top of each Foundation stack
    /// Triggered only when (legally) playing a King to a Foundation stack
    fn check_win_conds(&mut self) {
        self.win = self.foundation
            .stacks(0..4)
            .expect("There are 4 stacks in the Foundation")
            .iter()
            .all(|s| 
                s.last().map_or(false, |c| {
                    c.peek_inner()
                    .rank()
                    .expect("No Jokers in card set") == FrenchRank::King
                }
            )
        );
    }

    // TODO
    fn win_screen(&mut self) {
        let _ = event::read();
        self.init();
        self.win = false;
    }
}

/// Display format for Klondike game
/// Deprecated for interactive use - Prefer using draw_que() or draw_imm() methods
impl Display for KlondikeGame {
    fn fmt(&self, _: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut stdout = std::io::stdout();
        let (w, _h) = terminal::size().unwrap_or((50, 50));

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