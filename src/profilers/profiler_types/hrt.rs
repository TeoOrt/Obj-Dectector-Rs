use crate::Profile;
use crate::ProfileStats;
use anyhow::Result;
use std::time::{Duration, Instant};

#[derive(Default, Clone, Debug)]
pub struct HrtProfiler {
    start: Option<Instant>,
    stop: Option<Instant>,
    time_stamps: Vec<Duration>, // have to changee this to a circular_buffer
}
impl HrtProfiler {
    pub fn limit_timestamp_recording(&mut self, size: usize) {
        self.time_stamps.reserve_exact(size);
    }
    pub fn get_durations(&self) -> Vec<Duration> {
        return self.time_stamps.clone();
    }
}

impl Profile for HrtProfiler {
    fn start(&mut self) -> Instant {
        self.start = Some(Instant::now());
        return self.start.unwrap();
    }
    fn stop(&mut self) -> Duration {
        let stop = Instant::now();
        self.stop = Some(stop);
        return stop.elapsed();
    }
    fn stop_and_record(&mut self) -> Duration {
        let stop = Instant::now();
        self.stop = Some(stop);

        let elapsed = match self.start {
            Some(elapsed) => elapsed.elapsed(),
            _ => panic!("Warning No start has been set, you may have a race condition"),
        };
        self.time_stamps.push(elapsed);
        return elapsed;
    }

    fn get_stats(&self) -> Result<ProfileStats> {
        let avg: Duration = self.time_stamps.iter().sum();
        if self.time_stamps.len() < 1 {
            return Err(anyhow::anyhow!("Empty profiler will not provide any stats"));
        }
        let avg_stops = avg.as_micros() / self.time_stamps.len() as u128;
        let dur = Duration::from_micros(avg_stops as u64);

        let max = self.time_stamps.iter().max().unwrap();
        let min = self.time_stamps.iter().min().unwrap();
        let prof_stats = ProfileStats {
            avg_stops: dur,
            min_stop: min.clone(),
            max_stop: max.clone(),
        };
        Ok(prof_stats)
    }
}
