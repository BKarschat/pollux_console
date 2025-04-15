use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    widgets::{Block, Borders, List, ListItem},
    Frame,
};

pub trait UiWidget {
    fn render(&self, f: &mut Frame, area: Rect);
}

struct UiHandler {
    widgets: Vec<Box<dyn UIWidget>>,
}

impl UiHandler {
    pub fn new() -> Self {
        Self {
            widgets: Vec::new(),
        }
    }

    pub fn add_widget(&mut self, widget: Box<dyn UIWidget>) {
        self.widgets.push(widget);
    }

    // should handle the ui chunks, and they should know how to render their content
    pub fn render_ui(&self, frame: &mut Frame, areas: &[Rect]) {
        for (i, widget) in self.widgets.iter().enumerate() {
            if let Some(area) = areas.get(i) {
                widget.render(frame, *area);
            }
        }
    }
}
