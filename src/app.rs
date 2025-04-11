use std::{
    io,
    sync::{Arc, Mutex},
    time::{Duration, Instant},
};

use ratatui;

use crossterm::event::{self, Event, KeyCode};

use crate::ui::draw_ui;

pub fn run_app(
    terminal: &mut ratatui::DefaultTerminal,
    interfaces: Arc<Mutex<Vec<String>>>,
) -> io::Result<()> {
    // Kill threa d if user quits the add, so you need a tic
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
        terminal.draw(|f| draw_ui(f, &items_snapshot))?;

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
