use std::{
    sync::mpsc,
    thread,
    time::{Duration, Instant},
};

use crossterm::event::{self, KeyCode};
use ratatui::{self, DefaultTerminal};
use tokio;
use tokio_stream::StreamExt;

use crate::{modules::network, ui::*};

#[derive(Debug)]
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
        Self::default()
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

    pub async fn run(mut self, terminal: &mut DefaultTerminal) -> Result<()> {
        let frame_rate = Duration::from_secs_f64(1.0 / 60.0);
        let mut interval = tokio::time::interval(frame_rate);
        let mut events = EventStream::new();

        while self.running {
            tokio::select! {
                _ = interval.tick() => {terminal.draw(|frame| self.render(frame))?;},
                Some(Ok(event)) = events.next() => self.handle_event(&event),
            }
            //self.render_app(terminal)?;
            //self.handle_events()?;
        }
        Ok(())
    }
}
