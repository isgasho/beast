use crate::parse::*;
use plotly::common::Title;
use plotly::layout::{Axis, BarMode, Layout};
use plotly::{Bar, Plot};
use std::time::Duration;

// pub struct BenchmarkPlot {
//     bm_list: Vec<Vec<String>>,
//     value_list: Vec<Vec<f32>>,
// }

// impl BenchmarkPlot {
//     pub fn new() -> BenchmarkPlot {
//         BenchmarkPlot {
//             bm_list: Vec::new(),
//             value_list: Vec::new(),
//         }
//     }

//     pub fn add_benchmark_infos(&self, context: serde_json::Value) {}

//     pub fn add_benchmark_result(&self, bm_result: serde_json::Value) {
//         //self.bmplot.add_trace();
//     }

//     pub fn plot(&self) {
//         let benchmarks1 = vec!["bm11", "bm12"];
//         let trace1 = Bar::new(benchmarks1, vec![20, 14]).name("prime500");

//         let benchmarks2 = vec!["bm21", "bm22"];
//         let trace2 = Bar::new(benchmarks2, vec![12, 18]).name("prim1000");

//         let layout = Layout::new().bar_mode(BarMode::Group);
//         let mut plot = Plot::new();
//         plot.add_trace(trace1);
//         plot.add_trace(trace2);
//         plot.set_layout(layout);
//         plot.show();
//     }
// }

pub fn plot_all(all_results: &Vec<BenchmarkResults>, plot_time_unit: &str) {
    // currently first benchmark is used as reference for cpu info and time unit
    let plot_title = format!(
        "CPU count: {}, MHz/CPU: {}",
        all_results[0].context.num_cpus, all_results[0].context.mhz_per_cpu
    )
    .to_string();

    let y_title = format!("CPU runtime [{}]", plot_time_unit).to_string();

    let layout = Layout::new()
        .title(Title::from(plot_title.as_str()))
        .bar_mode(BarMode::Group)
        .bar_group_gap(0.1)
        .y_axis(Axis::new().title(Title::from(y_title.as_str())));

    let mut plot = Plot::new();

    plot.set_layout(layout);

    for bm_results in all_results {
        let mut sub_bm_names = vec![];
        let mut sub_bm_cpu_times = vec![];
        let bm_results_name = bm_results.context.executable.as_path().file_name().unwrap();

        // collect sub benchmarks results for trace
        for sub_bm_res in &bm_results.benchmarks {
            let sub_bm_duration =
                from_benchmark_time(sub_bm_res.time_unit.as_ref(), sub_bm_res.cpu_time as u64);
            let sub_bm_converted_cpu_time = convert_time_to_unit(sub_bm_duration, plot_time_unit);

            sub_bm_names.push(sub_bm_res.name.clone());
            sub_bm_cpu_times.push(sub_bm_converted_cpu_time);
        }

        let res_trace =
            Bar::new(sub_bm_names, sub_bm_cpu_times).name(&bm_results_name.to_string_lossy());

        plot.add_trace(res_trace);
    }

    plot.show();
}

fn from_benchmark_time(from_time_unit: Option<&String>, time: u64) -> Duration {
    match from_time_unit {
        Some(from_time_unit) => match from_time_unit.as_ref() {
            "ns" => Duration::from_nanos(time),
            "us" => Duration::from_micros(time),
            "ms" => Duration::from_millis(time),
            _ => panic!("Unknown time unit provided!"),
        },
        None => {
            println!("No time unit was provided. Assuming ns!");
            Duration::from_nanos(time)
        }
    }
}

fn convert_time_to_unit(duration: Duration, time_unit: &str) -> f64 {
    let converted_time = match time_unit {
        "ns" => duration.as_nanos(),
        "us" => duration.as_micros(),
        "ms" => duration.as_millis(),
        _ => panic!("Unknown time unit provided!"),
    };
    converted_time as f64
}
