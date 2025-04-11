use crossterm::event::{self, Event, KeyCode};
use get_if_addrs::{get_if_addrs, IfAddr};
use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::Span,
    widgets::{Block, Borders, List, ListItem, Paragraph},
};
use std::{
    io,
    sync::{Arc, Mutex},
    thread,
    time::{Duration, Instant},
};

fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();

    //get interfaces thread safe via mutex and shared state
    let interfaces = Arc::new(Mutex::new(Vec::new()));
    let interface_clone = Arc::clone(&interfaces);
    // get the backend thread running
    thread::spawn(move || loop {
        let data = list_interfaces();
        if let Ok(mut locked) = interface_clone.lock() {
            *locked = data;
        }
        thread::sleep(Duration::from_secs(1));
    });

    // Loop
    let result = run_app(&mut terminal, interfaces);

    // CLeanup
    //
    ratatui::restore();

    result
}

fn run_app(
    terminal: &mut ratatui::DefaultTerminal,
    interfaces: Arc<Mutex<Vec<String>>>,
) -> io::Result<()> {
    // Kill thread if user quits the add, so you need a tic
    let tick_rate = Duration::from_millis(200);
    let mut last_tick = Instant::now();

    loop {
        let items_snapshot = {
            if let Ok(locked) = interfaces.lock() {
                locked.clone()
            } else {
                vec!["Error shared state".into()]
            }
        };
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

            let header = Paragraph::new("=== Pollux sniffing ===")
                .style(Style::default().fg(Color::Yellow))
                .block(Block::default().borders(Borders::ALL).title("Header"));
            let items: Vec<ListItem> = items_snapshot
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
        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if key.code == KeyCode::Char('q') {
                    return Ok(());
                }
            }
            if last_tick.elapsed() >= tick_rate {
                last_tick = Instant::now();
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
