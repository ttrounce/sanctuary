use crate::page::DrawablePage;
use crossterm::event::KeyCode;
use std::io::Stdout;
use tui::{
    backend::CrosstermBackend,
    layout::Alignment,
    widgets::{Block, Borders, Paragraph},
    Frame,
};

pub struct OffPage {

}

impl OffPage {
    pub fn new_box() -> Box<OffPage> {
        Box::new(OffPage {})
    }
}

impl DrawablePage for OffPage {
    fn draw(&mut self, frame: &mut Frame<CrosstermBackend<Stdout>>) {
        let core_block = Block::default()
            .borders(Borders::ALL)
            .title(" Off Page (q to exit) ");

        let paragraph =
            Paragraph::new("This is another window. Press 'n' to return to the main window.")
                .alignment(Alignment::Center)
                .block(core_block);

        frame.render_widget(paragraph, frame.size());
    }

    fn key(&mut self, _key: KeyCode) {
        
    }

    fn next_page(&mut self) -> u32 {
        1
    }
}
