pub mod ui;
pub mod network;

use ratatui::{backend::Backend, Frame};
use layout::main_layout;
use network::{draw_interfaces, draw_network_data}


