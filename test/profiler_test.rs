use std::{
    thread,
    time::{Duration, Instant},
};

use camera_merger::{HrtProfiler, Profile};

#[test]
fn empty_profiler() {
    let ptr = HrtProfiler::default();
    let stats = ptr.get_stats();
    assert!(stats.is_err())
}

#[test]
fn start_stop() {
    let mut strw = HrtProfiler::default();
    let expected_us: u128 = 1000;
    let sleep_val_ns: u32 = expected_us as u32 * 1000;

    for iter in 0..100 {
        let _ = strw.start();
        let start = Instant::now();
        thread::sleep(Duration::new(0, sleep_val_ns));
        let val = strw.stop_and_record().as_micros();
        let eel = start.elapsed().as_micros();
        assert!(
            eel <= val + 50,
            "value was {} and elapsed value was {} at iter {}",
            val,
            eel,
            iter
        );
    }
    let stats = strw.get_stats().expect("Empty  timestamp container");

    assert_ne!(stats.max_stop, Duration::from_micros(0));
    assert_ne!(stats.min_stop, Duration::from_micros(0));
    let max_time = Duration::from_millis(expected_us as u64 + 400);
    assert!(
        stats.avg_stops < max_time,
        "Value  of avg_stops was {:?} and expected was {:?} , {:?} ",
        stats.avg_stops,
        max_time,
        stats
    );
    assert!(
        stats.avg_stops.as_micros() > expected_us,
        "Value  of avg_stops was {:?} and expected was {} , {:?} ",
        stats.avg_stops,
        expected_us,
        stats
    );
}
