use std::{
    io,
    sync::{Arc, Mutex},
    time::{Duration, Instant},
};

use ratatui;

use crossterm::{event::{self, Event, KeyCode}, terminal};

use crate::ui::draw_ui;

pub struct App {
    pub running: bool,
    pub logs: Vec<String>,
    pub scroll: u16,
}

impl App {
    pub fn new() -> Self {
        running: true;
        logs: vec![""];
        scroll: 0;
    }

    pub fn run_app(&mut self) -> io::Result<()> {
        // Kill threa d if user quits the add, so you need a tic
        let tick_rate = Duration::from_millis(200);
        let mut last_tick = Instant::now();
        
        //init ratatui terminal

        let terminal = ratatui::init();
        loop {
            if !self.running { break;}
            ui::draw(&mut terminal, self);
            input::handle_input(self);
            };
ratatui::restore();
   Ok(()) 
        }
   
}
