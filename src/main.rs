use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{
        self, disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
    },
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::Span,
    widgets::{Block, Borders, Paragraph},
    Terminal,
};
use std::io;
use std::time::Duration;

fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();

    // Loop
    let result = run_app(&mut terminal);

    // CLeanup
    //
    ratatui::restore();

    result
}

fn run_app(terminal: &mut ratatui::DefaultTerminal) -> io::Result<()> {
    loop {
        terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(1)
                .constraints([
                    Constraint::Length(3),
                    Constraint::Min(0),
                    Constraint::Length(1),
                ])
                .split(f.size());

            let header = Paragraph::new("=== Header ===")
                .style(Style::default().fg(Color::Yellow))
                .block(Block::default().borders(Borders::ALL).title("Header"));

            let body = Paragraph::new("Das ist der Body")
                .block(Block::default().borders(Borders::ALL).title("Body"));

            let footer = Paragraph::new(Span::styled(
                "Press q to quit",
                Style::default().add_modifier(Modifier::ITALIC),
            ));

            f.render_widget(header, chunks[0]);
            f.render_widget(body, chunks[1]);
            f.render_widget(footer, chunks[2]);
        })?;

        //Event handling

        if event::poll(Duration::from_millis(200))? {
            if let Event::Key(key) = event::read()? {
                if key.code == KeyCode::Char('q') {
                    return Ok(());
                }
            }
        }
    }
}
