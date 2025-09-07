use std::{
    sync::mpsc,
    thread,
    time::{Duration, Instant},
};

use crossterm::event;
use ratatui::{self, DefaultTerminal};

use crate::{modules::network, ui};

pub struct App {
    pub running: bool,
    pub logs: Vec<String>,
    pub scroll: u16,
    ui_handler: ui::UiHandler,
}

impl App {
    pub fn new() -> Self {
        Self {
            running: true,
            logs: Vec::new(),
            scroll: 0,
            ui_handler: ui::UiHandler::new(),
        }
    }

    fn render_app(&mut self, tui: &mut DefaultTerminal) -> Result<()> {
        tui.draw(|frame| frame.render_widget(self, frame.area()))?;
        Ok(())
    }

    fn handle_events(&mut self) -> Result<()> {
        let frame_rate = Duration::from_secs_f64(1.0 / 60.0);
        if !event::poll(frame_rate)? {
            return Ok(());
        }
        if event::read()?.is_key_press() {
            self.running = false;
        Ok(())
    }
    
    pub fn run_app(&mut self, terminal: &mut DefaultTerminal) -> Result<()> {
        // 
        while self.running {
            self.render_app(terminal)?;
            self.handle_events()?;
        }
        Ok(())
    }


        // Kill threa d if user quits the add, so you need a tic
        let tick_rate = Duration::from_secs_f64(1.0 / 60.0);
        let mut last_tick = Instant::now();

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
            }
            if let Ok(interfaces) = rx.recv() {
                // TODO what is happing here?!
                terminal.draw(|f| ui_handler::render_ui(f, &));
            }
            input::handle_input(self);
        }
        ratatui::restore();
        Ok(())
    }
}
