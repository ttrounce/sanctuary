use tui::widgets::{Borders, Block};

use crate::page::DrawablePage;

pub struct CharacterSheetPage {

}

impl CharacterSheetPage {
    pub fn new_box() -> Box<CharacterSheetPage> {
        Box::new(CharacterSheetPage {})
    }
}

impl DrawablePage for CharacterSheetPage {
    fn draw(&mut self, frame: &mut tui::Frame<tui::backend::CrosstermBackend<std::io::Stdout>>) {
        let cs_main_block = Block::default()
        .borders(Borders::ALL);
    }

    fn key(&mut self, key: crossterm::event::KeyCode) {
        // todo!()
    }

    fn next_page(&mut self) -> u32 {
        0
    }
}