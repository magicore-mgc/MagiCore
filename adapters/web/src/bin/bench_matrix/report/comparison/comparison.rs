// Baseline comparison report
// So sánh median và giữ ngưỡng regression hiện có.
use crate::bench_matrix::*;

pub(in crate::bench_matrix) fn print_comparison(
    previous: &[ScenarioMeasurement],
    current: &[ScenarioMeasurement],
) -> bool {
    println!();
    println!("Baseline comparison");
    println!(
        "{:<24} {:>12} {:>12} {:>12} {:>12} {:>12}",
        "scenario", "prev-med", "curr-med", "delta", "prev-σ", "curr-σ"
    );
    let mut has_regression = false;

    for current_scenario in current {
        let Some(previous_scenario) = previous
            .iter()
            .find(|candidate| candidate.name == current_scenario.name)
        else {
            println!(
                "{:<24} {:>12} {:>12.2}ms {:>12} {:>12} {:>12}",
                current_scenario.name,
                "-",
                current_scenario.median_ms,
                "new",
                "-",
                format!("{:.2}ms", current_scenario.stddev_ms),
            );
            continue;
        };
        let delta_pct = if previous_scenario.median_ms > 0.0 {
            ((current_scenario.median_ms - previous_scenario.median_ms)
                / previous_scenario.median_ms)
                * 100.0
        } else {
            0.0
        };
        if delta_pct > 10.0 {
            has_regression = true;
        }
        let flag = if delta_pct > 10.0 { " WARN:" } else { "" };
        println!(
            "{:<24} {:>8.2}ms {:>8.2}ms {:>10.2}%{:>4} {:>8.2}ms {:>8.2}ms",
            current_scenario.name,
            previous_scenario.median_ms,
            current_scenario.median_ms,
            delta_pct,
            flag,
            previous_scenario.stddev_ms,
            current_scenario.stddev_ms,
        );
    }

    has_regression
}
