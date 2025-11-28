use std::cell::RefCell;
use std::{collections::HashMap, rc::Rc, time::Duration};

use crate::Profile;
use anyhow::Result;

pub struct ProfilerHistogram {
    profiler: Rc<RefCell<dyn Profile>>,
    bins: usize,
    data: Vec<u128>,
}

impl ProfilerHistogram {
    pub fn new(profiler: Rc<RefCell<dyn Profile>>, data: Vec<u128>) -> Self {
        Self {
            profiler,
            bins: 10,
            data,
        }
    }
    pub fn bin(&mut self, step_size: usize) -> &ProfilerHistogram {
        self.bins = step_size;
        self
    }
    pub fn compute(&self) -> Result<HashMap<u128, usize>> {
        let stats = self.profiler.borrow().get_stats()?;
        let bin_width: u128 =
            (stats.max_stop.as_micros() - stats.min_stop.as_micros()) / (self.bins as u128);
        let mut counts = HashMap::new();
        // fill the histogram
        for value in self.data.clone() {
            let mut idx = ((value - stats.min_stop.as_micros()) / bin_width) as u128;
            if idx >= self.bins as u128 {
                idx = self.bins as u128 - 1;
            }
            let entry_val = (idx * bin_width) + stats.min_stop.as_micros();
            *counts.entry(entry_val).or_insert(0) += 1;
        }
        Ok(counts)
    }
}

#[cfg(test)]
mod tests {
    use crate::ProfileStats;

    use super::*;
    use std::cell::RefCell;
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
        // fn get_durations(&self) -> Vec<Duration> { vec![] }
    }

    #[test]
    fn test_histogram_compute() {
        let profiler = Rc::new(RefCell::new(FakeProfiler { min: 0, max: 100 }));

        // These values span 0 → 100 micros
        let data = vec![5, 15, 25, 35, 45, 55, 75, 85, 95, 65, 66, 68];

        let hist = ProfilerHistogram::new(profiler, data)
            .bin(10)
            .compute()
            .expect("histogram failed");

        assert_eq!(hist.len(), 10, "should have exactly 9 bins {:?}", hist);

        // Verify some known bins contain values
        assert_eq!(hist.get(&0), Some(&1)); // 5
        assert_eq!(hist.get(&10), Some(&1)); // 15
        assert_eq!(hist.get(&20), Some(&1)); // 25
        assert_eq!(hist.get(&90), Some(&1)); // 95

        dbg!(hist);
    }
}
