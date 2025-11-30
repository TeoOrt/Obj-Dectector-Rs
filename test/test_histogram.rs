use anyhow::Result;
use camera_merger::Profile;
use camera_merger::ProfileStats;
use camera_merger::plot_histogram;

use std::cell::RefCell;
use std::path::Path;
use std::rc::Rc;
use std::time::{Duration, Instant};

/// A simple fake profiler for testing
struct FakeProfiler {
    min: u128,
    max: u128,
}

impl Profile for FakeProfiler {
    fn get_stats(&self) -> Result<ProfileStats> {
        let fake_duration = Duration::new(0, 0);
        let min = Duration::from_micros(self.min as u64);
        let max = Duration::from_micros(self.max as u64);
        Ok(ProfileStats {
            avg_stops: fake_duration,
            min_stop: min,
            max_stop: max,
        })
    }
    fn start(&mut self) -> Instant {
        Instant::now()
    }
    fn stop_and_record(&mut self) -> Duration {
        Instant::now().elapsed()
    }
    fn stop(&mut self) -> Duration {
        Instant::now().elapsed()
    }
}

#[test]
fn test_histogram_compute() {
    let profiler = Rc::new(RefCell::new(FakeProfiler { min: 0, max: 100 }));
    // These values span 0 → 100 micros
    let data = vec![5, 15, 25, 35, 45, 55, 75, 85, 95, 65, 66, 68];
    let file_path = "Test.png";
    assert!(plot_histogram(profiler, data, file_path).is_ok());
    assert!(Path::new(file_path).exists());
}
