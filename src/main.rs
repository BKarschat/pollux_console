mod app;
mod input;
mod modules;
mod ui;

use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::Span,
    widgets::{Block, Borders, List, ListItem, Paragraph},
};
use std::{
    result::Result::Ok,
    sync::{Arc, Mutex},
    thread,
    time::{Duration, Instant},
};
use App;

fn main() {
    //get interfaces thread safe via mutex and shared state
    let interfaces = Arc::new(Mutex::new(Vec::new()));
    let interfaces_clone = Arc::clone(&interfaces);
    // get the backend thread running
    thread::spawn(move || loop {
        let data = get_interfaces();
        if let Ok(mut locked) = interfaces_clone.lock() {
            *locked = data;
        }
        thread::sleep(Duration::from_secs(1));
    });

    let app = App::new();

    // Loop
    let result = app.run_app()?;

    // CLeanup
    //
    if let Err(e) = result {
        eprintln!("Error: {}", e);
    }
}
