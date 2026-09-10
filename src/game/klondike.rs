use std::io::Write;

use crate::{
    cards::{
        FlippableCard,
        deck::{Deck, DeckError},
        tableau::Tableau,
    },
    lib_prelude::*,
};

type KlondikeCard = FlippableCard<FrenchCard>;
type KlondikeDeck = Deck<KlondikeCard>;
type KlondikeTableau = Tableau<KlondikeCard>;
type KlondikeFoundation = Tableau<KlondikeCard>;

#[derive(Clone, Debug)]
pub enum KlondikeGameError {
    CardError(KlondikeCard),
    DeckError(DeckError),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum KlondikeGameElement {
    Talon,
    Tableau,
    Foundation,
}

impl KlondikeGameElement {
    // Allow cycling through Klondike Game elements in a specified order
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
    curr_selection: Vec<KlondikeCard>,
}

impl KlondikeGame {
    pub fn new_game_default() -> Self {
        KlondikeGame {
            talon: KlondikeDeck::new_standard_french_deck(true, true),
            tableau: KlondikeTableau::new(StackVariant::HorizontalLtR, 7),
            foundation: KlondikeFoundation::new(StackVariant::Flush, 4),
            hovered_element: KlondikeGameElement::Talon,
            curr_selection: vec![],
        }
    }

    pub fn init_game(&mut self) {
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
            self.tableau.play_to_stack(face_up_card, i).expect("Talon should not be emptied in initial setup");

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

        // Start game with talon selected
        self.hovered_element = KlondikeGameElement::Talon;
        self.talon.select_top();
    }

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

    pub fn run_game(&mut self) -> Result<(), std::io::Error> {
        let mut stdout = std::io::stdout();
        execute!(stdout,
            terminal::EnterAlternateScreen,
            terminal::Clear(terminal::ClearType::All),
            cursor::Hide,
            cursor::MoveTo(0, 0),
            style::Print(format!("{}", self)),
        )?;

        loop {
            print!("{}", self);

            if let Err(err) = match event::read() {
                Ok(ev) => {
                    match ev {
                        Event::Key(ke) => {
                            match (ke.kind, ke.code) {
                                (KeyEventKind::Press, KeyCode::Esc) => break,
                                (KeyEventKind::Press, KeyCode::Enter) => {
                                    match self.hovered_element {
                                        KlondikeGameElement::Foundation => Ok(()),
                                        KlondikeGameElement::Tableau => Ok(()),
                                        KlondikeGameElement::Talon => {
                                            if self.talon.is_top_deck_selected() {
                                                self.draw_talon()
                                            } else {
                                                // TODO Select top card of talon "discard" to play
                                                Ok(())
                                            }
                                        },
                                    }
                                },
                                (KeyEventKind::Press, KeyCode::Tab) => {
                                    match self.hovered_element {
                                        KlondikeGameElement::Talon => {
                                            self.talon.deselect_top();
                                            self.hovered_element.next();
                                            self.tableau.activate();
                                        },
                                        KlondikeGameElement::Tableau => {
                                            self.tableau.deactivate();
                                            self.hovered_element.next();
                                            self.foundation.activate();
                                        },
                                        KlondikeGameElement::Foundation => {
                                            self.foundation.deactivate();
                                            self.hovered_element.next();
                                            self.talon.select_top();
                                        },
                                    }
                                    Ok(())
                                },
                                (KeyEventKind::Press, KeyCode::Down) => {
                                    match self.hovered_element {
                                        KlondikeGameElement::Talon => {
                                            self.talon.toggle_select_deck_discard();
                                        },
                                        KlondikeGameElement::Tableau => {
                                            self.tableau.inc_selected_stack();
                                            self.tableau.update_active_stack();
                                        },
                                        KlondikeGameElement::Foundation => {
                                            self.foundation.inc_selected_stack();
                                            self.foundation.update_active_stack();
                                        },
                                    }
                                    Ok(())
                                },
                                (KeyEventKind::Press, KeyCode::Up) => {
                                    match self.hovered_element {
                                        KlondikeGameElement::Talon => {
                                            self.talon.toggle_select_deck_discard();
                                        },
                                        KlondikeGameElement::Tableau => {
                                            self.tableau.dec_selected_stack();
                                            self.tableau.update_active_stack();
                                        },
                                        KlondikeGameElement::Foundation => {
                                            self.foundation.dec_selected_stack();
                                            self.foundation.update_active_stack();
                                        },
                                    }
                                    Ok(())
                                },
                                (KeyEventKind::Press, KeyCode::Left) => {
                                    match self.hovered_element {
                                        KlondikeGameElement::Tableau => {},
                                        _ => {},
                                    }
                                    Ok(())
                                },
                                (KeyEventKind::Press, KeyCode::Right) => {
                                    match self.hovered_element {
                                        KlondikeGameElement::Tableau => {},
                                        _ => {},
                                    }
                                    Ok(())
                                },
                                _ => { Ok(()) },
                            }
                        },
                        _ => { Ok(()) }
                    }
                },
                Err(_) => { Ok(()) }
            } {
                match err {
                    KlondikeGameError::DeckError(DeckError::DrawOnEmptyDeck) => {},
                    KlondikeGameError::DeckError(DeckError::NoDefaultDiscard) => {},
                    KlondikeGameError::DeckError(DeckError::NoValidCard) => {},
                    _ => {}
                }
            }

        };

        execute!(stdout, cursor::Show, terminal::LeaveAlternateScreen)
    }

    fn check_card_selection(&self) -> bool {
        todo!("Implement card selection rules");
    }

    fn check_move(&mut self) -> Result<(), KlondikeGameError> {
        todo!("Implement card movement/placement rules");
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

impl Display for KlondikeGame {
    fn fmt(&self, _: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut stdout = std::io::stdout();
        let _ = queue!(stdout,
                terminal::Clear(terminal::ClearType::All),
                cursor::MoveTo(0, 0),
                style::Print(format!("--------------------------------------------------------------------------\n                                Keybindings\n--------------------------------------------------------------------------\n <Tab> - Cycle game element selection [Talon > Tableau > Foundation]\n <Up>/<Down> - Toggle Talon draw/play or select Tablueau/Foundation column\n <Left>/<Right> - Select card in Tableau column\n <z> - Deselect current selection\n--------------------------------------------------------------------------\n")),
                style::Print(format!("{}\n\n", self.talon)),
                style::Print(format!("{}\n", self.tableau)),
                style::Print(format!("{}\n", self.foundation)),
        );
        for c in &self.curr_selection {
            let _ = queue!(stdout,
                style::Print(format!("{}", c)),
            );
        }
        let _ = stdout.flush();
        Ok(())
    }
}