use std::{cell::RefCell, collections::BTreeMap, rc::Rc};

use crate::{HrtProfiler, StopWatch, plot_histogram};

#[derive(Default)]
pub struct ProfilerHub {
    profilers: BTreeMap<String, Rc<RefCell<dyn StopWatch>>>,
}

pub enum ProfilerType {
    RealTimeClock = 0,
    ClockMonotic,
}

impl ProfilerHub {
    pub fn register(
        &mut self,
        name: String,
        profiler_type: ProfilerType,
    ) -> Rc<RefCell<dyn StopWatch>> {
        let profiler = match profiler_type {
            ProfilerType::RealTimeClock => Rc::new(RefCell::new(HrtProfiler::default())),
            _ => todo!(),
        };
        self.profilers.entry(name).or_insert(profiler);

        Rc::clone(profiler)
    }
}

impl Drop for ProfilerHub {
    fn drop(&mut self) {
        self.profilers.iter().map(|(profile_name, ptr)| {
            plot_histogram(ptr, vec![1; 100], &profile_name + ".png");
        });
    }
}
