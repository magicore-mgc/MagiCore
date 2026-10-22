// Benchmark command arguments
// Phân tích tùy chọn dòng lệnh cho benchmark.
use crate::bench_matrix::*;

pub(in crate::bench_matrix) fn parse_arguments() -> anyhow::Result<BenchOptions> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let mut runs = 7usize;
    let mut save_baseline: Option<String> = None;
    let mut compare_baseline: Option<String> = None;
    let mut profile = BenchmarkProfile::Standard;
    let mut idx = 0usize;
    while idx < args.len() {
        match args[idx].as_str() {
            "--runs" => {
                idx += 1;
                let value = args
                    .get(idx)
                    .ok_or_else(|| anyhow::anyhow!("missing value for --runs"))?;
                runs = value.parse::<usize>()?;
            }
            "--save-baseline" => {
                idx += 1;
                let value = args
                    .get(idx)
                    .ok_or_else(|| anyhow::anyhow!("missing value for --save-baseline"))?;
                save_baseline = Some(value.clone());
            }
            "--compare-baseline" => {
                idx += 1;
                let value = args
                    .get(idx)
                    .ok_or_else(|| anyhow::anyhow!("missing value for --compare-baseline"))?;
                compare_baseline = Some(value.clone());
            }
            "--profile" => {
                idx += 1;
                let value = args
                    .get(idx)
                    .ok_or_else(|| anyhow::anyhow!("missing value for --profile"))?;
                profile = match value.as_str() {
                    "standard" => BenchmarkProfile::Standard,
                    "heavy" => BenchmarkProfile::Heavy,
                    other => return Err(anyhow::anyhow!("unknown profile: {other}")),
                };
            }
            other => return Err(anyhow::anyhow!("unknown argument: {other}")),
        }
        idx += 1;
    }
    Ok(BenchOptions {
        runs,
        save_baseline,
        compare_baseline,
        profile,
    })
}
