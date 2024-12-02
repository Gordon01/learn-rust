use std::sync::Arc;

use btleplug::{
    api::{bleuuid::uuid_from_u16, Peripheral as ApiPeripheral, WriteType},
    platform::Peripheral,
};

use tokio::sync::broadcast::{self, Sender};
use uuid::Uuid;

use crate::{
    command::IdmCommand,
    error::{Error, Result},
};

const UUID_SERVICE: Uuid = uuid_from_u16(0xfa);
const UUID_WRITE_DATA: Uuid = uuid_from_u16(0xfa02);

#[derive(Debug, Clone)]
pub struct IdmDisplay {
    btle_device: Arc<Peripheral>,
    // TODO: Change for single consumer channel
    tx: Sender<Vec<u8>>,
}

impl IdmDisplay {
    pub async fn new(btle_device: Peripheral) -> Result<Self> {
        btle_device.connect().await?;
        btle_device.discover_services().await?;

        // Try to find the characteristic responsible for writing to the device
        let write_data_char = btle_device
            .characteristics()
            .iter()
            .find(|characteristic| {
                characteristic.uuid == UUID_WRITE_DATA
                    && characteristic.service_uuid == UUID_SERVICE
            })
            .ok_or(Error::CharacteristicNotFound)
            .unwrap()
            .clone();

        // TODO: Don't hardcode constants
        let (tx, mut rx) = broadcast::channel::<Vec<u8>>(1024);

        let btle_device = Arc::new(btle_device);
        let btle_device_clone = btle_device.clone();

        tokio::spawn(async move {
            while let Ok(packet) = rx.recv().await {
                // Ignore error if happened
                let _ = btle_device_clone
                    .write(&write_data_char, &packet, WriteType::WithResponse)
                    .await;
            }
        });

        Ok(IdmDisplay { btle_device, tx })
    }

    pub async fn disconnect(&self) -> Result<()> {
        Ok(self.btle_device.disconnect().await?)
    }

    pub const fn size(&self) -> (u32, u32) {
        (32, 32)
    }

    pub fn send(&self, command: IdmCommand) {
        let packets = command.to_packets();
        for packet in packets {
            self.tx
                .send(packet)
                .expect("BUG: There are no active receivers");
        }
    }

    pub fn address(&self) -> String {
        self.btle_device.address().to_string_no_delim()
    }
}
