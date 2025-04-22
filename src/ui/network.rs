use ratatui::widgets::{Block, Borders, List, ListItem};
use std::{collections::HashSet, hash::Hash};

use crate::ui::ui;

use ui::UiTraitWidget;

pub struct InterfaceListWidget {
    ifaces: Vec<String>,
    render: bool,
}

impl InterfaceListWidget {
    pub fn new() -> Self {
        Self {
            ifaces: Vec::new(),
            render: true,
        }
    }
}

impl UiTraitWidget for InterfaceListWidget {
    fn render(&self, f: &mut ratatui::Frame, area: ratatui::prelude::Rect) {
        if !self.render {
            return;
        };
        let items = self
            .ifaces
            .iter()
            .map(|iface| ListItem::new(iface.clone()))
            .collect();

        let list =
            List::new(items).block(Block::default().title("Interfaces").borders(Borders::ALL));
        f.render_widget(list, area);
        self.render = false;
    }
}
