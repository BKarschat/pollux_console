use color_eyre::{eyre::Ok, owo_colors::OwoColorize, Result};
use std::io;
use tracing::{event, info, Level};
use tracing_subscriber::FmtSubscriber;

use crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Stylize,
    symbols::border,
    text::{Line, Text},
    widgets::{Block, Paragraph, Widget},
    DefaultTerminal, Frame,
};

#[derive(Debug, Default)]
pub struct App {
    counter: u8,
    exit: bool,
}

impl App {
    /// runs the applications main loop

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    fn handle_events(&mut self) -> Result<()> {
        match crossterm::event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
            _ => {}
        };
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            KeyCode::Left => self.decrement_counter(),
            KeyCode::Right => self.increment_counter(),
            _ => {}
        }
    }

    fn exit(&mut self) {
        self.exit = true;
    }

    fn decrement_counter(&mut self) {
        // for error handling purpose use saturating_sub instead!
        self.counter += 1;
    }

    fn increment_counter(&mut self) {
        // for error handling purpose use saturating_sub instead
        self.counter -= 1;
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Line::from(" Pollux gonna sniff ".bold());
        let instructions = Line::from(vec![
            "Decrement ".into(),
            "<Left>".blue().bold(),
            " Increment ".into(),
            "<Right>".blue().bold(),
            " Quit ".into(),
            "<Q>".blue().bold(),
        ]);

        let block = Block::bordered()
            .title(title.centered())
            .title_bottom(instructions.centered())
            .border_set(border::ROUNDED);

        let counter_text = Text::from(vec![Line::from(vec![
            "Value: ".into(),
            self.counter.to_string().yellow(),
        ])]);

        Paragraph::new(counter_text)
            .centered()
            .block(block)
            .render(area, buf);
    }
}
fn main() -> Result<()> {
    // a builder for 'FmtSubscriber'
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::TRACE)
        .finish();
    tracing::subscriber::set_global_default(subscriber)
        .expect("setting the default to the subscriber failed!");

    color_eyre::install()?;
    info!("Hello, first console app in rust!");
    let main_terminal = ratatui::init();
    let result_main = run(main_terminal);
    ratatui::restore();
    result_main
}

fn run(mut terminal: DefaultTerminal) -> Result<()> {
    loop {
        terminal.draw(render)?;
        if matches!(crossterm::event::read()?, Event::Key(_)) {
            break Ok(());
        }
    }
}

fn render(frame: &mut Frame) {
    frame.render_widget("Rust App", frame.area());
}

#[cfg(test)]
mod tests {
    use crate::App;

    #[test]
    fn handle_key_event() -> std::io::Result<()> {
        let mut app = App::default();
        app.handle_key_event(crossterm::event::KeyCode::Right.into());
        assert_eq!(app.counter, 1);

        app.handle_key_event(crossterm::event::KeyCode::Left.into());
        assert_eq!(app.counter, 0);

        let mut app = App::default();
        app.handle_key_event(crossterm::event::KeyCode::Char('q').into());
        assert!(app.exit);

        Ok(())
    }
}
