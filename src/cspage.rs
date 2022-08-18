use std::{io::Stdout, i32::MAX};

use crossterm::event::KeyCode;
use tui::{Frame, backend::CrosstermBackend, layout::{Layout, Direction, Constraint}, widgets::{ListState, ListItem, List, Block, Borders, Paragraph}, style::{Style, Modifier}};

use crate::state::{State, ProgramState};

pub struct CharacterSheetState {
    scroll_index: i32,
    scroll_index_max: i32
}

impl State for CharacterSheetState {
    fn new() -> Self {
        CharacterSheetState { scroll_index: 0, scroll_index_max: 0  }
    }
}

pub fn character_sheet_draw(local_state: &mut CharacterSheetState, global_state: &mut ProgramState, frame: &mut Frame<CrosstermBackend<Stdout>>) {
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
    scroll_list_state.select(Some(local_state.scroll_index as usize));

    let scroll_item_list = [ListItem::new("Item 0"), ListItem::new("Item 1"), ListItem::new("Item 2"), ListItem::new("Item 3")];
    local_state.scroll_index_max = scroll_item_list.len() as i32;
    
    let scroll_list = List::new(scroll_item_list)
        .block(Block::default().borders(Borders::ALL).title(" List "))
        .highlight_style(Style::default().add_modifier(Modifier::BOLD))
        .highlight_symbol("> ");

    let cs_item_0 = Paragraph::new("Stuff all about Item 0!")
        .block(Block::default().borders(Borders::ALL));

    let cs_item_1 = Paragraph::new("Different stuff (it's all about Item 1)!")
        .block(Block::default().borders(Borders::ALL));

    match local_state.scroll_index {
        0 => frame.render_widget(cs_item_0, cs_chunks[1]),
        1 => frame.render_widget(cs_item_1, cs_chunks[1]),
        _ => ()
    }

    frame.render_stateful_widget(scroll_list, cs_chunks[0], &mut scroll_list_state);
}

pub fn character_sheet_key(local_state: &mut CharacterSheetState, state: &mut ProgramState, key: KeyCode) {
    let scroll_index_max = local_state.scroll_index_max;
    match key {
        KeyCode::Char('k') | KeyCode::Up => local_state.scroll_index = (((local_state.scroll_index - 1) % scroll_index_max) + scroll_index_max) %  scroll_index_max,
        KeyCode::Char('j') | KeyCode::Down => local_state.scroll_index = (local_state.scroll_index + 1) % local_state.scroll_index_max,
        _ => ()
    }
}