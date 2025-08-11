use std::time::{Duration, Instant};

#[derive(Debug,Clone,Copy)]
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

pub struct HrtProfiler {
    start: Instant,
    stop: Instant,
    time_stamps: Vec<Duration>,
}

impl HrtProfiler {
    pub fn new() -> Self {
        HrtProfiler {
            start: Instant::now(),
            stop: Instant::now(),
            time_stamps: Vec::new(),
        }
    }
}

impl Profile for HrtProfiler {
    fn start(&mut self) -> u128 {
        self.start = Instant::now();
        self.time_stamps.push(self.start.elapsed());
        return self.start.elapsed().as_micros();
    }
    fn stop(&mut self) -> u128 {
        self.stop = Instant::now();
        return self.stop.elapsed().as_micros();
    }
    fn stop_and_record(&mut self) -> u128 {
        self.stop = Instant::now();
        self.time_stamps.push(self.stop - self.start);
        return self.stop.elapsed().as_micros();
    }

    fn get_stats(&self) -> ProfileStats {
        let avg : Duration= self.time_stamps.iter().sum();
        let max = self.time_stamps.iter().max().unwrap();
        let min = self.time_stamps.iter().min().unwrap();
        ProfileStats {
            avg_stops: avg.as_micros()/ self.time_stamps.len() as u128,
            min_stop: min.as_micros(),
            max_stop: max.as_micros(),
        }
    }
}
