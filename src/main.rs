<<<<<<< HEAD
use std::{io, thread, time::Duration};

use tui::{
    backend::CrosstermBackend,
    widgets::{Widget, Block, Borders, Paragraph, Wrap},
    layout::{Layout, Constraint, Direction, Alignment},
    Terminal, symbols::block
};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};


fn main() -> Result<(), io::Error> {

    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.draw(|f| {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(0)
            .constraints(
                [
                    Constraint::Percentage(10),
                    Constraint::Percentage(80),
                    Constraint::Percentage(10)
                ].as_ref()
            )
            .split(f.size());

        let subchunk = Layout::default()
            .direction(Direction::Horizontal)
            .margin(0)
            .constraints(
                [
                    Constraint::Percentage(20), 
                    Constraint::Percentage(80)
                ].as_ref()
            )
            .split(chunks[1]);
        
        let headerchunks = Layout::default()
            .direction(Direction::Horizontal)
            .margin(0)
            .constraints(
                [
                    Constraint::Percentage(90),
                    Constraint::Percentage(10)
                ].as_ref()
            )
            .split(chunks[0]);

        let footerchunks = Layout::default()
            .direction(Direction::Horizontal)
            .margin(0)
            .constraints(
                [
                    Constraint::Percentage(100)
                ].as_ref()
            )
            .split(chunks[2]);

        let paragraph = Paragraph::new("Quae dolores omnis voluptatibus quas et voluptatem velit vero. Qui dolorem aliquid quia ullam delectus dolore sit magnam. Aut dolorem modi qui voluptates laboriosam. Et molestias et omnis dicta porro porro sapiente in.")
            .alignment(Alignment::Center)
            .block(Block::default().title(" Info Box ").title_alignment(Alignment::Center).borders(Borders::ALL))
            .wrap(Wrap { trim: true });
        
        let helpblock = Block::default()
            .title(" Help ")
            .borders(Borders::ALL);
        
        let headerblock = Block::default()
            .title(" Header ")
            .borders(Borders::ALL);

        let scrollblock = Block::default()
            .title(" Scroller ")
            .borders(Borders::ALL);

        let footerparagraph = Paragraph::new("Quae dolores omnis voluptatibus quas et voluptatem velit vero.")
            .alignment(Alignment::Left)
            .block(Block::default().title(" Footer Paragraph ").borders(Borders::ALL));

        f.render_widget(paragraph, subchunk[1]);

        f.render_widget(helpblock, headerchunks[1]);

        f.render_widget(headerblock, headerchunks[0]);

        f.render_widget(scrollblock, subchunk[0]);

        f.render_widget(footerparagraph, footerchunks[0]);

    })?;

    thread::sleep(Duration::from_millis(5000));

    // restore terminal
    disable_raw_mode()?;
=======
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
>>>>>>> origin/develop
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
<<<<<<< HEAD
    )?;
    terminal.show_cursor()?;

    Ok(())
=======
    )
    .unwrap();
    terminal.show_cursor().unwrap();
>>>>>>> origin/develop
}
