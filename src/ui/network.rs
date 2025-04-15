use std::{collections::HashSet, hash::Hash};
use ratatui::widgets::{Borders, List, ListItem};

use crate::ui::ui;

use super::ui::UIWidget;

pub struct InterfaceListWidget {
    ifaces: Vec<String>,
    render: bool,
}

impl InterfaceListWidget {
    pub fn new(data: Vec<String>) -> Self {
        Self {data, true}
    }
    
    pub fn change_data(tmp_data: Vec<String>) {
        // if data has changed store data and let it render
        if tmp_data.iter().collect() != self.ifaces.iter().collect() {
            // new data
            self.render = true;
            self.ifaces = tmp_data;
}
    }
}

impl UIWidget for InterfaceListWidget {
    fn render(&self, f: &mut ratatui::Frame, area: ratatui::prelude::Rect) {
        if !self.render return;
        let items = self
            .ifaces
            .iter()
            .map(|iface| ListItem::new(iface.clone()))
            .collect();

        let list = List::new(items).block(Block::default().title("Interfaces").borders(Borders::ALL));
        f.render_widget(list, area);
        self.render = false;
    }
}
