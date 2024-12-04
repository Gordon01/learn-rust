use chrono::{NaiveDateTime, Timelike};
use image::Rgb;

const CHUNK_MAX_SIZE: usize = 4096;
const GIF_HEADER_SIZE: u16 = 16;
const MASTER_GIF_HEADER: u8 = 0;
const SECONDARY_GIF_HEADER: u8 = 2;

pub enum IdmCommand {
    SwitchOn,
    SwitchOff,
    Grafiti { color: Rgb<u8>, x: u8, y: u8 },
    Gif { bytes_buffer: Vec<u8> },
    Time { time: NaiveDateTime },
}

impl IdmCommand {
    pub(crate) fn to_packets(&self) -> Vec<Vec<u8>> {
        match self {
            Self::SwitchOn => vec![vec![0x05, 0x00, 0x07, 0x01, 1]],
            Self::SwitchOff => vec![vec![0x05, 0x00, 0x07, 0x01, 0]],
            Self::Grafiti { color, x, y } => vec![vec![
                0x0a, 0x00, 0x05, 0x01, 0x00, color.0[0], color.0[1], color.0[2], *x, *y,
            ]],

            Self::Gif { bytes_buffer } => {
                let mut packets = Vec::new();

                let payload_crc =
                    crc::Crc::<u32>::new(&crc::CRC_32_ISO_HDLC).checksum(bytes_buffer);
                let total_payload_size = bytes_buffer.len() as u32;

                // Split payload to chunks
                let chunks = bytes_buffer.chunks(CHUNK_MAX_SIZE);

                // Send chunks
                for (index, chunk) in chunks.enumerate() {
                    let header_type = if index == 0 {
                        MASTER_GIF_HEADER
                    } else {
                        SECONDARY_GIF_HEADER
                    };

                    let chunk_size = chunk.len() as u16;

                    // Prepare header
                    let packet_size = (chunk_size + GIF_HEADER_SIZE).to_le_bytes();
                    let fixed1 = [0x1, 0x0];
                    let header_type = [header_type];
                    let total_payload_size = total_payload_size.to_le_bytes();
                    let payload_crc = payload_crc.to_le_bytes();
                    let fixed2 = [0x5, 0x0, 0xD];

                    let header = [
                        packet_size.as_slice(),
                        fixed1.as_slice(),
                        header_type.as_slice(),
                        total_payload_size.as_slice(),
                        payload_crc.as_slice(),
                        fixed2.as_slice(),
                    ]
                    .concat();

                    // Prepare packet
                    let packet = [header.as_slice(), chunk].concat();

                    packets.push(packet);
                }

                // There is a bug in https://github.com/derkalle4/python3-idotmatrix-client
                // Need to send master header with information about payload_crc and total size
                // of the gif to resolve this bug.
                let packet_size = GIF_HEADER_SIZE.to_le_bytes();
                let fixed1 = [0x1, 0x0];
                let header_type = [MASTER_GIF_HEADER];
                let total_payload_size = total_payload_size.to_le_bytes();
                let payload_crc = payload_crc.to_le_bytes();
                let fixed2 = [0x5, 0x0, 0xD];

                let packet = [
                    packet_size.as_slice(),
                    fixed1.as_slice(),
                    header_type.as_slice(),
                    total_payload_size.as_slice(),
                    payload_crc.as_slice(),
                    fixed2.as_slice(),
                ]
                .concat();

                packets.push(packet);
                packets
            }
            Self::Time { time } => {
                vec![vec![
                    0x0b,
                    0x00,
                    0x01,
                    0x80,
                    0xe7,
                    0x0c,
                    0x12,
                    0x01,
                    time.hour() as u8,
                    time.minute() as u8,
                    0x10,
                ]]
            }
        }
    }
}
