use crate::task::{TaskResult, run_once};
use crate::metrics::compute_metrics;
use crate::report::print_summary;

mod cli;
mod task;
mod metrics;
mod report;

fn main() {
    let config = cli::parse_args();

    let mut results: Vec<TaskResult> = Vec::new();

    for _ in 0..config.iterations {
        let  t = run_once(&config.command);
        results.push(t);
    }

    if let Some(m) = compute_metrics(&results) {
        print_summary(&config, &m);
    } else {
        println!("No successful runs — cannot compute metrics.");
        std::process::exit(1);
    }
}
