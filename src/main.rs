// #![feature(portable_simd)]

use anyhow::Result;
use camera_merger::{Camera, CameraBuilder, CameraConfig, CameraOperator, ImageReceiver, Start, Stop};
use std::{ thread, time::Duration};

fn main() -> Result<()> {
    let config = CameraConfig::from_file("CamConfig.toml")?;
    let cameras: Vec<Camera> = config
        .get_video_device_list()?
        .iter()
        .map(|devices| {
            CameraBuilder::default()
                .with_video_idx(devices.clone())
                .with_display_window(format!("Dev{devices}"))
                .expect("Error creating window")
                .with_quit_key('q')
                .build()
                .expect("Error")
        })
        .collect();
    // Interesting fact for macos this code doesn;t show the window
    let mut reader = ImageReceiver::new();
    reader.initialze();
    let mut operator = CameraOperator::new(cameras, reader.get_transmitter());

    operator.initialze();

    operator.start();
    reader.start();
    thread::sleep(Duration::new(60, 0));
    operator.stop();
    reader.stop();
    Ok(())
}
