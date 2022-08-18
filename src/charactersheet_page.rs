use crossterm::event::KeyCode;
use tui::{widgets::{Borders, Block, List, ListItem, ListState, Paragraph}, layout::{Constraint, Layout, Direction, Alignment}, style::{Style, Modifier}};

use crate::page::DrawablePage;

pub struct CharacterSheetPage {
    scroll_index: i32,
    scroll_index_max: i32
}

impl CharacterSheetPage {
    pub fn new_box() -> Box<CharacterSheetPage> {
        Box::new(CharacterSheetPage {scroll_index: 0, scroll_index_max: 0})
    }
}

impl DrawablePage for CharacterSheetPage {
    fn draw(&mut self, frame: &mut tui::Frame<tui::backend::CrosstermBackend<std::io::Stdout>>) {
        let cs_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .margin(0)
            .constraints(
                [
                    Constraint::Percentage(20),
                    Constraint::Percentage(80)
                ].as_ref()
            )
            .split(frame.size());
        
        let mut scroll_list_state = ListState::default();
        scroll_list_state.select(Some(self.scroll_index as usize));

        let scroll_item_list = [ListItem::new("Item 0"), ListItem::new("Item 1"), ListItem::new("Item 2"), ListItem::new("Item 3")];
        self.scroll_index_max = scroll_item_list.len() as i32;
        
        let scroll_list = List::new(scroll_item_list)
            .block(Block::default().borders(Borders::ALL).title(" List "))
            .highlight_style(Style::default().add_modifier(Modifier::BOLD))
            .highlight_symbol("> ");

        let cs_item_0 = Paragraph::new("Stuff all about Item 0!")
            .block(Block::default().borders(Borders::ALL));

        let cs_item_1 = Paragraph::new("Different stuff (it's all about Item 1)!")
            .block(Block::default().borders(Borders::ALL));

        match self.scroll_index {
            0 => frame.render_widget(cs_item_0, cs_chunks[1]),
            1 => frame.render_widget(cs_item_1, cs_chunks[1]),
            _ => ()
        }

        frame.render_stateful_widget(scroll_list, cs_chunks[0], &mut scroll_list_state);

    }

    fn key(&mut self, key: crossterm::event::KeyCode) {
        match key {
            KeyCode::Char('k') | KeyCode::Up => self.scroll_index = (((self.scroll_index - 1) % self.scroll_index_max) + self.scroll_index_max) % self.scroll_index_max,
            KeyCode::Char('j') | KeyCode::Down => self.scroll_index = (self.scroll_index + 1) % self.scroll_index_max,
            _ => ()

        }
    }

    fn next_page(&mut self) -> u32 {
        0
    }
}