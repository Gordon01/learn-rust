mod canvas;
mod hw_id;
mod screens;

use poem_openapi::{Enum, Object};

pub use canvas::Screen;
pub use hw_id::HwId;
pub use screens::Screens;

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

#[derive(Debug, Object, Clone)]
pub struct Pixel {
    x: u32,
    y: u32,
    /// Цвет в формате RGB
    color: [u8; 3],
}
