use std::io;

pub(crate) type Result<T> = std::result::Result<T, Error>;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("{0}")]
    BtlePlugErr(#[from] btleplug::Error),
    #[error("Out of bounds in array")]
    OutOfBounds,
    #[error("Powered on bluetooth adapter wasn't found")]
    AdapterNotFound,
    #[error("Device wasn't found")]
    DeviceNotFound,
    #[error("Required characteristic wasn't found")]
    CharacteristicNotFound,
    #[error("{0}")]
    ImageErr(#[from] image::error::ImageError),
    #[error("{0}")]
    IoErr(#[from] io::Error),
    #[error("Gif's size is not 32x32")]
    GifErr,
    #[error("{0}")]
    IntegerOverflow(#[from] std::num::TryFromIntError),
    #[error("Invalid pixel's coordinate")]
    InvalidCoordinate,
}
