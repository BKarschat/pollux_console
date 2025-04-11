use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{
        self, disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
    },
};
use get_if_addrs::{get_if_addrs, IfAddr};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::Span,
    widgets::{Block, Borders, List, ListItem, Paragraph},
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
        // get actual interfaces and ip addresses
        // could make everything slow...
        let interfaces = list_interfaces();

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
            let items: Vec<ListItem> = interfaces
                .iter()
                .map(|iface| ListItem::new(iface.clone()))
                .collect();

            let body = List::new(items).block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Network Interfaces"),
            );

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

fn list_interfaces() -> Vec<String> {
    match get_if_addrs() {
        Ok(ifaces) => ifaces
            .into_iter()
            .map(|iface| {
                let ip = match iface.addr {
                    IfAddr::V4(ipv4) => ipv4.ip.to_string(),
                    IfAddr::V6(ipv6) => ipv6.ip.to_string(),
                };
                format!("{} ({})", iface.name, ip)
            })
            .collect(),
        Err(_) => vec!["Fehler beim Laden der Ifaces! ".into()],
    }
}
