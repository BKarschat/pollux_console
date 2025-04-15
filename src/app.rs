use ui::ui;
use modules::network;
use std::{
    time::{Duration, Instant},
    sync::{Arc, Mutex},
};

use ratatui;



pub struct App {
    pub running: bool,
    pub logs: Vec<String>,
    pub scroll: u16,
    ui_handler: UiHandler,
}

impl App {
    pub fn new() -> Self {
        Self {running: true, logs:Vec::new(), scroll: 0, ui_handler: UiHandler::new()}
    }

    pub fn run_app(&mut self) -> io::Result<()> {
        // Kill threa d if user quits the add, so you need a tic
        let tick_rate = Duration::from_millis(200);
        let mut last_tick = Instant::now();
        
        //init ratatui terminal

        let terminal = ratatui::init();
        
        // init modules
        let network_interfaces = network::I

        // start getting async data to display

        let interfaces = Arc::new(Mutex::new(Vec::new()));
        let interface_clone = Arc::clone(&interfaces);
        thread::spawn(move || loop {
            let data = get_interfaces();
        // add network interfaces to the ui
        ui_hander::add_widget(InterfaceListWidget::new(Vec<String>::new()));
         
        loop {
            if !self.running { break;}

            terminal.draw(|f| ui_handler::render_ui(f, &snapshot));
            input::handle_input(self);
            };
ratatui::restore();
   Ok(()) 
        }
   
}
