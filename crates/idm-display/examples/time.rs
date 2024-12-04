use chrono::Local;
use idm_display::{IdmCommand, IdmService};

// Example of crate usage
#[tokio::main]
async fn main() {
    let service = IdmService::new().await.unwrap();
    let mut listener = service.subscribe().await;

    if let Ok(display) = listener.recv().await {
        let time = Local::now().naive_local();
        display.send(IdmCommand::Time { time });
        display.disconnect().await.unwrap();
    }
}
