use std::sync::Arc;

use crate::{
    error::{Error, Result},
    IdmDisplay,
};

use btleplug::{
    api::{Central, CentralEvent, CentralState, Manager, Peripheral as ApiPeripheral, ScanFilter},
    platform::{Adapter, Peripheral, PeripheralId},
};
use futures::StreamExt;
use tokio::sync::broadcast::{self, Receiver, Sender};

const PREFIX: &str = "IDM-";
const MAX_DISPLAYS_NUMBER: usize = 3;

pub struct IdmService {
    tx: Sender<IdmServiceEvent>,
    rx: Receiver<IdmServiceEvent>,
    btle_adapter: Adapter,
}

#[derive(Debug, Clone)]
pub enum IdmServiceEvent {
    Connected(IdmDisplay),
    Disconnected(String),
}

impl IdmService {
    pub async fn new() -> Result<Arc<Self>> {
        let btle_manager = btleplug::platform::Manager::new().await?;
        let btle_adapters = btle_manager.adapters().await?;
        let btle_adapter = Self::powered_on(btle_adapters).await?;

        let (tx, rx) = broadcast::channel(MAX_DISPLAYS_NUMBER);
        let idm_service = Arc::new(Self {
            tx,
            rx,
            btle_adapter,
        });

        let cloned = idm_service.clone();

        // Spawn thread that will be scanning for devices and will send events into `tx`
        tokio::spawn(async move { cloned.scan_devices().await });

        Ok(idm_service)
    }

    /// Choose first powered on bluetooth adapter
    async fn powered_on(adapters: Vec<Adapter>) -> Result<Adapter> {
        for adapter in adapters {
            if let Ok(state) = adapter.adapter_state().await {
                if state == CentralState::PoweredOn {
                    return Ok(adapter);
                }
            }
        }

        Err(Error::AdapterNotFound)
    }

    async fn scan_devices(&self) {
        // Start scanning for bluetooth devices
        self.btle_adapter
            .start_scan(ScanFilter::default())
            .await
            .expect("BUG: Couldn't start scanning");

        let mut events = self
            .btle_adapter
            .events()
            .await
            .expect("BUG: Couldn't get events from bluetooth adapter");

        while let Some(event) = events.next().await {
            match event {
                CentralEvent::DeviceDiscovered(id) => {
                    if let Some(device) = self.idm_device(id).await {
                        if let Ok(display) = IdmDisplay::new(device).await {
                            self.tx
                                .send(IdmServiceEvent::Connected(display))
                                .expect("BUG: There are no active receivers");
                        }
                    }
                }

                CentralEvent::DeviceDisconnected(id) => {
                    if let Some(device) = self.idm_device(id).await {
                        let addr = device.address().to_string_no_delim();
                        self.tx
                            .send(IdmServiceEvent::Disconnected(addr))
                            .expect("BUG: There are no active receivers");
                    }
                }
                _ => continue,
            }
        }
    }

    async fn idm_device(&self, id: PeripheralId) -> Option<Peripheral> {
        // Check if `btle_device` with this `id` has been discovered
        if let Ok(btle_device) = self.btle_adapter.peripheral(&id).await {
            // Check if discovered `btle_device` has any properties
            if let Ok(Some(btle_property)) = btle_device.properties().await {
                // Check if discovered `btle_device` has local name
                if let Some(btle_device_name) = btle_property.local_name {
                    if btle_device_name.contains(PREFIX) {
                        return Some(btle_device);
                    }
                }
            }
        }

        None
    }

    pub async fn subscribe(&self) -> Receiver<IdmServiceEvent> {
        self.rx.resubscribe()
    }
}
