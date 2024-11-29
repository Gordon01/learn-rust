pub(crate) type Result<T> = std::result::Result<T, Error>;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("{0}")]
    BtlePlugErr(#[from] btleplug::Error),
    #[error("Powered on bluetooth adapter wasn't found")]
    AdapterNotFound,
    #[error("Required characteristic wasn't found")]
    CharacteristicNotFound,
}
