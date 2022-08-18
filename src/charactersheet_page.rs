use std::io::Stdout;

use crossterm::event::KeyCode;
use tui::{
    backend::CrosstermBackend,
    widgets::{Block, Borders, Paragraph},
    Frame, layout::Alignment,
};

use crate::{
    page::DrawablePage,
    state::{State},
};

pub struct CharacterSheetPage {}

impl CharacterSheetPage {
    pub fn new_box() -> Box<CharacterSheetPage> {
        Box::new(CharacterSheetPage {})
    }
}

impl DrawablePage for CharacterSheetPage {
    fn draw(&mut self, state: &mut State, frame: &mut Frame<CrosstermBackend<Stdout>>) {        
        let b: i32 = state.retrieve_or::<i32>("potato", &0).clone();

        let my_number = state.retrieve_or::<i32>("my_number", &0).to_owned();

        let paragraph = Paragraph::new(format!("Number: {}", b))
            .block(
                Block::default()
                .borders(Borders::ALL)
                .title("Character Sheet"),
            )
            .alignment(Alignment::Center);

        frame.render_widget(paragraph, frame.size());
    }

    fn key(&mut self, state: &mut State, key: KeyCode) {
        match key {
            KeyCode::Char('p') =>
                {
                    state.store("", 10);
                },
            _ => (),
        };
        // todo!()
    }

    fn next_page(&mut self) -> &str {
        "csheet"
    }
}
