use poem::{listener::TcpListener, EndpointExt, Route, Server};
use poem_openapi::OpenApiService;
use tokio::{net::UdpSocket, select};
use tracing::{error, info};
use tracing_subscriber::prelude::*;

use hosts::Hosts;
use udp_server::UdpServer;
use web_log::{BroadcastLayer, LogTail};

mod api;
mod hosts;
mod udp_server;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let (broadcast, tracing_rx) = BroadcastLayer::new();
    let log_tail = LogTail::new(tracing_rx.clone());

    let registry = tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer().compact())
        .with(broadcast);
    match tracing_journald::layer() {
        Ok(layer) => {
            registry.with(layer).init();
        }
        Err(e) => {
            registry.init();
            error!("couldn't connect to journald: {}", e);
        }
    }

    let api = OpenApiService::new(
        api::Api,
        env!("CARGO_PKG_DESCRIPTION"),
        env!("CARGO_PKG_VERSION"),
    )
    .server("http://45.151.30.139:8080/api");
    let help = api.swagger_ui();
    let hosts = Hosts::default();
    let yaml_spec = api.spec_endpoint_yaml(); // Usefull for LLMs

    let app = Route::new()
        .nest("/api", api)
        .nest("/help", help)
        .nest("/spec", yaml_spec)
        .data(hosts.clone())
        .data(tracing_rx)
        .data(log_tail);

    let http = "0.0.0.0:8080";
    let udp = "0.0.0.0:4200";
    info!("Listening on: HTTP: {http}, UDP: {udp}...");

    let udp_server = UdpServer {
        hosts,
        socket: UdpSocket::bind(udp).await?,
    };

    select! {
        _ = Server::new(TcpListener::bind(http)).run(app) => {},
        _ = udp_server.run() => {}
    }

    Ok(())
}
