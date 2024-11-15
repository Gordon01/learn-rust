use std::{mem::size_of, ptr::slice_from_raw_parts};

use as_slice::AsSlice;
use image::Rgb;

use crate::command::Command;

const FIXED1: [u8; 2] = [0x1, 0x0];
const FIXED2: [u8; 3] = [0x5, 0x0, 0xD];
const MASTER_HEADER: u8 = 0x0;
const SECANDARY_HEADER: u8 = 0x2;

const SWITCH_ON: [u8; 5] = [0x05, 0x00, 0x07, 0x01, 1];
const SWITCH_OFF: [u8; 5] = [0x05, 0x00, 0x07, 0x01, 0];
const GRAFFITI_MODE: [u8; 5] = [0x0a, 0x00, 0x05, 0x01, 0x00];

pub(crate) struct Header {
    inner: Box<dyn AsSlice<Element = u8>>,
}

impl Header {
    pub(crate) fn new(cmd: Command) -> Self {
        match cmd {
            Command::On => Self {
                inner: Box::new(SwitchOnHeader::new()),
            },
            Command::Off => Self {
                inner: Box::new(SwitchOffHeader::new()),
            },
            Command::Grafiti(color, x, y) => Self {
                inner: Box::new(GraffitiModeHeader::new(color, x, y)),
            },
            Command::Gif(total_payload_size, chunk_size, payload_crc, header_type) => Self {
                inner: Box::new(GifModeHeader::new(
                    total_payload_size,
                    chunk_size,
                    payload_crc,
                    header_type,
                )),
            },
        }
    }
}

impl AsSlice for Header {
    type Element = u8;

    fn as_slice(&self) -> &[Self::Element] {
        self.inner.as_slice()
    }
}

#[allow(dead_code)]
#[repr(packed)]
struct GifModeHeader {
    total_chunk_size: u16,
    fixed1: [u8; 2],
    header_type: u8,
    total_payload_size: u32,
    payload_crc: u32,
    fixed2: [u8; 3],
}

pub(crate) enum HeaderType {
    Master,
    Secondary,
}

impl GifModeHeader {
    fn new(
        total_payload_size: u32,
        chunk_size: u16,
        payload_crc: u32,
        header_type: HeaderType,
    ) -> Self {
        match header_type {
            HeaderType::Master => Self {
                // `total_chunk_size = chunk size + header size`
                total_chunk_size: (chunk_size + size_of::<Self>() as u16).to_le(),
                total_payload_size: total_payload_size.to_le(),
                payload_crc: payload_crc.to_le(),
                ..Default::default()
            },
            HeaderType::Secondary => Self {
                total_chunk_size: (chunk_size + size_of::<Self>() as u16).to_le(),
                total_payload_size: total_payload_size.to_le(),
                payload_crc: payload_crc.to_le(),
                header_type: SECANDARY_HEADER.to_le(),
                ..Default::default()
            },
        }
    }
}

impl Default for GifModeHeader {
    fn default() -> Self {
        Self {
            total_chunk_size: 0,
            fixed1: FIXED1,
            header_type: MASTER_HEADER.to_le(),
            total_payload_size: 0x0,
            payload_crc: 0x0,
            fixed2: FIXED2,
        }
    }
}

impl AsSlice for GifModeHeader {
    type Element = u8;

    fn as_slice(&self) -> &[Self::Element] {
        // Cast reference &GifModeHeader to raw pointer *const GifModeHeader and then to *const u8
        let ptr = (self as *const Self).cast::<u8>();
        let ptr_to_slice = slice_from_raw_parts(ptr, size_of::<Self>());

        // SAFETY: Since pointer to slice and length of slice are correct
        unsafe { ptr_to_slice.as_ref() }.expect("BUG: NULL pointer occured")
    }
}

struct SwitchOnHeader {
    header: [u8; 5],
}

impl SwitchOnHeader {
    fn new() -> Self {
        Self { header: SWITCH_ON }
    }
}

impl AsSlice for SwitchOnHeader {
    type Element = u8;

    fn as_slice(&self) -> &[Self::Element] {
        self.header.as_slice()
    }
}

struct SwitchOffHeader {
    header: [u8; 5],
}

impl SwitchOffHeader {
    fn new() -> Self {
        Self { header: SWITCH_OFF }
    }
}

impl AsSlice for SwitchOffHeader {
    type Element = u8;

    fn as_slice(&self) -> &[Self::Element] {
        self.header.as_slice()
    }
}

#[allow(dead_code)]
#[repr(packed)]
struct GraffitiModeHeader {
    header: [u8; 5],
    red: u8,
    green: u8,
    blue: u8,
    x: u8,
    y: u8,
}

impl GraffitiModeHeader {
    fn new(color: Rgb<u8>, x: u8, y: u8) -> Self {
        Self {
            header: GRAFFITI_MODE,
            red: color.0[2],
            green: color.0[1],
            blue: color.0[0],
            x,
            y,
        }
    }
}

impl AsSlice for GraffitiModeHeader {
    type Element = u8;

    fn as_slice(&self) -> &[Self::Element] {
        // Cast reference &GraffitiModeHeader to raw pointer *const GraffitiModeHeader and then to *const u8
        let ptr = (self as *const Self).cast::<u8>();
        let ptr_to_slice = slice_from_raw_parts(ptr, size_of::<Self>());

        // SAFETY: Since pointer to slice and length of slice are correct
        unsafe { ptr_to_slice.as_ref() }.expect("BUG: NULL pointer occured")
    }
}
