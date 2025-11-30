use std::{
    fmt::Debug,
    time::{Duration, Instant},
};

use anyhow::Result;

#[derive(Clone, Copy)]
pub struct ProfileStats {
    pub avg_stops: Duration,
    pub min_stop: Duration,
    pub max_stop: Duration,
}

pub trait Profile {
    fn start(&mut self) -> Instant; //us
    fn stop(&mut self) -> Duration; // us
    fn stop_and_record(&mut self) -> Duration; //us
    fn get_stats(&self) -> Result<ProfileStats>;
}

impl Debug for ProfileStats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Avg Time : {}us\nMin Time: {}us\nMax Time: {}us",
            self.avg_stops.as_micros(),
            self.min_stop.as_micros(),
            self.max_stop.as_micros()
        )
    }
}
