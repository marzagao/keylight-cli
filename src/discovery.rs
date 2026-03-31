use mdns_sd::{ServiceDaemon, ServiceEvent};
use std::collections::HashSet;
use std::net::IpAddr;
use std::time::{Duration, Instant};

const SERVICE_TYPE: &str = "_elg._tcp.local.";

pub struct DiscoveredLight {
    pub name: String,
    pub ip: String,
    pub port: u16,
}

pub fn discover_lights(
    timeout: Duration,
) -> Result<Vec<DiscoveredLight>, Box<dyn std::error::Error>> {
    let mdns = ServiceDaemon::new()?;
    let receiver = mdns.browse(SERVICE_TYPE)?;

    let mut lights = Vec::new();
    let mut seen_names = HashSet::new();
    let start = Instant::now();

    while start.elapsed() < timeout {
        let remaining = timeout.saturating_sub(start.elapsed());
        if remaining.is_zero() {
            break;
        }

        match receiver.recv_timeout(remaining) {
            Ok(ServiceEvent::ServiceResolved(info)) => {
                let name = info.get_fullname().to_string();
                if seen_names.contains(&name) {
                    continue;
                }

                // Prefer IPv4 addresses over IPv6 for local network devices
                let ipv4 = info.get_addresses().iter().find(|a| matches!(a, IpAddr::V4(_)));
                let ip = match ipv4 {
                    Some(addr) => addr.to_string(),
                    None => match info.get_addresses().iter().next() {
                        Some(addr) => addr.to_string(),
                        None => continue,
                    },
                };

                log::info!(
                    "Discovered light: {} at {}:{}",
                    name,
                    ip,
                    info.get_port()
                );

                seen_names.insert(name.clone());
                lights.push(DiscoveredLight {
                    name,
                    ip,
                    port: info.get_port(),
                });
            }
            Ok(_) => {}
            Err(_) => break,
        }
    }

    mdns.shutdown()?;
    Ok(lights)
}
