use idm_display::{IdmCommand, IdmService, IdmServiceEvent};
use image::Rgb;

// Example of crate usage
#[tokio::main]
async fn main() {
    let service = IdmService::new().await.unwrap();
    let mut listener = service.subscribe().await;

    if let Ok(IdmServiceEvent::Connected(display)) = listener.recv().await {
        // Draw the red-blue cross
        for index in 0..display.size().0 as u8 {
            let blue = Rgb::from([0x0, 0x0, 0xFF]);
            display.send(IdmCommand::Grafiti {
                color: blue,
                x: index,
                y: index,
            });

            let red = Rgb::from([0xFF, 0x0, 0x0]);
            display.send(IdmCommand::Grafiti {
                color: red,
                x: display.size().0 as u8 - index - 1,
                y: index,
            })
        }
        display.disconnect().await.unwrap();
    }
}
