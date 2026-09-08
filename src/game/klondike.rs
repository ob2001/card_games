use crossterm::event::{KeyCode, KeyEventKind};

use crate::{
    cards::{FlippableCard, deck::Deck, tableau::Tableau},
    lib_prelude::*,
    ui::prelude::*,
};

pub struct Klondike {
    talon: Deck<FlippableCard<FrenchCard>>,
    tableau: Tableau<FlippableCard<FrenchCard>>,
    foundation: Tableau<FlippableCard<FrenchCard>>,
}

impl Klondike {
    pub fn new_game_default() -> Self {
        Klondike {
            talon: Deck::new_standard_french_deck(true),
            tableau: Tableau::new(StackVariant::VerticalTtB, 7),
            foundation: Tableau::new(StackVariant::Flush, 4),
        }
    }

    pub fn init_game(&mut self) {
        self.talon
            .replenish_default()
            .expect("Talon is initialized with default discard");
        self.talon.take_cards(&mut self.tableau.gather_all());
        self.talon.take_cards(&mut self.foundation.gather_all());
        self.talon.all_face_down();
        self.talon.shuffle();

        for i in 0..self.tableau.num_stacks() {
            let mut face_up_card = self.talon.draw().unwrap();
            face_up_card.flip_face_up();
            self.tableau.play_to_stack(face_up_card, i).expect("Talon should not be emptied in initial setup");

            for s in self.tableau.stacks_mut((i + 1)..7) {
                s.play_to(
                    self.talon
                        .draw()
                        .expect("Talon should not be emptied in initial setup"))
                    .expect("There should be no issues in initial dealto tableau");
            }
        }

        for _ in 0..3 {
            self.talon.discard_default().expect("Talon should not be emptied in initial setup");
        }
    }

    pub fn run_game(&mut self) -> std::io::Result<()> {
        let foundation_rect = Rect::new(0, 1, 30, 1);
        let tableau_rect = Rect::new(0, 10, 150, 10);
        ratatui::run(|terminal| {
            loop {
                terminal.draw(|frame| {
                    // frame.render_widget(ratatui::widgets::, Rect::new(0, 0, 10, 1));
                    frame.render_widget(format!("{:?}", self.foundation), foundation_rect);
                    frame.render_widget(format!("{:?}", self.tableau), tableau_rect);
                })?;
                if let Ok(e) = event::read() {
                    match e {
                        event::Event::Key(ke) => match (ke.kind, ke.code) {
                            (KeyEventKind::Press, KeyCode::Esc) => break Ok(()),
                            _ => {}
                        },
                        _ => {}
                    }
                }
            }
        })
    }
}

impl std::fmt::Debug for Klondike {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}\n\n{:?}\n{:?}",
            self.talon, self.tableau, self.foundation
        )
    }
}

impl std::fmt::Display for Klondike {
    fn fmt(&self, _: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!("Display not yet implemented for Klondike");
    }
}

impl ratatui::widgets::Widget for FlippableCard<FrenchCard> {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        use crate::cards::french_card::FrenchCard;
        match self.peek_inner_checked() {
            None => Span::raw(self.to_string()).black().render(area, buf),
            Some(FrenchCard::Spades(_)) | Some(FrenchCard::Clubs(_)) => {
                Span::raw(self.to_string()).red().render(area, buf)
            }
            Some(FrenchCard::Hearts(_)) | Some(FrenchCard::Diamonds(_)) => {
                Span::raw(self.to_string()).black().render(area, buf)
            }
            _ => {}
        }
    }
}

impl ratatui::widgets::Widget for crate::cards::card_stack::CardStack<FlippableCard<FrenchCard>> {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        todo!();
    }
}

impl ratatui::widgets::Widget for crate::cards::tableau::Tableau<FlippableCard<FrenchCard>> {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        todo!()
    }
}
