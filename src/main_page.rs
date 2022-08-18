use crate::page::DrawablePage;
use crossterm::event::KeyCode;
use std::io::Stdout;
use std::str;
use tui::{
    backend::CrosstermBackend,
    layout::Alignment,
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame,
};

pub struct MainPage {
    content: Vec<u8>,
    next_page: u32,
}

impl MainPage {
    pub fn new_box() -> Box<MainPage> {
        Box::new(MainPage { content: vec![], next_page: 0 })
    }
}

impl DrawablePage for MainPage {
    fn draw(&mut self, frame: &mut Frame<CrosstermBackend<Stdout>>) {
        let core_block = Block::default()
            .borders(Borders::ALL)
            .title(" Main Page (q to exit) ");

        let paragraph = Paragraph::new(str::from_utf8(self.content.as_slice()).unwrap())
            .alignment(Alignment::Left)
            .block(core_block)
            .wrap(Wrap { trim: true });

        frame.render_widget(paragraph, frame.size());
    }

    fn key(&mut self, key: KeyCode) {
        if key == KeyCode::Char('n') {
            self.next_page = 1;
        }
    }

    fn next_page(&mut self) -> u32 {
        return self.next_page
    }
}
