use std::{
    sync::mpsc,
    thread,
    time::{Duration, Instant},
};

<<<<<<< HEAD
use crossterm::{event::KeyCode, Event, EventStream};
use ratatui::{self, DefaultTerminal};
use tokio_stream::StreamExt;

use crate::{modules::network, ui, input};

#[derive(Debug, Default)]
=======
use crossterm::event::{self, KeyCode};
use ratatui::{self, DefaultTerminal};
use tokio;
use tokio_stream::StreamExt;

use crate::{modules::network, ui::*};

#[derive(Debug)]
>>>>>>> a44460c13cd3186624d16190d6a401976ae7cfb2
pub struct App {
    pub running: bool,
    pub logs: Vec<String>,
    ui_handler: ui::UiHandler,
}

impl Default for App {
    fn default() -> Self {
        Self {
            running: true,
            logs: Vec::new(),
            ui_handler: ui::UiHandler::new(),
        }
    }
}

impl App {
    pub fn new() -> Self {
<<<<<<< HEAD
        Self {
            running: true,
            logs: Vec::new(),
            scroll: 0,
            ui_handler: ui::UiHandler::new()
        }
=======
        Self::default()
>>>>>>> a44460c13cd3186624d16190d6a401976ae7cfb2
    }

    fn render_app(&mut self, tui: &mut DefaultTerminal) -> Result<()> {
        tui.draw(|frame| frame.render_widget(self, frame.area()))?;
        Ok(())
    }

    fn handle_event(&mut self, event: &Event) -> Result<()> {
        if let Some(key) = event.as_key_press_event() {
            match key.code {
                KeyCode::Char('q') | KeyCode::Esc => self.running = false,
                KeyCode::Char('j') | KeyCode::Down => self.pull_r,

                _ => Ok(()),
            }
            return Ok(());
        }
        if event::read()?.is_key_press() {
            self.running = false;
        }
        Ok(())
    }
<<<<<<< HEAD
   
    fn handle_event (&mut self, event: Event) {
        if let Some(key) = event.as_key_pressed_event() {
        match key.code {
            KeyCode::Char('q') //TODO
        }

    pub async fn run(&mut self, terminal: &mut DefaultTerminal) -> Result<()> {
        // 
        // Kill threa d if user quits the add, so you need a tic
        let tick_rate = Duration::from_secs_f64(1.0 / 60.0);
        let mut interval = tokio::time::intervall(tick_rate);
        let mut events = EventStream::new();
        

        while self.running {
            tokio::select!i {
                _ = interval.tick() => { terminal.draw(|frame| self.render(frame))?; },
                Some(Ok(event)) = events.next() => handle_input(&event);
            self.render_app(terminal)?;
            self.handle_events()?;
        }
        Ok(())
    }


                //init ratatui terminal

        let terminal = ratatui::init();

        //init channels
        let (tx, rx) = mpsc::channel();
        let tx_clone = tx.clone();

        // init modules
        let network_interfaces = network::InterfaceData::new(tx_clone);

        // add network interfaces to the ui
        let ui_handler = ui::ui::UiHandler::new();

        ui_handler.add_widget(Box::new(ui::network::InterfaceListWidget::new()));
        // Todo add data to UI!
        loop {
            if !self.running {
                break;
=======

    pub async fn run(mut self, terminal: &mut DefaultTerminal) -> Result<()> {
        let frame_rate = Duration::from_secs_f64(1.0 / 60.0);
        let mut interval = tokio::time::interval(frame_rate);
        let mut events = EventStream::new();

        while self.running {
            tokio::select! {
                _ = interval.tick() => {terminal.draw(|frame| self.render(frame))?;},
                Some(Ok(event)) = events.next() => self.handle_event(&event),
>>>>>>> a44460c13cd3186624d16190d6a401976ae7cfb2
            }
            //self.render_app(terminal)?;
            //self.handle_events()?;
        }
        Ok(())
    }
}
