// Benchmark report table
// In bảng metric và scenario theo thứ tự hiện có.
use crate::bench_matrix::*;

pub(in crate::bench_matrix) fn print_report(scenarios: &[ScenarioMeasurement]) {
    println!("Benchmark matrix (install/materialization only)");
    println!(
        "{:<24} {:>4} {:>10} {:>10} {:>10} {:>10} {:>10} {:>12} {:>10} {:>10} {:>10} {:>10} {:>10}",
        "scenario",
        "runs",
        "median",
        "avg",
        "stddev",
        "min",
        "max",
        "cache-bytes",
        "files",
        "nm-bytes",
        "hardlinks",
        "cache-files",
        "cache-bytes"
    );
    for scenario in scenarios {
        println!(
            "{:<24} {:>4} {:>8.2}ms {:>8.2}ms {:>8.2}ms {:>8.2}ms {:>8.2}ms {:>12} {:>10} {:>10} {:>10} {:>10} {:>10}",
            scenario.name,
            scenario.runs,
            scenario.median_ms,
            scenario.avg_ms,
            scenario.stddev_ms,
            scenario.min_ms,
            scenario.max_ms,
            scenario.bytes_from_cache,
            scenario.node_modules_files,
            scenario.node_modules_bytes,
            scenario.hardlinked_files,
            scenario.cache_files,
            scenario.cache_bytes,
        );
    }
}
