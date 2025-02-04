use std::{
    collections::HashMap,
    net::{IpAddr, SocketAddr},
    sync::{Arc, RwLock},
    time::SystemTime,
};

use chrono::Utc;
use poem_openapi::Object;
use tokio::sync::oneshot;
use tracing::{debug, info, warn};

static LOCK_BUG_MSG: &str = "BUG: Got poisoned lock";

#[derive(Clone, Default)]
pub struct Hosts {
    hosts: Arc<RwLock<HashMap<String, Host>>>,
    waiters: Arc<RwLock<HashMap<String, Entry>>>,
}

#[derive(Object, Clone, Debug)]
pub struct Host {
    pub ip: IpAddr,
    pub port: u16,
    /// ISO 8601 combined date and time with time zone.
    pub last_seen: chrono::DateTime<Utc>,
}

enum Entry {
    /// Состояние ожидания: oneshot::Sender, по которому позже update() отправит значение.
    Waiting(oneshot::Sender<SocketAddr>),
    /// Кэшированное (последнее) значение.
    Cached(SocketAddr),
}

impl Hosts {
    pub fn get(&self, hostname: impl AsRef<str>) -> Option<Host> {
        let hosts = self.hosts.read().expect(LOCK_BUG_MSG);
        hosts.get(hostname.as_ref()).cloned()
    }

    // ChatGPT wrote this

    /// Non-blocking update function.
    ///
    /// When a peer sends its update (via UDP), this function:
    /// - If there's an active waiting channel for the given hostname,
    ///   it sends the new address to the waiting receiver and logs the notification.
    /// - Regardless, it updates the internal state to cache the most recent address.
    pub fn update(&self, hostname: impl ToString, address: SocketAddr) {
        let hostname = hostname.to_string();
        let mut waiters = self.waiters.write().expect("Lock poisoned");
        match waiters.remove(&hostname) {
            Some(Entry::Waiting(tx)) => {
                debug!("Sending update for {}: {}", hostname, address);
                if tx.send(address).is_err() {
                    warn!("Failed to send update for {}: receiver dropped", hostname);
                } else {
                    info!("Notified waiting peer for {}", hostname);
                }
            }
            Some(Entry::Cached(old_addr)) => {
                debug!(
                    "Replacing cached address for {}: {} -> {}",
                    hostname, old_addr, address
                );
            }
            None => {
                debug!("No existing entry for {}, inserting new value", hostname);
            }
        }
        waiters.insert(hostname.clone(), Entry::Cached(address));

        let mut hosts = self.hosts.write().expect(LOCK_BUG_MSG);
        let host = Host {
            ip: address.ip(),
            port: address.port(),
            last_seen: chrono::DateTime::from(SystemTime::now()),
        };
        hosts.insert(hostname.clone(), host);
        info!("Updated cached address for {hostname}: {address}");
    }

    /// Returns a oneshot receiver that will yield the peer's address when available.
    /// If a cached value exists, it is consumed (removed) so that wait_for is single-use.
    /// Otherwise, a new waiting channel is created to be resolved by update().
    pub fn wait_for(&self, hostname: impl ToString) -> oneshot::Receiver<SocketAddr> {
        let hostname = hostname.to_string();
        let mut waiters = self.waiters.write().expect("Lock poisoned");
        if let Some(entry) = waiters.remove(&hostname) {
            match entry {
                Entry::Cached(addr) => {
                    debug!(
                        "Found and consuming cached address for {}: {}",
                        hostname, addr
                    );
                    let (tx, rx) = oneshot::channel();
                    let _ = tx.send(addr);
                    rx
                }
                Entry::Waiting(_) => {
                    debug!("Replacing existing waiting entry for {}", hostname);
                    let (tx, rx) = oneshot::channel();
                    waiters.insert(hostname, Entry::Waiting(tx));
                    rx
                }
            }
        } else {
            debug!("No entry for {}. Creating new waiting channel.", hostname);
            let (tx, rx) = oneshot::channel();
            waiters.insert(hostname, Entry::Waiting(tx));
            rx
        }
    }
}

#[cfg(test)]
mod tests {
    use std::{net::SocketAddr, str::FromStr, time::Duration};

    use tokio::{task, time};

    use super::Hosts;

    #[tokio::test]
    async fn wait_and_register() {
        let hosts = Hosts::default();
        let peer1 = "peer1";
        let peer2 = "peer2";
        let addr1 = SocketAddr::from_str("192.168.0.1:1234").unwrap();
        let addr2 = SocketAddr::from_str("192.168.0.2:5678").unwrap();
        let timeout = Duration::from_millis(100);

        let wait1 = task::spawn({
            let hosts = hosts.clone();
            async move {
                let peer_addr = time::timeout(timeout, hosts.wait_for(peer1))
                    .await
                    .unwrap()
                    .unwrap();
                assert_eq!(addr1, peer_addr);
            }
        });
        hosts.update(peer2, addr2);

        // Simulate registration delay
        time::sleep(timeout / 3).await;

        let wait2 = task::spawn({
            let hosts = hosts.clone();
            async move {
                let peer_addr = time::timeout(timeout, hosts.wait_for(peer2))
                    .await
                    .unwrap()
                    .unwrap();
                assert_eq!(addr2, peer_addr);
            }
        });
        hosts.update(peer1, addr1);

        wait1.await.unwrap();
        wait2.await.unwrap();
    }

    #[tokio::test]
    async fn wait_is_single_use() {
        let hosts = Hosts::default();
        let peer = "peer";
        let addr1 = SocketAddr::from_str("192.168.0.1:1234").unwrap();
        let addr2 = SocketAddr::from_str("192.168.0.2:5678").unwrap();
        let timeout = Duration::from_millis(100);

        hosts.update(peer, addr1);
        let peer_addr = time::timeout(timeout, hosts.wait_for(peer)).await.unwrap();
        assert_eq!(Ok(addr1), peer_addr);
        let peer_addr = time::timeout(timeout, hosts.wait_for(peer)).await;
        assert!(peer_addr.is_err()); // Check for timeout

        hosts.update(peer, addr2);
        let peer_addr = time::timeout(timeout, hosts.wait_for(peer)).await.unwrap();
        assert_eq!(Ok(addr2), peer_addr);
    }
}
