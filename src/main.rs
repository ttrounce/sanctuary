mod page_character_sheet;
mod state;

use page_character_sheet::{character_sheet_key, character_sheet_draw};
use crossterm::{
    event::{self, poll, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use state::ProgramState;
use std::{
    hash::Hash,
    io::{self, stdout, Stdout},
    time::Duration,
};
use tui::{backend::CrosstermBackend, Terminal};

type Backend = CrosstermBackend<Stdout>;

#[derive(Eq, Hash, PartialEq)]
pub enum Page {
    CharacterSheet,
}

fn main() -> Result<(), io::Error> {
    let mut terminal = create_terminal();
    start(&mut terminal)?;
    restore_terminal(terminal);
    Ok(())
}

fn start(terminal: &mut Terminal<Backend>) -> io::Result<()> {
    let mut program_state = ProgramState::new();

    loop {
        if poll(Duration::from_secs(0))? {
            if let Event::Key(key) = event::read()? {
                if key.code == KeyCode::Char('q') { return Ok(()) }
                match program_state.current_page {
                    Page::CharacterSheet => character_sheet_key(&mut program_state, key.code),
                }
            }
        }

        terminal.draw(|f| match &mut program_state.current_page {
            Page::CharacterSheet => character_sheet_draw(&mut program_state, f),
        })?;
    }
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
