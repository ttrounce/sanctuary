mod main_page;
mod off_page;
mod page;

use crossterm::{
    event::{self, poll, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use main_page::MainPage;
use off_page::OffPage;
use page::DrawablePage;
use std::{
    io::{self, stdout, Stdout},
    time::Duration,
};
use tui::{backend::CrosstermBackend, Terminal};

type Backend = CrosstermBackend<Stdout>;

fn main() -> Result<(), io::Error> {
    let mut terminal = create_terminal();
    start(&mut terminal)?;
    restore_terminal(terminal);
    Ok(())
}

fn start(terminal: &mut Terminal<Backend>) -> io::Result<()> {
    let mut pages: Vec<Box<dyn DrawablePage>> = vec![
        MainPage::new_box(),
        OffPage::new_box()
    ];

    let mut current_page_index: u32 = 0;
    let mut current_key_code: Option<KeyCode> = None;
    loop {
        let mut next_page: u32 = 0;

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
            let current_page = &mut pages[current_page_index as usize];
            if let Some(key) = current_key_code {
                current_page.key(key);
            }
            current_page.draw(f);
            next_page = current_page.next_page();
        })?;

        if pages.len() >= next_page as usize {
            current_page_index = next_page;
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
