use std::time::Duration;

use poem::web::Data;
use poem_openapi::{
    param::{Path, Query},
    payload::{Json, PlainText},
    ApiResponse, OpenApi,
};
use tracing::info;

use crate::hosts::{Host, Hosts};

#[derive(ApiResponse)]
enum GetHostResponce {
    #[oai(status = 200)]
    Ok(Json<Host>),
    /// Узел с таким именем не найден.
    #[oai(status = 404)]
    HostNotFound,
}

#[derive(ApiResponse)]
enum WaitHostResponce {
    /// IP-адрес хоста.
    #[oai(status = 200)]
    Ok(PlainText<String>),
    /// За указанный интервал хост не отправлял информацию о себе.
    #[oai(status = 404)]
    Timeout,
    /// Внутренний баг сервера
    #[oai(status = 500)]
    ISE(PlainText<String>),
}

pub struct Api;

#[OpenApi]
impl Api {
    /// Последняя информация о запрошенном хосте с момента старта сервиса.
    #[oai(path = "/host/:hostname", method = "get")]
    async fn get_host(&self, hostname: Path<String>, hosts: Data<&Hosts>) -> GetHostResponce {
        match hosts.get(hostname.0) {
            Some(host) => GetHostResponce::Ok(Json(host)),
            None => GetHostResponce::HostNotFound,
        }
    }

    /// Подождать указанный хост.
    #[oai(path = "/wait/:hostname", method = "get")]
    async fn wait_host(
        &self,
        hostname: Path<String>,
        hosts: Data<&Hosts>,
        /// По умолчанию: 10 seconds
        Query(timeout): Query<Option<String>>,
    ) -> WaitHostResponce {
        let timeout = Duration::from_secs(
            timeout
                .unwrap_or("10".to_string())
                .parse::<u64>()
                .ok()
                .unwrap_or(10),
        );

        let res = tokio::time::timeout(timeout, hosts.wait_for(hostname.0)).await;
        info!("wait: {:?}", res);
        match res {
            Ok(Ok(a)) => WaitHostResponce::Ok(PlainText(a.to_string())),
            Ok(Err(e)) => WaitHostResponce::ISE(PlainText(e.to_string())), // BUG: Sender dropped
            Err(_) => WaitHostResponce::Timeout,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::{net::SocketAddr, str::FromStr, sync::Arc, time::Duration};

    use poem::{http::StatusCode, test::TestClient, EndpointExt, Route};
    use poem_openapi::OpenApiService;
    use tokio::{task, time};

    use super::Api;
    use crate::hosts::Hosts;

    const VALID_IPV4_ADDR: &str = "192.168.0.1:1234";
    const VALID_HOSTNAME: &str = "test";

    #[tokio::test]
    async fn add_one_host() {
        let hosts = Hosts::default();
        hosts.update("test", SocketAddr::from_str(VALID_IPV4_ADDR).unwrap());

        let api = OpenApiService::new(Api, "", "").server("");
        let app = Route::new().nest("/", api).data(hosts);
        let cli = TestClient::new(app);

        let resp = cli.get(format!("/host/{VALID_HOSTNAME}")).send().await;
        resp.assert_status_is_ok();

        let resp = cli.get("/host/nonexistent").send().await;
        resp.assert_status(StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn wait_one_host() {
        let hosts = Hosts::default();

        let api = OpenApiService::new(Api, "", "").server("");
        let app = Route::new().nest("/", api).data(hosts.clone());
        let cli = TestClient::new(app);

        cli.get("/wait/nonexistent?timeout=1")
            .send()
            .await
            .assert_status(StatusCode::NOT_FOUND);

        task::spawn(async move {
            time::sleep(Duration::from_millis(100)).await;
            hosts.update(
                VALID_HOSTNAME,
                SocketAddr::from_str(VALID_IPV4_ADDR).unwrap(),
            );
        });
        let resp = cli
            .get(format!("/wait/{VALID_HOSTNAME}?timeout=1"))
            .send()
            .await;
        resp.assert_status_is_ok();
        resp.assert_text(VALID_IPV4_ADDR).await;
    }

    #[tokio::test]
    async fn parallel_wait_and_register() {
        let hosts = Hosts::default();
        let api = OpenApiService::new(Api, "", "").server("");
        let app = Route::new().nest("/", api).data(hosts.clone());
        let cli = Arc::new(TestClient::new(app));

        let peer1 = "peer1";
        let peer2 = "peer2";
        let addr1 = SocketAddr::from_str("192.168.0.1:1234").unwrap();
        let addr2 = SocketAddr::from_str("192.168.0.2:5678").unwrap();

        // Wait peer1, we are peer2
        let wait1 = task::spawn({
            let cli = cli.clone();
            async move {
                let resp = cli.get(format!("/wait/{peer1}?timeout=1")).send().await;
                resp.assert_status_is_ok();
                resp.assert_text(addr1.to_string()).await;
            }
        });
        // Our UDP outgoing port
        hosts.update(peer2, addr2);

        // Simulate registration delay
        time::sleep(Duration::from_millis(100)).await;
        let wait2 = task::spawn({
            let cli = cli.clone();
            async move {
                let resp = cli.get(format!("/wait/{peer2}?timeout=1")).send().await;
                resp.assert_status_is_ok();
                resp.assert_text(addr2.to_string()).await;
            }
        });
        hosts.update(peer1, addr1);

        wait1.await.unwrap();
        wait2.await.unwrap();
    }
}
