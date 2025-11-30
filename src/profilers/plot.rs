use crate::{Profile, profilers::hist::ProfilerHistogram};
use plotters::prelude::*;
use tract_onnx::prelude::tract_itertools::Itertools;

use anyhow::Result;
use std::cell::RefCell;
use std::rc::Rc;

pub fn plot_histogram(
    profiler: Rc<RefCell<dyn Profile>>,
    data: Vec<u128>,
    filename: &str,
) -> Result<()> {
    let bin_size = 10;

    let mut histo = ProfilerHistogram::new(profiler.clone(), data);

    let conv_hist: Vec<(usize, i32)> = histo
        .bin(bin_size)
        .compute()?
        .into_iter()
        .map(|(k, v)| (k as usize, v as i32))
        .sorted_by_key(|x| x.0)
        .collect();
    let max_by_second_element = conv_hist.iter().max_by(|a, b| a.1.cmp(&b.1)).unwrap().1;
    let min_by_second_element = conv_hist.iter().min_by(|a, b| a.1.cmp(&b.1)).unwrap().1;

    let stats = profiler.borrow().get_stats()?;

    let drawing_area = BitMapBackend::new(filename, (300, 200)).into_drawing_area();
    drawing_area.fill(&WHITE).unwrap();

    let mut chart_builder = ChartBuilder::on(&drawing_area);

    chart_builder
        .margin(10)
        .set_left_and_bottom_label_area_size(20);

    let mut chart_context = chart_builder
        .build_cartesian_2d(
            (stats.min_stop.as_micros() as usize..stats.max_stop.as_micros() as usize)
                .into_segmented(),
            min_by_second_element..max_by_second_element,
        )
        .unwrap();

    chart_context.configure_mesh().draw().unwrap();
    chart_context
        .draw_series(
            Histogram::vertical(&chart_context)
                .style(BLUE.filled())
                .margin(10)
                .data(conv_hist),
        )
        .unwrap();
    Ok(())
}
