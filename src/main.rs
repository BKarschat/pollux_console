mod app;
mod input;
mod net;
mod ui;

use crate::app::run_app;
use crate::net::get_interfaces;
use app::App;
use crossterm::event::{self, Event, KeyCode};
use get_if_addrs::{get_if_addrs, IfAddr};
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

fn main() {
    let mut terminal = ratatui::init();

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
