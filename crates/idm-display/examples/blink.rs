use idm_display::{IdmCommand, IdmService};
use std::time::{Duration, Instant};

// Example of crate usage
#[tokio::main]
async fn main() {
    let service = IdmService::new().await.unwrap();
    let mut listener = service.subscribe().await;
    let mut displays = Vec::new();

    if let Ok(display) = listener.recv().await {
        displays.push(display);
    }

    let display = displays.first().unwrap();
    let start = Instant::now();
    while start.elapsed() < Duration::from_secs(4) {
        display.send(IdmCommand::SwitchOff).await.unwrap();
        tokio::time::sleep(Duration::from_millis(500)).await;

        display.send(IdmCommand::SwitchOn).await.unwrap();
        tokio::time::sleep(Duration::from_millis(500)).await;
    }
    display.disconnect().await.unwrap();
}
