use std::{
    sync::{
        atomic::{AtomicU64, Ordering},
        Arc,
    },
    time::Instant,
};

use poem::{
    handler,
    listener::TcpListener,
    web::{Data, Html},
    EndpointExt, Route, Server,
};
use poem_openapi::{
    param::Query,
    payload::{Json, PlainText},
    Object, OpenApi, OpenApiService,
};

struct Api;

/// Server health report
#[derive(Debug, Object, Clone)]
struct Health {
    /// Uptime in minutes
    uptime: u64,
    /// Total hits
    hits: u64,
    /// Hits per second
    hps: f64,
}

#[OpenApi]
impl Api {
    #[oai(path = "/hello", method = "get")]
    async fn index(
        &self,
        Query(name): Query<Option<String>>,
        Data(counter): Data<&Arc<AtomicU64>>,
    ) -> PlainText<String> {
        let cnt = counter.load(Ordering::Relaxed) + 1;
        counter.store(cnt, Ordering::Relaxed);

        match name {
            Some(name) => PlainText(format!("привет, {name}, вы пришли {cnt}!")),
            None => PlainText("привет, отправьте имя в параметре `name`!".to_string()),
        }
    }

    #[oai(path = "/health", method = "get")]
    async fn health(
        &self,
        Data(start): Data<&Arc<Instant>>,
        Data(counter): Data<&Arc<AtomicU64>>,
    ) -> Json<Health> {
        let uptime = start.elapsed().as_secs();
        let hits = counter.load(Ordering::Relaxed);

        Json(Health {
            uptime,
            hits,
            hps: hits as f64 / uptime as f64,
        })
    }
}

#[handler]
fn index() -> Html<&'static str> {
    Html(
        r#"<!doctype html>
<title>Привет, МФТИ!</title>
<h1>Это работает!</h1>
<a href="/help">Открыть Swagger UI</a><br>
<a href="/spec">Скачать спецификацию OpenApi в формате YAML</a><br>
<a href="/api">Корень REST API</a>
"#,
    )
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "poem=debug");
    }
    tracing_subscriber::fmt::init();

    let api = OpenApiService::new(Api, "Лекция 10", "1.0").server("http://localhost:7878/api");
    let help = api.swagger_ui();
    let yaml_spec = api.spec_endpoint_yaml();
    let counter = Arc::new(AtomicU64::new(0));
    let start = Arc::new(Instant::now());

    Server::new(TcpListener::bind("0.0.0.0:7878"))
        .run(
            Route::new()
                .at("/", index)
                .nest("/api", api)
                .nest("/help", help)
                .nest("/spec", yaml_spec)
                .data(counter)
                .data(start),
        )
        .await
}
