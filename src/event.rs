use crate::app::App;
use crossterm::event::{self, Event, KeyCode};
use std::time::Duration;

pub fn handle_input(app: &mut App) -> Result<(), Box<dyn std::error::Error>> {
    if event::poll(Duration::from_millis(100))? {
        if let Event::Key(key) = event::read() {
            match key.code {
                KeyCode::Char('q') => app.running = false,
                KeyCode::Down => app.scroll += 1,
                KeyCode::Up => app.scroll += 1,
                //TODO Enter
                _ => {}
            }
        }
    }
    OK(())
}
