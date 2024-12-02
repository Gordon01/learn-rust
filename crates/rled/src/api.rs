use std::sync::LazyLock;

use futures_util::{stream::BoxStream, StreamExt};
use poem::web::Data;
use poem_openapi::{
    param::Path,
    payload::{EventStream, Json},
    ApiResponse, Object, OpenApi,
};
use tokio::sync::broadcast;
use tokio_stream::wrappers::BroadcastStream;

use crate::screen::{HwId, Pixel, ScreenInfo, Screens};

pub(crate) static PIXEL_CHANNEL: LazyLock<(
    broadcast::Sender<PixelEvent>,
    broadcast::Receiver<PixelEvent>,
)> = LazyLock::new(|| broadcast::channel(1024));

/// Событие об изменении пикселя экрана
#[derive(Object, Clone)]
pub(crate) struct PixelEvent {
    /// Идентификатор экрана
    // pub(crate) id: String,
    pub(crate) id: HwId,
    pub(crate) pixel: Pixel,
}

#[derive(ApiResponse)]
enum PutPixelResponse {
    /// Значение пикселя экрана успешно установлено
    #[oai(status = 200)]
    Ok,
    /// Экран не найден
    #[oai(status = 404)]
    ScreenNotFound,
    /// Координаты пикселя за пределами экрана
    #[oai(status = 400)]
    #[allow(dead_code)]
    OutOfRange,
    /// Графический режим на этом экране не поддерживается
    #[oai(status = 409)]
    #[allow(dead_code)]
    NotSupported,
}

#[derive(ApiResponse)]
enum GetImageResponce {
    #[oai(status = 200)]
    Ok(Json<Vec<u8>>),
    /// Экран не найден
    #[oai(status = 404)]
    ScreenNotFound,
    /// Графический режим на этом экране не поддерживается
    #[oai(status = 409)]
    #[allow(dead_code)]
    NotSupported,
}

pub struct Api;

#[OpenApi]
impl Api {
    /// Возвращает информацию обо всех обнаруженных экранах
    #[oai(path = "/screens", method = "get")]
    async fn index(&self, screens: Data<&Screens>) -> Json<Vec<ScreenInfo>> {
        Json(screens.list())
    }

    /// Возвращает данные пикселей указанного экрана
    #[oai(path = "/image/:addr", method = "get")]
    async fn image(
        &self,
        screens: Data<&Screens>,
        #[allow(unused_variables)] addr: Path<String>,
        id: HwId,
    ) -> GetImageResponce {
        match screens.image(&id) {
            Some(data) => GetImageResponce::Ok(Json(data)),
            None => GetImageResponce::ScreenNotFound,
        }
    }

    /// Устанавливает один пиксель указанного экрана
    #[oai(path = "/pixel/:addr", method = "put")]
    async fn pixel(
        &self,
        Json(pixel): Json<Pixel>,
        screens: Data<&Screens>,
        #[allow(unused_variables)] addr: Path<String>,
        id: HwId,
    ) -> PutPixelResponse {
        let res = screens.set_pixel(&id, &pixel);
        let _ = PIXEL_CHANNEL.0.send(PixelEvent {
            id, //id.to_string(),
            pixel,
        });
        match res {
            Ok(_) => PutPixelResponse::Ok,
            Err(_) => PutPixelResponse::ScreenNotFound,
        }
    }

    #[oai(path = "/events", method = "get")]
    async fn events(&self) -> EventStream<BoxStream<'static, PixelEvent>> {
        EventStream::new(Box::pin(
            BroadcastStream::new(PIXEL_CHANNEL.1.resubscribe()).map(|e| e.unwrap()),
        ))
    }
}
