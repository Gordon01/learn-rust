use idm_display::IdmDisplay;

use super::{Color, Error, Mode, Pixel, Result, ScreenInfo};

/// Один экран
pub struct Screen {
    info: ScreenInfo,
    /// Значения пикселей в этом экране в формате RGB
    image: Option<Vec<u8>>,
    pub(crate) device: Option<IdmDisplay>,
}

impl Screen {
    pub fn info(&self) -> ScreenInfo {
        self.info.clone()
    }

    pub fn image_data(&self) -> Result<Vec<u8>> {
        self.image.clone().ok_or(Error::NotGraphical)
    }

    /// Устанавливает значение пикселя на экране
    pub fn set_pixel(&mut self, pixel: &Pixel) -> Result<()> {
        if self.info.mode != Mode::Graphical {
            return Err(Error::NotGraphical);
        }
        if pixel.x >= self.info.width {
            return Err(Error::XOutOfRange(pixel.x, self.info.width));
        }
        if pixel.y >= self.info.height.unwrap() {
            return Err(Error::YOutOfRange(pixel.y, self.info.height.unwrap()));
        }

        let offset = ((pixel.x + pixel.y * self.info.width) * 3) as usize;
        if let Some(data) = self.image.as_mut() {
            data[offset..offset + 3].copy_from_slice(&pixel.color);
        };
        Ok(())
    }
}

impl From<ScreenInfo> for Screen {
    fn from(value: ScreenInfo) -> Self {
        let image_data = if value.mode == Mode::Graphical {
            let size = (value.width
                * value
                    .height
                    .expect("BUG: Graphical screen doesn't contain a height")
                * 3) as usize;
            Some(vec![0u8; size])
        } else {
            None
        };
        Self {
            info: value,
            image: image_data,
            device: None,
        }
    }
}

impl Default for Screen {
    fn default() -> Self {
        let info = ScreenInfo {
            width: 32,
            height: Some(32),
            color: Color::Rgb32,
            mode: Mode::Graphical,
            id: Default::default(),
        };
        Self::from(info)
    }
}

#[cfg(test)]
mod test {
    use crate::screen::*;

    #[test]
    fn set_pixel_on_graphic() {
        let mut screen: Screen = ScreenInfo {
            width: 32,
            height: Some(32),
            color: Color::Rgb32,
            mode: Mode::Graphical,
            id: Default::default(),
        }
        .into();
        let pixel = Pixel {
            x: 0,
            y: 0,
            color: [1, 2, 3],
        };
        assert!(screen.set_pixel(&pixel).is_ok());
    }

    #[test]
    fn set_pixel_on_non_graphic() {
        let mut seg_screen: Screen = ScreenInfo {
            width: 32,
            height: Some(32),
            color: Color::Rgb32,
            mode: Mode::Char,
            id: Default::default(),
        }
        .into();
        let pixel = Pixel {
            x: 0,
            y: 0,
            color: [1, 2, 3],
        };
        assert_eq!(
            canvas::Error::NotGraphical,
            seg_screen.set_pixel(&pixel).unwrap_err()
        );

        let mut char_screen: Screen = ScreenInfo {
            width: 32,
            height: Some(32),
            color: Color::Rgb32,
            mode: Mode::SevenSeg,
            id: Default::default(),
        }
        .into();
        assert_eq!(
            canvas::Error::NotGraphical,
            char_screen.set_pixel(&pixel).unwrap_err()
        );
    }

    #[test]
    fn image_data() {
        let screen: Screen = ScreenInfo {
            width: 1,
            height: Some(1),
            color: Color::Rgb32,
            mode: Mode::Graphical,
            id: Default::default(),
        }
        .into();
        assert_eq!(vec![0; 3], screen.image_data().unwrap());
    }

    #[test]
    fn image_data_non_graphic() {
        let screen: Screen = ScreenInfo {
            width: 1,
            height: Some(1),
            color: Color::Rgb32,
            mode: Mode::Char,
            id: Default::default(),
        }
        .into();

        assert_eq!(
            canvas::Error::NotGraphical,
            screen.image_data().unwrap_err()
        );
    }
}
