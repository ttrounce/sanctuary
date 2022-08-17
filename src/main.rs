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
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}
