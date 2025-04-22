use std::{
    sync::mpsc,
    thread,
    time::{Duration, Instant},
};

use ratatui;

use crate::{
    modules::network,
    ui::{network, ui},
};

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

    pub fn run_app(&mut self) -> io::Result<()> {
        // Kill threa d if user quits the add, so you need a tic
        let tick_rate = Duration::from_millis(200);
        let mut last_tick = Instant::now();

        //init ratatui terminal

        let terminal = ratatui::init();

        //init channels
        let (tx, rx) = mpsc::channel();
        let tx_clone = tx.clone();

        // init modules
        let network_interfaces = network::InterfaceData::new(tx_clone);

        // add network interfaces to the ui
        let ui_handler = ui::UiHandler::new();

        ui_handler.add_widget(Box::new(ui::network::InterfaceListWidget::new()));
        // Todo add data to UI!
        loop {
            if !self.running {
                break;
            }
            if let Ok(interfaces) = rx.recv() {
                // TODO what is happing here?!
                terminal.draw(|f| ui_handler::render_ui(f, &snapshot));
            }
            input::handle_input(self);
        }
        ratatui::restore();
        Ok(())
    }
}
