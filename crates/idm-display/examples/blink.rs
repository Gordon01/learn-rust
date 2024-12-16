use idm_display::{IdmCommand, IdmService, IdmServiceEvent};
use std::time::{Duration, Instant};

// Example of crate usage
#[tokio::main]
async fn main() {
    let service = IdmService::new().await.unwrap();
    let mut listener = service.subscribe().await;

    if let Ok(IdmServiceEvent::Connected(display)) = listener.recv().await {
        let start = Instant::now();
        while start.elapsed() < Duration::from_secs(4) {
            display.send(IdmCommand::SwitchOff);
            tokio::time::sleep(Duration::from_millis(500)).await;

            display.send(IdmCommand::SwitchOn);
            tokio::time::sleep(Duration::from_millis(500)).await;
        }
        display.disconnect().await.unwrap();
    }
}
