use get_if_addrs::{get_if_addrs, IfAddr};

pub fn get_interfaces() -> Vec<String> {
    match get_if_addrs() {
        Ok(ifaces) => ifaces
            .into_iter()
            .map(|iface| {
                let ip = match iface.addr {
                    IfAddr::V4(ipv4) => ipv4.ip.to_string(),
                    IfAddr::V6(ipv6) => ipv6.ip.to_string(),
                };
                format!("{} ({})", iface.name, ip)
            })
            .collect(),
        Err(_) => vec!["Error, could not load ifaces!".into()],
    }
}
