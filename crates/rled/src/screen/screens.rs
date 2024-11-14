use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

use tracing::info;

use super::{HwId, Screen, ScreenInfo};

/// Хранит все обнаруженные экраны. Внутри использует подсчет ссылок, поэтому
/// ее можно клонировать.
#[derive(Default, Clone)]
pub struct Screens {
    screens: Arc<RwLock<HashMap<HwId, Screen>>>,
}

impl Screens {
    /// Добавляет экран
    /// TODO: добавить проверку что экран уже существует
    pub fn insert(&self, screen: ScreenInfo) {
        let mut screens = self.screens.write().unwrap();
        screens.insert(screen.id.clone(), screen.into());
    }

    /// Возвращает информацию об экранах в виде массива. Не включает в себя
    /// данные пикселей или строки.
    pub fn list(&self) -> Vec<ScreenInfo> {
        let screens = self.screens.read().unwrap();
        screens.values().map(|s| s.info()).collect()
    }

    /// Возвращает данные изображения экрана в виде байтов
    /// TODO: Возвращать Result, так как ошибок может быть несколько:
    /// 1. Экран не найден
    /// 2. Экран не графический
    pub fn image(&self, id: &HwId) -> Option<Vec<u8>> {
        let screens = self.screens.read().unwrap();
        info!("HWID: {id:?}");
        screens.get(id).and_then(|s| s.image_data().ok())
    }

    /// Применяет переданную функцию на изменяемую ссылку на экран, если
    /// он найден. Иначе возвращает `None` и не делает ничего.
    pub fn map(&self, id: &HwId, f: impl FnOnce(&mut Screen)) -> Option<()> {
        let mut screens = self.screens.write().unwrap();
        screens.get_mut(id).map(f)
    }
}

#[cfg(test)]
mod test {
    use crate::screen::{Color, Mode, Pixel, ScreenInfo};

    use super::Screens;

    #[test]
    fn add_one_screen() {
        let screens = Screens::default();
        let id = "000000".try_into().unwrap();
        screens.insert(ScreenInfo {
            width: 10,
            height: Some(10),
            color: Color::Rgb32,
            mode: Mode::Graphical,
            id,
        });
        let list = screens.list();
        let id = &list[0].id;
        assert_eq!(1, list.len());

        // Установим первый пиксель и попробуем его прочитать
        let pixel = Pixel {
            x: 0,
            y: 0,
            color: [1, 2, 3],
        };
        screens.map(id, |s| {
            s.set_pixel(&pixel).unwrap();
        });

        let image = screens.image(id).unwrap();
        assert_eq!([1, 2, 3], image[0..3]);
    }
}
