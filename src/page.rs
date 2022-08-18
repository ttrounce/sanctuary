use crossterm::event::KeyCode;
use std::io::Stdout;
use tui::{backend::CrosstermBackend, Frame};

use crate::state::State;

pub trait DrawablePage {
    fn draw(&mut self, state: &mut State, frame: &mut Frame<CrosstermBackend<Stdout>>);
    fn key(&mut self, state: &mut State, key: KeyCode);
    fn next_page(&mut self) -> &str;
}
