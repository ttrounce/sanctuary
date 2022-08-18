use crossterm::event::KeyCode;
use std::io::Stdout;
use tui::{backend::CrosstermBackend, Frame};

pub trait DrawablePage {
    fn draw(&mut self, frame: &mut Frame<CrosstermBackend<Stdout>>);
    fn key(&mut self, key: KeyCode);
    fn next_page(&mut self) -> u32;
}
