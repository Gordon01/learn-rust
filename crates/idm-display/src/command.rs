use image::Rgb;

use crate::header::HeaderType;

pub(crate) enum Command {
    On,
    Off,
    Grafiti(Rgb<u8>, u8, u8),
    Gif(u32, u16, u32, HeaderType),
}
