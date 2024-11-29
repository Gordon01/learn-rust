use crate::{
    error::{Error, Result},
    IdmDisplay,
};

use btleplug::{
    api::{Central, CentralEvent, CentralState, Manager, Peripheral as ApiPeripheral, ScanFilter},
    platform::Adapter,
};
use futures::StreamExt;
use tokio::sync::broadcast::{self, Receiver, Sender};

const PREFIX: &str = "IDM-";
const MAX_DISPLAYS_NUMBER: usize = 3;

pub struct IdmService {
    rx: Receiver<IdmDisplay>,
}

impl IdmService {
    pub async fn new() -> Result<Self> {
        let btle_manager = btleplug::platform::Manager::new().await?;
        let btle_adapters = btle_manager.adapters().await?;
        let btle_adapter = Self::powered_on(btle_adapters).await?;

        let (tx, rx) = broadcast::channel(MAX_DISPLAYS_NUMBER);

        // Spawn thread that will search for devices and send them into `tx`
        tokio::spawn(async move { Self::find_device(btle_adapter, tx).await });

        Ok(Self { rx })
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

    async fn find_device(btle_adapter: Adapter, tx: Sender<IdmDisplay>) {
        // Start scanning for bluetooth devices
        btle_adapter
            .start_scan(ScanFilter::default())
            .await
            .expect("BUG: Couldn't start scanning");
        let mut events = btle_adapter
            .events()
            .await
            .expect("BUG: Couldn't get events from bluetooth adapter");

        // FIXME: Rewrite it more simple and clean
        while let Some(event) = events.next().await {
            match event {
                CentralEvent::DeviceDiscovered(id) => {
                    if let Ok(discovered_device) = btle_adapter.peripheral(&id).await {
                        // Check if discovered device has any properties
                        if let Ok(Some(discovered_property)) = discovered_device.properties().await
                        {
                            // Check if discovered device has local name
                            if let Some(discovered_name) = discovered_property.local_name {
                                if discovered_name.contains(PREFIX) {
                                    tx.send(
                                        IdmDisplay::new(discovered_device)
                                            .await
                                            // FIXME: Maybe better use if let Ok(display) = IdmDisplay::new(discovered_device)
                                            .expect("BUG: Couldn't create `IdmDisplay`"),
                                    )
                                    .expect("BUG: There are no active receivers");
                                }
                            }
                        }
                    }
                }

                // TODO for Yana: `Implement DeviceDisconnected(PeripheralId)` event
                _ => continue,
            }
        }
    }

    pub async fn subscribe(&self) -> Receiver<IdmDisplay> {
        self.rx.resubscribe()
    }
}
