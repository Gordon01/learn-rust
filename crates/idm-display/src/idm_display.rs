use as_slice::AsSlice;
use std::{fs::File, io::Read, path::Path};
use tokio_async_drop::tokio_async_drop;

use btleplug::{
    api::{bleuuid::uuid_from_u16, Characteristic, Peripheral as ApiPeripheral, WriteType},
    platform::Peripheral,
};
use image::{GenericImageView, Rgb};
use uuid::Uuid;

use crate::{
    btle_adapter::BtleAdapter,
    command::Command,
    error::{Error, Result},
    header::{Header, HeaderType},
};

const UUID_SERVICE: Uuid = uuid_from_u16(0xfa);
const UUID_WRITE_DATA: Uuid = uuid_from_u16(0xfa02);
const DISPLAY_SIZE: (u8, u8) = (32, 32);
const CHUNK_MAX_SIZE: usize = 4096;

pub struct IdmDisplay {
    btle_device: Peripheral,
    write_data_char: Characteristic,
}

impl IdmDisplay {
    pub async fn new(device_id: &str) -> Result<Self> {
        let btle_adapter = BtleAdapter::new().await?;
        let btle_device = btle_adapter.find_device(device_id).await?;

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
            .ok_or(Error::CharacteristicNotFound)?
            .clone();

        Ok(IdmDisplay {
            btle_device,
            write_data_char,
        })
    }

    pub const fn size(&self) -> (u8, u8) {
        DISPLAY_SIZE
    }

    pub async fn gif(&self, path: &Path) -> Result<()> {
        let image = image::open(path)?;
        if image.dimensions() != (DISPLAY_SIZE.0 as u32, DISPLAY_SIZE.1 as u32) {
            return Err(Error::GifErr);
        }

        let mut file = File::open(path)?;
        let mut bytes_buffer = Vec::new();

        file.read_to_end(&mut bytes_buffer)?;

        let crc32 = crc::Crc::<u32>::new(&crc::CRC_32_ISO_HDLC).checksum(&bytes_buffer);
        let total_payload_size: u32 = bytes_buffer.len().try_into()?;

        // Split payload to chunks
        let chunks = bytes_buffer.chunks(CHUNK_MAX_SIZE);

        // Send chunks
        for (index, chunk) in chunks.enumerate() {
            let header_type = if index == 0 {
                HeaderType::Master
            } else {
                HeaderType::Secondary
            };

            let chunk_size: u16 = chunk.len().try_into()?;

            let header = Header::new(Command::Gif(
                total_payload_size,
                chunk_size,
                crc32,
                header_type,
            ));

            // data = header (master or secondary) + each chunk
            let data = [header.as_slice(), chunk].concat();
            self.send(data.as_slice()).await?;
        }

        // There is a bug in https://github.com/derkalle4/python3-idotmatrix-client
        // Need to send master header with information about crc32 and total size
        // of the gif to resolve this bug.
        let data = Header::new(Command::Gif(
            total_payload_size,
            0,
            crc32,
            HeaderType::Master,
        ));

        self.send(data.as_slice()).await
    }

    pub async fn switch_on(&self) -> Result<()> {
        let data = Header::new(Command::On);
        self.send(data.as_slice()).await
    }

    pub async fn switch_off(&self) -> Result<()> {
        let data = Header::new(Command::Off);
        self.send(data.as_slice()).await
    }

    /// `color` should be represent as code 0x00RRGGBB
    pub async fn graffiti(&self, color: Rgb<u8>, x: u8, y: u8) -> Result<()> {
        if (x, y) >= DISPLAY_SIZE {
            return Err(Error::InvalidCoordinate);
        }

        let data = Header::new(Command::Grafiti(color, x, y));
        self.send(data.as_slice()).await
    }

    async fn send(&self, data: &[u8]) -> Result<()> {
        Ok(self
            .btle_device
            .write(&self.write_data_char, data, WriteType::WithResponse)
            .await?)
    }
}

impl Drop for IdmDisplay {
    fn drop(&mut self) {
        tokio_async_drop!({
            let _ = self.btle_device.disconnect().await;
        });
    }
}
