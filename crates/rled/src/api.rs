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

use crate::screen::{Error, HwId, Pixel, ScreenInfo, Screens};

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
            Ok(data) => GetImageResponce::Ok(Json(data)),
            Err(Error::NotFound) => GetImageResponce::ScreenNotFound,
            Err(_) => GetImageResponce::NotSupported,
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
            Err(Error::NotFound) => PutPixelResponse::ScreenNotFound,
            Err(Error::XOutOfRange(_, _) | Error::YOutOfRange(_, _)) => {
                PutPixelResponse::OutOfRange
            }
            Err(Error::NotGraphical) => PutPixelResponse::NotSupported,
            _ => PutPixelResponse::ScreenNotFound,
        }
    }

    #[oai(path = "/events", method = "get")]
    async fn events(&self) -> EventStream<BoxStream<'static, PixelEvent>> {
        EventStream::new(Box::pin(
            BroadcastStream::new(PIXEL_CHANNEL.1.resubscribe()).map(|e| e.unwrap()),
        ))
    }
}

#[cfg(test)]
mod tests {

    use poem::{http::StatusCode, test::TestClient, EndpointExt, Route};
    use poem_openapi::OpenApiService;

    use super::Api;
    use crate::screen::{Color, Mode, Pixel, ScreenInfo, Screens};

    #[tokio::test]
    async fn get_image() {
        let screens = Screens::new().await;
        screens.insert(ScreenInfo {
            id: "000000".try_into().unwrap(),
            width: 32,
            height: Some(32),
            color: Color::Rgb32,
            mode: Mode::Graphical,
        });
        screens.insert(ScreenInfo {
            id: "000001".try_into().unwrap(),
            width: 32,
            height: Some(32),
            color: Color::Rgb32,
            mode: Mode::Char,
        });

        let api = OpenApiService::new(Api, "", "").server("");
        let app = Route::new().nest("/", api).data(screens);
        let cli = TestClient::new(app);

        let resp = cli.get("/image/000000").send().await;
        resp.assert_status_is_ok();

        let resp = cli.get("/image/000001").send().await;
        resp.assert_status(StatusCode::CONFLICT);

        let resp = cli.get("/image/000002").send().await;
        resp.assert_status(StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn put_pixel() {
        let screens = Screens::new().await;
        screens.insert(ScreenInfo {
            id: "000000".try_into().unwrap(),
            width: 32,
            height: Some(32),
            color: Color::Rgb32,
            mode: Mode::Graphical,
        });
        screens.insert(ScreenInfo {
            id: "000001".try_into().unwrap(),
            width: 32,
            height: Some(32),
            color: Color::Rgb32,
            mode: Mode::Char,
        });

        let api = OpenApiService::new(Api, "", "").server("");
        let app = Route::new().nest("/", api).data(screens);
        let cli = TestClient::new(app);

        let pixel_data = Pixel {
            x: 10,
            y: 10,
            color: [0, 0, 0],
        };

        let resp = cli.put("/pixel/000000").body_json(&pixel_data).send().await;
        resp.assert_status_is_ok();

        let pixel_data = Pixel {
            x: 33,
            y: 10,
            color: [0, 0, 0],
        };
        let resp = cli.put("/pixel/000000").body_json(&pixel_data).send().await;
        resp.assert_status(StatusCode::BAD_REQUEST);

        let pixel_data = Pixel {
            x: 10,
            y: 10,
            color: [0, 0, 0],
        };
        let resp = cli.put("/pixel/000001").body_json(&pixel_data).send().await;
        resp.assert_status(StatusCode::CONFLICT);

        let pixel_data = Pixel {
            x: 10,
            y: 10,
            color: [0, 0, 0],
        };
        let resp = cli.put("/pixel/000002").body_json(&pixel_data).send().await;
        resp.assert_status(StatusCode::NOT_FOUND);

        let screens = Screens::new().await;
        screens.insert(ScreenInfo {
            width: 10,
            height: Some(10),
            color: Color::Rgb32,
            mode: Mode::Graphical,
            id: "000000".try_into().unwrap(),
        });
        let pixel_data = Pixel {
            x: 0,
            y: 0,
            color: [1, 2, 3],
        };
        screens
            .set_pixel(&"000000".try_into().unwrap(), &pixel_data)
            .unwrap();
        let image = screens.image(&"000000".try_into().unwrap()).unwrap();
        assert_eq!([1, 2, 3], image[0..3]);
    }
}
