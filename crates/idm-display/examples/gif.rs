use std::{fs::File, io::Read};

use idm_display::{IdmCommand, IdmService, IdmServiceEvent};
use image::GenericImageView;
const GIF_PATH: &str = "idm-display/gifs/christmas_tree.gif";

// Example of crate usage
#[tokio::main]
async fn main() {
    let service = IdmService::new().await.unwrap();
    let mut listener = service.subscribe().await;

    if let Ok(IdmServiceEvent::Connected(display)) = listener.recv().await {
        let image = image::open(GIF_PATH).unwrap();

        // The caller must guarantee gif's dimensions == display's size
        if image.dimensions() == display.size() {
            let mut file = File::open(GIF_PATH).unwrap();
            let mut bytes_buffer = Vec::new();

            file.read_to_end(&mut bytes_buffer).unwrap();

            display.send(IdmCommand::Gif { bytes_buffer });
        }

        display.disconnect().await.unwrap();
    }
}
