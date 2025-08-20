#![feature(portable_simd)]

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
    let mut reader = ImageReceiver::new();
    let mut operator = CameraOperator::new(cameras, reader.get_transmitter());

    // Main operation
    operator.initialze();
    operator.start();
    reader.start();

    thread::sleep(Duration::new(60, 0));
    operator.stop();
    Ok(())
}
