use futures_util::stream::BoxStream;
use poem::web::Data;
use poem::{listener::TcpListener, EndpointExt, Route, Server};
use poem_openapi::OpenApiService;
use poem_openapi::{
    param::{Path, Query},
    payload::{EventStream, PlainText},
    OpenApi,
};
use tokio_stream::{wrappers::BroadcastStream, StreamExt};
use tracing::info;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

use web_log::{BroadcastLayer, LogTail, TraceReceiver};

pub struct Api;

#[OpenApi]
impl Api {
    #[oai(path = "/hello/:name", method = "get")]
    async fn hello(&self, Path(name): Path<String>) -> PlainText<String> {
        info!(name = name, "обращение к эндпойнту hello");
        PlainText(format!("Привет, {name}"))
    }

    #[oai(path = "/log/events", method = "get")]
    async fn log(&self, tracer: Data<&TraceReceiver>) -> EventStream<BoxStream<'static, String>> {
        EventStream::new(Box::pin(
            BroadcastStream::new(tracer.resubscribe()).map(|e| e.unwrap()),
        ))
    }

    #[oai(path = "/log/tail", method = "get")]
    async fn tail(
        &self,
        log: Data<&LogTail>,
        /// Число последних строк (по умолчанию 20)
        Query(n): Query<Option<usize>>,
    ) -> PlainText<String> {
        PlainText(log.tail(n.unwrap_or(20)))
    }
}

#[tokio::main]
async fn main() {
    let (broadcast, tracing_rx) = BroadcastLayer::new();
    let log_tail = LogTail::new(tracing_rx.clone());
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer().compact())
        .with(broadcast)
        .init();

    let api = OpenApiService::new(
        Api,
        env!("CARGO_PKG_DESCRIPTION"),
        env!("CARGO_PKG_VERSION"),
    )
    .server("http://localhost:8081/api");

    let app = Route::new().nest("/", api).data(tracing_rx).data(log_tail);

    Server::new(TcpListener::bind("0.0.0.0:8081"))
        .run(app)
        .await
        .unwrap();
}
