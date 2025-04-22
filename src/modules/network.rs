use std::sync::mpsc;
use std::thread;
use std::time::Duration;

use get_if_addrs::{get_if_addrs, IfAddr};

pub struct InterfaceData {
    interfaces: Vec<String>,
    updated: bool,
    channel_tx: mpsc::Sender<Vec<String>>,
}

impl InterfaceData {
    pub fn new(tx: mpsc::Sender<Vec<String>>) -> Self {
        Self {
            interfaces: Vec::new(),
            updated: false,
            channel_tx: tx,
        }
    }

    pub fn change_data(&self, tmp_data: Vec<String>) {
        // if data has changed store data and let it render
        if tmp_data.iter().collect() != self.interfaces.iter().collect() {
            // new data
            self.updated = true;
            self.interfaces = tmp_data;
        }
    }

    pub fn get_interfaces(&self) {
        thread::spawn(move || loop {
            match get_if_addrs() {
                Ok(ifaces) => {
                    self.interfaces = ifaces
                        .into_iter()
                        .map(|iface| {
                            let ip = match iface.addr {
                                IfAddr::V4(ipv4) => ipv4.ip.to_string(),
                                IfAddr::V6(ipv6) => ipv6.ip.to_string(),
                            };
                            format!("{} ({})", iface.name, ip)
                        })
                        .collect()
                }
                Err(_) => self.interfaces = vec!["Error, could not load ifaces!".into()],
            }
            change_data(ifaces);

            if self.updated {
                if self.channel_tx.send(self.interfaces).is_err() {
                    break;
                    //reciever is down
                }
                thread::sleep(Duration::from_secs(1));
            }
        });
    }
}
