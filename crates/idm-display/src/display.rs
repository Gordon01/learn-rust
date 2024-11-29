use btleplug::{
    api::{bleuuid::uuid_from_u16, Characteristic, Peripheral as ApiPeripheral, WriteType},
    platform::Peripheral,
};

use uuid::Uuid;

use crate::{
    command::IdmCommand,
    error::{Error, Result},
};

const UUID_SERVICE: Uuid = uuid_from_u16(0xfa);
const UUID_WRITE_DATA: Uuid = uuid_from_u16(0xfa02);

#[derive(Debug, Clone)]
pub struct IdmDisplay {
    btle_device: Peripheral,
    write_data_char: Characteristic,
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

        Ok(IdmDisplay {
            btle_device,
            write_data_char,
        })
    }

    pub async fn disconnect(&self) -> Result<()> {
        Ok(self.btle_device.disconnect().await?)
    }

    pub const fn size(&self) -> (u32, u32) {
        (32, 32)
    }

    pub async fn send(&self, command: IdmCommand) -> Result<()> {
        for packet in command.to_packets() {
            self.btle_device
                .write(&self.write_data_char, &packet, WriteType::WithResponse)
                .await?;
        }

        Ok(())
    }
}
