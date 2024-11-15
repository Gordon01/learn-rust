use idm_display::IdmDisplay;
use std::time::{Duration, Instant};

const DEVICE_NAME: &str = "IDM-FAACB1";

#[tokio::main]
async fn main() {
    let display = IdmDisplay::new(DEVICE_NAME).await.unwrap();
    let start = Instant::now();

    while start.elapsed() < Duration::from_secs(4) {
        display.switch_off().await.unwrap();
        tokio::time::sleep(Duration::from_millis(500)).await;

        display.switch_on().await.unwrap();
        tokio::time::sleep(Duration::from_millis(500)).await;
    }
}
