

use std::{thread, time::Duration};

use camera_merger::profilers::profile_structs::{HrtProfiler, Profile};

#[test]
fn start_stop() {
    let mut  strw = HrtProfiler::new();
    let sleep_val_ns : u32= 990000;
    let sleep_val_mls : u128= sleep_val_ns as u128 / 1000;

    let _ = strw.start();
    for _ in 0..1000{
        thread::sleep(Duration::new(0, sleep_val_ns));
        strw.stop_and_record();
    }
    let stats = strw.get_stats();

    assert_ne!(stats.max_stop, 0);
    assert_ne!(stats.min_stop , 0);
    assert!(stats.avg_stops < sleep_val_mls + 100);
    assert!(stats.avg_stops > sleep_val_mls - 100 );
}



