use idm_display::{IdmCommand, IdmService};
use image::Rgb;

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

    // Draw the red-blue cross
    for index in 0..display.size().0 as u8 {
        let blue = Rgb::from([0x0, 0x0, 0xFF]);
        display
            .send(IdmCommand::Grafiti {
                color: blue,
                x: index,
                y: index,
            })
            .await
            .unwrap();

        let red = Rgb::from([0xFF, 0x0, 0x0]);
        display
            .send(IdmCommand::Grafiti {
                color: red,
                x: display.size().0 as u8 - index - 1,
                y: index,
            })
            .await
            .unwrap();
    }
    display.disconnect().await.unwrap();
}
