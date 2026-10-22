use std::net::{Ipv4Addr, SocketAddrV4};

use clap::Parser;
use poem::{listener::TcpListener, EndpointExt, Route, Server};
use poem_openapi::OpenApiService;
use tokio::{net::UdpSocket, select};
#[cfg(target_os = "linux")]
use tracing::error;
use tracing::info;
use tracing_subscriber::prelude::*;

use hosts::Hosts;
use udp_server::UdpServer;
use web_log::{BroadcastLayer, LogTail};

mod api;
mod hosts;
mod udp_server;

#[derive(Debug, Parser)]
#[command(version, about)]
struct Args {
    /// TCP port for the HTTP API (listens on all IPv4 interfaces).
    #[arg(long, default_value_t = 8080, value_parser = clap::value_parser!(u16).range(1..))]
    http_port: u16,

    /// UDP port for rendezvous (listens on all IPv4 interfaces).
    #[arg(long, default_value_t = 4200, value_parser = clap::value_parser!(u16).range(1..))]
    udp_port: u16,
}

fn api_service() -> OpenApiService<api::Api, ()> {
    OpenApiService::new(
        api::Api,
        env!("CARGO_PKG_DESCRIPTION"),
        env!("CARGO_PKG_VERSION"),
    )
    .server("/api")
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let (broadcast, tracing_rx) = BroadcastLayer::new();
    let log_tail = LogTail::new(tracing_rx.clone());

    let registry = tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer().compact())
        .with(broadcast);
    #[cfg(target_os = "linux")]
    match tracing_journald::layer() {
        Ok(layer) => {
            registry.with(layer).init();
        }
        Err(e) => {
            registry.init();
            error!("couldn't connect to journald: {}", e);
        }
    }

    #[cfg(not(target_os = "linux"))]
    registry.init();

    let api = api_service();
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

    let http = SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, args.http_port);
    let udp = SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, args.udp_port);
    info!("Listening on: HTTP: {http}, UDP: {udp}...");

    let udp_server = UdpServer {
        hosts,
        socket: UdpSocket::bind(udp).await?,
    };

    select! {
        result = Server::new(TcpListener::bind(http)).run(app) => result?,
        result = udp_server.run() => result?,
    }

    Ok(())
}
