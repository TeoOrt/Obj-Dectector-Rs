// #![feature(portable_simd)]

use anyhow::Result;
use camera_merger::labels::get_labels;
use camera_merger::{
    Camera, CameraBuilder, CameraConfig, CameraOperator, ChannelID, EventServer, ImageReceiver,
    Message, display_video,
};
use camera_merger::{HrtProfiler, Profile, plot_histogram};
use crossbeam::channel;
use crossbeam::channel::Receiver;
use rand::{Rng, rng};
use std::rc::Rc;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

static GLOBAL_CHANNEL_SIZE: usize = 1024; // 1kb

fn main() -> Result<()> {
    let mut strw = HrtProfiler::default();
    let mut rng = rand::rng();
    for _ in 0..1000 {
        let _ = strw.start();
        thread::sleep(Duration::new(0, rng.random_range(10000..100000)));
        strw.stop_and_record();
    }
    let stats = strw.get_stats();

    assert_ne!(stats.max_stop, 0);
    assert_ne!(stats.min_stop, 0);
    // assert!(stats.avg_stops < sleep_val_mls + 100);
    // assert!(stats.avg_stops > sleep_val_mls - 100);
    let ptr = Rc::new(strw);
    plot_histogram(ptr);

    if true {
        return Ok(());
    }
    let config = CameraConfig::from_file("CamConfig.toml")?;
    let event_bus_server = Arc::new(EventServer::default());

    let label_path = "Pylearn/data/coco.yaml";
    let labels = get_labels(label_path)?;

    let cameras: Vec<Camera> = config
        .get_video_device_list()?
        .iter()
        .map(|devices| {
            CameraBuilder::default()
                .with_video_idx(devices.clone())
                .with_display_window(format!("Camera1"))
                .expect("Error creating window")
                .with_event_server(event_bus_server.clone())
                .build()
                .expect("Error")
        })
        .collect();

    // gui sending
    let (gui_transmitter, gui_recv) = channel::unbounded();
    let ml_rx_list: Vec<Receiver<Message>> = cameras
        .iter()
        .enumerate()
        .map(|(idx, _)| {
            // Each camera will have it's own channel so this way we can
            // send which camera is sending their data
            let (tx, rx) = crossbeam::channel::bounded(GLOBAL_CHANNEL_SIZE);
            event_bus_server
                .register_msg(ChannelID::Camera(idx as u32), tx)
                .unwrap();
            rx
        })
        .collect();

    event_bus_server.register_msg(ChannelID::WindowDisplay, gui_transmitter)?;
    // event_bus_server.register_msg(MessageID::Interpreter, ml_processor)?;

    // Interesting fact for macos this code doesn;t show the window
    // let reader = ImageReceiver::new(event_bus_server.clone()).initialze();
    // let mut operator = CameraOperator::new(cameras, event_bus_server.clone());
    //
    let mut operator = CameraOperator::new(cameras, event_bus_server.clone());
    let mut reader = ImageReceiver::new(event_bus_server.clone(), ml_rx_list);
    event_bus_server.broadcast(Message::Start);
    reader.initialze();

    operator.initialze();
    display_video(gui_recv, event_bus_server.clone());
    Ok(())
}
