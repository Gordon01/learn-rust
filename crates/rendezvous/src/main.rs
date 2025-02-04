use hosts::Hosts;
use poem::{listener::TcpListener, EndpointExt, Route, Server};
use poem_openapi::OpenApiService;
use tokio::{net::UdpSocket, select};
use tracing::info;
use udp_server::UdpServer;

mod api;
mod hosts;
mod udp_server;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "poem=debug");
    }
    tracing_subscriber::fmt::init();

    let api =
        OpenApiService::new(api::Api, "Рандеву сервер", "1.0").server("http://localhost:8080/api");
    let help = api.swagger_ui();
    let hosts = Hosts::default();
    let yaml_spec = api.spec_endpoint_yaml(); // Usefull for LLMs

    let app = Route::new()
        .nest("/api", api)
        .nest("/help", help)
        .nest("/spec", yaml_spec)
        .data(hosts.clone());

    let http = "0.0.0.0:8080";
    let udp = "0.0.0.0:4200";
    info!("Listening on: {http}...");

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
