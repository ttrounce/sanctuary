use std::{io::Stdout};
use crossterm::event::KeyCode;
use tui::{widgets::{Borders, Block, List, ListItem, ListState, Paragraph}, layout::{Constraint, Layout, Alignment, Direction}, style::{Style, Modifier}, Frame};
use tui::{backend::CrosstermBackend};

use crate::{
    state::{ProgramState},
};

pub fn character_sheet_draw(state: &mut ProgramState, frame: &mut Frame<CrosstermBackend<Stdout>>) {
    let cs_chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(0)
        .constraints(
            [
                Constraint::Percentage(10),
                Constraint::Percentage(80),
                Constraint::Percentage(10)
            ].as_ref()
        )
        .split(frame.size());

    let cs_header_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .margin(0)
        .constraints(
            [
                Constraint::Percentage(15),
                Constraint::Percentage(10),
                Constraint::Percentage(70),
                Constraint::Percentage(5)
            ].as_ref()
        )
        .split(cs_chunks[0]);
    
    let cs_info_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .margin(0)
        .constraints(
            [
                Constraint::Percentage(20),
                Constraint::Percentage(80)
            ].as_ref()
        )
        .split(cs_chunks[1]);    

    let hitpoints_block = Paragraph::new("10/15 +2")
        .block(Block::default()
               .title("Hit Points")
               .borders(Borders::ALL))
        .alignment(Alignment::Center);
    
    let armorclass_block = Paragraph::new("AC:11")
        .block(Block::default()
               .title("Armor Class")
               .borders(Borders::ALL))
        .alignment(Alignment::Center);

    let name_block = Paragraph::new("Player-character, Class(3) Class(1)")
        .block(Block::default()
               .title("Character")
               .borders(Borders::ALL));

    let notify_block = Paragraph::new("*!")
        .block(Block::default()
               .borders(Borders::ALL))
        .alignment(Alignment::Center);

    let mut scroll_list_state = ListState::default();
    scroll_list_state.select(Some(state.scroll_index as usize));

    let scroll_item_list = [ListItem::new("Item 0"), ListItem::new("Item 1"), ListItem::new("Item 2"), ListItem::new("Item 3")];
    state.scroll_index_max = scroll_item_list.len() as i32;
    
    let scroll_list = List::new(scroll_item_list)
        .block(Block::default().borders(Borders::ALL).title(" List "))
        .highlight_style(Style::default().add_modifier(Modifier::BOLD))
        .highlight_symbol("> ");

    let cs_item_0 = Paragraph::new("Stuff all about Item 0!")
        .block(Block::default().borders(Borders::ALL));

    let cs_item_1 = Paragraph::new("Different stuff (it's all about Item 1)!")
        .block(Block::default().borders(Borders::ALL));

    let cs_item_2 = Paragraph::new("More different stuff, but this time about Item 2.")
        .block(Block::default().borders(Borders::ALL));

    let cs_item_3 = Paragraph::new("Even more different stuff: this time about Item 3!")
        .block(Block::default().borders(Borders::ALL));

    match state.scroll_index {
        0 => frame.render_widget(cs_item_0, cs_info_chunks[1]),
        1 => frame.render_widget(cs_item_1, cs_info_chunks[1]),
        2 => frame.render_widget(cs_item_2, cs_info_chunks[1]),
        3 => frame.render_widget(cs_item_3, cs_info_chunks[1]),
        _ => ()
    }

    let footer = Block::default()
        .borders(Borders::ALL);

    frame.render_widget(hitpoints_block, cs_header_chunks[0]);

    frame.render_widget(armorclass_block, cs_header_chunks[1]);

    frame.render_widget(name_block, cs_header_chunks[2]);

    frame.render_widget(notify_block, cs_header_chunks[3]);

    frame.render_stateful_widget(scroll_list, cs_info_chunks[0], &mut scroll_list_state);

    frame.render_widget(footer, cs_chunks[2]);

}

pub fn character_sheet_key(state: &mut ProgramState, key: KeyCode) {
    match key {
        KeyCode::Char('k') | KeyCode::Up => state.scroll_index = (((state.scroll_index - 1) % state.scroll_index_max) + state.scroll_index_max) % state.scroll_index_max,
        KeyCode::Char('j') | KeyCode::Down => state.scroll_index = (state.scroll_index + 1) % state.scroll_index_max,
        _ => ()

    }
}
