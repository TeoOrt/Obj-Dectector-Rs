use std::rc::Rc;

use crate::{Profile, labels};

pub struct ProfilerHistogram {
    profiler: Rc<dyn Profile>,
    labels: Vec<String>,
    bins: usize,
    data: Vec<u128>,
}

impl ProfilerHistogram {
    pub fn new(profiler: Rc<dyn Profile>, data: Vec<u128>) -> Self {
        Self {
            profiler,
            labels: Vec::new(),
            bins: 10,
            data,
        }
    }
    pub fn bin(&mut self, step_size: usize) -> &ProfilerHistogram {
        self.bins = step_size;
        self
    }
    pub fn compute(&self) -> Vec<usize> {
        let stats = self.profiler.get_stats();
        let bin_width: u128 = (stats.max_stop - stats.min_stop) / (self.bins as u128);
        let mut counts = vec![0; self.bins];
        // fill the histogram
        for value in self.data.clone() {
            let mut idx = ((value - stats.min_stop) / bin_width) as usize;
            if idx >= self.bins {
                idx = self.bins - 1;
            }
            counts[idx] += 1;
        }
        counts
    }
}
