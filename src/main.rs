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

        let subchunk = Layout::default().direction(Direction::Horizontal).margin(0).constraints([Constraint::Percentage(20), Constraint::Percentage(80)].as_ref()).split(chunks[1]);
            
        let paragraph = Paragraph::new("Quae dolores omnis voluptatibus quas et voluptatem velit vero. Qui dolorem aliquid quia ullam delectus dolore sit magnam. Aut dolorem modi qui voluptates laboriosam. Et molestias et omnis dicta porro porro sapiente in.")
            .alignment(Alignment::Center)
            .block(Block::default().borders(Borders::ALL))
            .wrap(Wrap { trim: true });

        f.render_widget(paragraph, chunks[2])
    })?;

    thread::sleep(Duration::from_millis(5000));

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}
