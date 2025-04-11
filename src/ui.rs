use ratatui::{
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders, List, ListItem},
    Frame,
};

pub fn draw_ui(f: &mut Frame, items: &[String]) {
    let size = f.area();
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(1)].as_ref())
        .split(size);

    let list_items: Vec<ListItem> = items
        .iter()
        .map(|iface| ListItem::new(iface.clone()))
        .collect();

    let list = List::new(list_items).block(
        Block::default()
            .title("Network interfaces")
            .borders(Borders::ALL),
    );

    f.render_widget(list, chunks[0]);
}
