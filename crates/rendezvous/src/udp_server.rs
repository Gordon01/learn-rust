use std::io;

use tokio::net::UdpSocket;
use tracing::{debug, warn};

use crate::hosts::Hosts;

pub struct UdpServer {
    pub(crate) hosts: Hosts,
    pub(crate) socket: UdpSocket,
}

impl UdpServer {
    pub async fn run(&self) -> Result<(), io::Error> {
        let mut buf = [0; 1024];

        loop {
            let (len, address) = self.socket.recv_from(&mut buf).await?;
            debug!("Received {len} bytes from {address}");

            if (0x00, 0xFF) != (buf[0], buf[len - 1]) {
                warn!("Received a message with unknown format");
                continue;
            }

            if let Ok(hostname) = String::from_utf8(buf[1..len - 1].to_vec()) {
                self.hosts.update(hostname.clone(), address);
            } else {
                warn!("Received non-UTF8 data");
            }
        }
    }
}
