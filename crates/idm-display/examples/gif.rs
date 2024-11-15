use std::path::Path;

use idm_display::IdmDisplay;

const GIF_PATH: &str = "idm-display/gifs/christmas_tree.gif";
const DEVICE_NAME: &str = "IDM-FAACB1";

#[tokio::main]
async fn main() {
    let display = IdmDisplay::new(DEVICE_NAME).await.unwrap();
    display.gif(Path::new(GIF_PATH)).await.unwrap();
}
