use std::time::Instant;

pub struct ProfileStats {
    pub avg_stops: u128,
    pub min_stop: u128,
    pub max_stop: u128,
}

pub trait Profile {
    fn start(&mut self) -> u128; //mic
    fn stop(&mut self) -> u128; //milis
    fn stop_and_record(&mut self) -> u128; //milis
    fn get_stats(self) -> ProfileStats;
}

pub struct HrtProfiler {
    start: Instant,
    stop: Instant,
    time_stamps: Vec<Instant>,
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
        self.time_stamps.push(self.start);
        return self.start.elapsed().as_micros();
    }
    fn stop(&mut self) -> u128 {
        self.stop = Instant::now();
        return self.stop.elapsed().as_micros();
    }
    fn stop_and_record(&mut self) -> u128 {
        self.stop = Instant::now();
        self.time_stamps.push(self.stop);
        return self.stop.elapsed().as_micros();
    }

    fn get_stats(self) -> ProfileStats {
        let mut elapsed_times = Vec::new();
        for i in 0..self.time_stamps.len() - 2 {
            let timer = self.time_stamps[i + 1].duration_since(self.time_stamps[i]);
            elapsed_times.push(timer);
        }
        let max = elapsed_times.iter().max().unwrap();
        let min = elapsed_times.iter().min().unwrap();
        let avg: u128 = elapsed_times.iter().map(|d| d.as_micros()).sum();
        ProfileStats {
            avg_stops: avg.div_ceil(elapsed_times.len() as u128),
            min_stop: min.as_micros(),
            max_stop: max.as_micros(),
        }
    }
}
