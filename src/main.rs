mod charactersheet_page;
mod page;
mod state;

use charactersheet_page::CharacterSheetPage;
use crossterm::{
    event::{self, poll, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use page::DrawablePage;
use state::State;
use std::{
    collections::HashMap,
    io::{self, stdout, Stdout},
    time::Duration,
};
use tui::{backend::CrosstermBackend, Terminal};

type Backend = CrosstermBackend<Stdout>;

trait DrawableWidget {}

fn main() -> Result<(), io::Error> {
    let mut terminal = create_terminal();
    start(&mut terminal)?;
    restore_terminal(terminal);
    Ok(())
}

fn start(terminal: &mut Terminal<Backend>) -> io::Result<()> {
    let mut state = State {
        objects: HashMap::new(),
    };
    let mut pages = HashMap::new();

    pages.insert(String::from("csheet"), CharacterSheetPage::new_box());

    let mut current_page_name = String::from("csheet");
    let mut current_key_code: Option<KeyCode> = None;
    loop {
        let mut next_page = String::new();

        if poll(Duration::from_millis(10))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => break,
                    _ => current_key_code = Some(key.code),
                }
            }
        } else {
            current_key_code = None;
        }

        terminal.draw(|f| {
            let current_page = pages.get_mut(&current_page_name).unwrap();
            if let Some(key) = current_key_code {
                current_page.key(&mut state, key);
            }
            current_page.draw(&mut state, f);
            next_page = current_page.next_page().to_owned();
        })?;

        if pages.contains_key(next_page.as_str()) {
            current_page_name = next_page;
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
