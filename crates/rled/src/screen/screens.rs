use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

use idm_display::{IdmCommand, IdmDisplay};
use image::Rgb;

use crate::api::PIXEL_CHANNEL;

use super::{Error, HwId, Pixel, Result, Screen, ScreenInfo};

/// Хранит все обнаруженные экраны. Внутри использует подсчет ссылок, поэтому
/// ее можно клонировать.
#[derive(Clone)]
pub struct Screens {
    screens: Arc<RwLock<HashMap<HwId, Screen>>>,
}

impl Screens {
    pub(crate) async fn new() -> Self {
        let screens = Self {
            screens: Arc::default(),
        };
        let screens_clone = screens.clone();

        tokio::spawn(async move {
            let mut rx = PIXEL_CHANNEL.1.resubscribe();
            while let Ok(event) = rx.recv().await {
                let mut screens = screens_clone.screens.write().unwrap();
                let id = event.id;
                let pixel = event.pixel;

                if let Some(screen) = screens.get_mut(&id) {
                    if let Some(display) = &screen.device {
                        display.send(IdmCommand::Grafiti {
                            color: Rgb::from(pixel.color),
                            x: pixel.x as u8,
                            y: pixel.y as u8,
                        })
                    };
                }
            }
        });

        screens
    }
    /// Добавляет экран
    /// TODO: добавить проверку что экран уже существует
    pub fn insert(&self, screen: ScreenInfo) {
        let mut screens = self.screens.write().unwrap();
        screens.insert(screen.id.clone(), screen.into());
    }

    pub fn insert_real(&self, screen_info: ScreenInfo, device: IdmDisplay) {
        let mut screens = self.screens.write().unwrap();
        let id = screen_info.id.clone();
        let mut screen: Screen = screen_info.into();
        screen.device = Some(device);
        screens.insert(id, screen);
    }

    /// Возвращает информацию об экранах в виде массива. Не включает в себя
    /// данные пикселей или строки.
    pub fn list(&self) -> Vec<ScreenInfo> {
        let screens = self.screens.read().unwrap();
        screens.values().map(|s| s.info()).collect()
    }

    /// Возвращает данные изображения экрана в виде байтов
    pub fn image(&self, id: &HwId) -> Result<Vec<u8>> {
        let screens = self.screens.read().unwrap();
        let screen = screens.get(id).ok_or(Error::NotFound)?;
        screen.image_data().map_err(|_| Error::NotGraphical)
    }

    pub fn set_pixel(&self, id: &HwId, pixel: &Pixel) -> Result<()> {
        let mut screens = self.screens.write().unwrap();
        screens
            .get_mut(id)
            .ok_or(Error::NotFound)
            .and_then(|s| s.set_pixel(pixel))
    }
}

#[cfg(test)]
mod test {
    use crate::screen::{Color, Mode, Pixel, ScreenInfo};

    use super::Screens;

    #[tokio::test]
    async fn add_one_screen() {
        let screens = Screens::new().await;
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
        screens.set_pixel(&id, &pixel).unwrap();

        let image = screens.image(id).unwrap();
        assert_eq!([1, 2, 3], image[0..3]);
    }
}
