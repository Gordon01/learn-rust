use idm_display::IdmService;
use poem::{handler, listener::TcpListener, web::Html, Endpoint, EndpointExt, Route, Server};
use poem_openapi::OpenApiService;

use screen::{Mode, ScreenInfo, Screens};
use tracing::{debug, info};

mod api;
mod screen;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "poem=debug");
    }
    tracing_subscriber::fmt::init();

    let api = OpenApiService::new(api::Api, "Сервер LED экранов", "1.0")
        .server("http://localhost:3000/api");
    let help = api.swagger_ui();
    let yaml_spec = api.spec_endpoint_yaml();

    // Добавляем один виртуальный экран
    let screens = Screens::new().await;
    screens.insert(ScreenInfo {
        id: "000000".try_into().unwrap(),
        width: 32,
        height: Some(32),
        color: screen::Color::Rgb32,
        mode: Mode::Graphical,
    });
    screens.insert(ScreenInfo {
        id: "111111".try_into().unwrap(),
        width: 16,
        height: Some(16),
        color: screen::Color::Rgb32,
        mode: Mode::Graphical,
    });

    let cloned_screens = screens.clone();
    tokio::spawn(async move {
        let bt_service = IdmService::new().await.unwrap();
        let mut listener = bt_service.subscribe().await;

        while let Ok(display) = listener.recv().await {
            let info = ScreenInfo {
                id: display.address().as_str()[6..12].try_into().unwrap(),
                width: display.size().0,
                height: Some(display.size().1),
                color: screen::Color::Rgb32,
                mode: Mode::Graphical,
            };
            cloned_screens.insert_real(info, display);
        }
    });

    let app = Route::new()
        .at("/", index)
        .at("/sim", simulator_index)
        .at("/sim/simulator.js", simulator_js)
        .nest("/api", api)
        .nest("/help", help)
        .nest("/spec", yaml_spec)
        .data(screens)
        .around(|ep, req| async move {
            // Логировать каждый запрос
            let uri = req.uri().clone();
            let resp = ep.get_response(req).await;
            debug!("{} {}", uri, resp.status());
            Ok(resp)
        });

    let addr = "0.0.0.0:3000";
    info!("Listening on: {addr}...");
    Server::new(TcpListener::bind(addr)).run(app).await?;

    Ok(())
}

#[handler]
fn index() -> Html<&'static str> {
    Html(include_str!("../html/index.html"))
}

#[handler]
fn simulator_index() -> Html<&'static str> {
    Html(include_str!("../html/simulator.html"))
}

#[handler]
fn simulator_js() -> Html<&'static str> {
    Html(include_str!("../html/simulator.js"))
}
