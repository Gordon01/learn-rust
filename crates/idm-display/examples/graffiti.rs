use idm_display::IdmDisplay;
use image::Rgb;

const DEVICE_NAME: &str = "IDM-FAACB1";

#[tokio::main]
async fn main() {
    let display = IdmDisplay::new(DEVICE_NAME).await.unwrap();

    // Draw the red-blue cross
    for index in 0..display.size().0 {
        let blue = Rgb::from([0x0, 0x0, 0xFF]);
        display.graffiti(blue, index, index).await.unwrap();

        let red = Rgb::from([0xFF, 0x0, 0x0]);
        display
            .graffiti(red, display.size().0 - index - 1, index)
            .await
            .unwrap();
    }
}
