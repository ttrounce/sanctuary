mod cspage;
mod state;

use cspage::{character_sheet_draw, character_sheet_key, CharacterSheetState};
use crossterm::{
    event::{self, poll, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use state::{ProgramState, State};
use std::{
    io::{self, stdout, Stdout},
    time::Duration, hash::Hash, collections::{HashMap, hash_map},
};
use tui::{backend::CrosstermBackend, Terminal};

type Backend = CrosstermBackend<Stdout>;

#[derive(Debug, Eq, Hash, PartialEq, Clone, Copy)]
pub enum Page
{
    None,
    CharacterSheet
}

fn main() -> Result<(), io::Error> {
    let mut terminal = create_terminal();
    start(&mut terminal)?;
    restore_terminal(terminal);
    Ok(())
}

fn start(terminal: &mut Terminal<Backend>) -> io::Result<()> {
    let mut program_state = ProgramState {
        objects: HashMap::new(),
        current_page: Page::CharacterSheet,
        current_key: None
    };

    let mut pages = HashMap::from([
        (Page::CharacterSheet, CharacterSheetState::new())
    ]);

    loop {

        if poll(Duration::from_secs(0))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => break,
                    _ => program_state.current_key = Some(key.code),
                }
            }
        } else {
            program_state.current_key = None;
        }

        let mut local_state = pages.get_mut(&program_state.current_page).unwrap();

        terminal.draw(|f| {
            match &mut program_state.current_page {
                Page::CharacterSheet => character_sheet_draw(&mut local_state, &mut program_state, f),
                _ => ()
            }
        })?;

        if let Some(key) = program_state.current_key {
            match program_state.current_page {
                Page::CharacterSheet => character_sheet_key(&mut local_state, &mut program_state, key),
                _ => ()
            }
        }
    }
    Ok(())
}

fn create_terminal() -> Terminal<Backend> {
    enable_raw_mode().unwrap();
    execute!(stdout(), EnterAlternateScreen, EnableMouseCapture).unwrap();
    let backend = CrosstermBackend::new(stdout());
    let terminal = Terminal::new(backend).unwrap();

    return terminal;
}

fn restore_terminal(mut terminal: Terminal<Backend>) {
    disable_raw_mode().unwrap();
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )
    .unwrap();
    terminal.show_cursor().unwrap();
}
