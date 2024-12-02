mod canvas;
mod hw_id;
mod screens;

use poem_openapi::{Enum, Object};
use serde::Serialize;

pub use canvas::Screen;
pub use hw_id::HwId;
pub use screens::Screens;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(thiserror::Error, Debug, PartialEq, Eq)]
pub enum Error {
    #[error("Координата Х({0}) больше ширины экрана {1}")]
    #[allow(dead_code)]
    XOutOfRange(u32, u32),
    #[error("Координата Y({0}) больше высоты экрана {1}")]
    #[allow(dead_code)]
    YOutOfRange(u32, u32),
    #[error("Экран не поддерживает цвета")]
    #[allow(dead_code)]
    NoColor,
    #[error("Экран не графический")]
    NotGraphical,
    #[error("Экран не найден")]
    NotFound,
}

// Ниже располагаются структуры plain old data, которые предназначены
// пересылки между сервером и клиентом

#[derive(Clone, Debug, Object)]
pub struct ScreenInfo {
    /// Ширина в пикселях или знакоместах
    pub width: u32,
    /// Высота для графических дисплеев
    pub height: Option<u32>,
    /// Цветовой режим
    pub color: Color,
    /// Режим отображения
    pub mode: Mode,
    /// Адрес
    pub id: HwId,
}

/// Цветовой режим
#[derive(Clone, Debug, Enum)]
pub enum Color {
    /// RGB пиксели, 32 бит на пиксель
    Rgb32,
    /// Черно-белый
    BlackAndWhite,
}

/// Режим отображения или тип дисплея
#[derive(Clone, Debug, Enum, PartialEq, Eq)]
pub enum Mode {
    /// Графический
    Graphical,
    /// Символьный
    Char,
    /// Семисегментный
    SevenSeg,
}

#[derive(Debug, Object, Clone, Serialize)]
pub struct Pixel {
    pub x: u32,
    pub y: u32,
    /// Цвет в формате RGB
    pub color: [u8; 3],
}
