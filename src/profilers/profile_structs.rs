use std::{
    fmt::Debug,
    time::{Duration, Instant},
};

#[derive(Clone, Copy)]
pub struct ProfileStats {
    pub avg_stops: u128,
    pub min_stop: u128,
    pub max_stop: u128,
}

pub trait Profile {
    fn start(&mut self) -> u128; //mic
    fn stop(&mut self) -> u128; //milis
    fn stop_and_record(&mut self) -> u128; //milis
    fn get_stats(&self) -> ProfileStats;
}

#[derive(Default)]
pub struct HrtProfiler {
    start: Option<Instant>,
    stop: Option<Instant>,
    time_stamps: Vec<Duration>,
}

impl Profile for HrtProfiler {
    fn start(&mut self) -> u128 {
        self.start = Some(Instant::now());
        // self.time_stamps.push(self.start.elapsed());
        let start = self.start.unwrap().elapsed().as_micros();
        return start;
    }
    fn stop(&mut self) -> u128 {
        let stop = Instant::now();
        self.stop = Some(stop);
        return stop.elapsed().as_micros();
    }
    fn stop_and_record(&mut self) -> u128 {
        let stop = Instant::now();
        self.stop = Some(stop);
        if self.start.is_none() {
            eprintln!("Warning No start has been set, you may have a race condition");
            return 0;
        }
        let start = self.start.unwrap();
        self.time_stamps.push(stop - start);
        return stop.elapsed().as_micros();
    }

    fn get_stats(&self) -> ProfileStats {
        let avg: Duration = self.time_stamps.iter().sum();
        let max = self.time_stamps.iter().max().unwrap();
        let min = self.time_stamps.iter().min().unwrap();
        ProfileStats {
            avg_stops: avg.as_micros() / self.time_stamps.len() as u128,
            min_stop: min.as_micros(),
            max_stop: max.as_micros(),
        }
    }
}

impl Debug for ProfileStats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Avg Time : {}us\nMin Time: {}us\nMax Time: {}us",
            self.avg_stops, self.min_stop, self.max_stop
        )
    }
}

impl Drop for HrtProfiler {
    fn drop(&mut self) {
        println!("Profiler finished\n {:?}", self.get_stats())
    }
}
