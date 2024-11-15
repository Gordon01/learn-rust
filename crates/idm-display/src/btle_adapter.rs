use std::{
    str::FromStr,
    time::{Duration, Instant},
};

use crate::error::{Error, Result};

use btleplug::{
    api::{
        BDAddr, Central, CentralEvent, CentralState, Manager, Peripheral as ApiPeripheral,
        ScanFilter,
    },
    platform::{Adapter, Peripheral},
};
use futures::StreamExt;

pub(crate) struct BtleAdapter {
    adapter: Adapter,
}

impl BtleAdapter {
    pub(crate) async fn new() -> Result<Self> {
        let btle_manager = btleplug::platform::Manager::new().await?;
        let btle_adapters = btle_manager.adapters().await?;
        Ok(Self {
            adapter: Self::powered_on(btle_adapters).await?,
        })
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

    pub(crate) async fn find_device(&self, device_id: &str) -> Result<Peripheral> {
        // Start scanning for bluetooth devices
        self.adapter.start_scan(ScanFilter::default()).await?;
        let mut events = self.adapter.events().await?;
        let start = Instant::now();

        // 2 seconds is enough to find device
        while start.elapsed() <= Duration::from_secs(2) {
            if let Some(CentralEvent::DeviceDiscovered(id)) = events.next().await {
                // Interested in CentralEvent::DeviceDiscovered event only

                let discovered_device = self.adapter.peripheral(&id).await?;
                let properties = discovered_device.properties().await?;

                // Check if discovered device has any properties
                if let Some(discovered_property) = properties {
                    // Check if device_id is bluetooth address
                    if let Ok(device_addr) = BDAddr::from_str(device_id) {
                        if device_addr == discovered_property.address {
                            return Ok(discovered_device);
                        }

                    // Check if device_id is name of device
                    } else if let Some(discovered_name) = discovered_property.local_name {
                        if device_id == discovered_name {
                            return Ok(discovered_device);
                        }
                    }
                }
            }
        }

        Err(Error::DeviceNotFound)
    }
}
