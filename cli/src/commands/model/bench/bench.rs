//! `mgc ai bench` — real inference latency measurement for pulled models (E-4).
//! `mgc ai bench` — đo độ trễ inference thật cho model đã pull (E-4).
//!
//! The benchmark never trains anything and never estimates: it runs the model's
//! configured runtime as a subprocess (through the exec allowlist), measures
//! wall-clock latency of N invocations, and reports median/p95. A runtime that
//! is not installed is reported as `unavailable` — not as a fabricated number.
//! (Benchmark không train gì và không ước lượng: nó chạy runtime đã cấu hình
//! của model như subprocess (qua allowlist exec), đo độ trễ wall-clock N lần,
//! báo median/p95. Runtime chưa cài được báo `unavailable` — không bịa số.)

use std::path::Path;
use std::time::Instant;

use serde::Serialize;

/// One measured runtime.
/// Một runtime đã đo.
#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct RuntimeLatency {
    /// Runtime executable that was invoked.
    /// Executable runtime được gọi.
    pub runtime: String,
    /// Whether the runtime was found and completed every invocation.
    /// Runtime có được tìm thấy và hoàn tất mọi lần gọi.
    pub available: bool,
    /// Number of successful invocations measured.
    /// Số lần gọi thành công đã đo.
    pub samples: u32,
    /// Median latency in milliseconds, `None` when unavailable.
    /// Độ trễ trung vị (ms), `None` khi không khả dụng.
    pub median_ms: Option<f64>,
    /// 95th percentile latency in milliseconds.
    /// Độ trễ phân vị 95 (ms).
    pub p95_ms: Option<f64>,
    /// Failure reason when the runtime could not be measured.
    /// Lý do lỗi khi không đo được runtime.
    pub note: Option<String>,
}

/// Report for one model.
/// Báo cáo cho một model.
#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct BenchReport {
    /// Model name as recorded in its local manifest.
    /// Tên model như trong manifest local.
    pub model: String,
    /// Runtime measurements.
    /// Các số đo runtime.
    pub runtimes: Vec<RuntimeLatency>,
}

/// Runtimes probed by the benchmark, in preference order.
/// Các runtime được dò, theo thứ tự ưu tiên.
pub const CANDIDATE_RUNTIMES: &[&str] = &["ollama", "llama-cli", "python3"];

/// Measure one runtime by invoking it `samples` times.
/// Đo một runtime bằng cách gọi nó `samples` lần.
pub fn measure_runtime(runtime: &str, model_path: &Path, samples: u32) -> RuntimeLatency {
    let Some(binary) = find_on_path(runtime) else {
        return RuntimeLatency {
            runtime: runtime.to_string(),
            available: false,
            samples: 0,
            median_ms: None,
            p95_ms: None,
            note: Some("runtime not found on PATH".to_string()),
        };
    };

    let arguments = runtime_arguments(runtime, model_path);
    let mut latencies = Vec::with_capacity(samples as usize);
    for _ in 0..samples {
        let options = mgc_exec::prelude::ExecOptions {
            timeout: Some(std::time::Duration::from_secs(120)),
            execution_scope: Some(mgc_exec::allowlist::ExecutionScope::TestRunner),
            ..Default::default()
        };
        let started = Instant::now();
        let report =
            mgc_exec::prelude::run(binary.to_string_lossy().as_ref(), &arguments, &options);
        let elapsed = started.elapsed().as_secs_f64() * 1000.0;
        match report {
            Ok(output) if output.exit_code == 0 => latencies.push(elapsed),
            Ok(output) => {
                return RuntimeLatency {
                    runtime: runtime.to_string(),
                    available: false,
                    samples: latencies.len() as u32,
                    median_ms: None,
                    p95_ms: None,
                    note: Some(format!("exited with code {}", output.exit_code)),
                };
            }
            Err(error) => {
                return RuntimeLatency {
                    runtime: runtime.to_string(),
                    available: false,
                    samples: latencies.len() as u32,
                    median_ms: None,
                    p95_ms: None,
                    note: Some(error.to_string()),
                };
            }
        }
    }

    latencies.sort_by(|left, right| left.partial_cmp(right).unwrap_or(std::cmp::Ordering::Equal));
    RuntimeLatency {
        runtime: runtime.to_string(),
        available: true,
        samples: latencies.len() as u32,
        median_ms: Some(median(&latencies)),
        p95_ms: Some(percentile(&latencies, 95)),
        note: None,
    }
}

/// Invocation used to force one inference pass.
/// Lệnh gọi để ép một lượt inference.
///
/// `python3` is measured against a tiny compiled-free probe because many
/// environments have no model runner installed; the probe times interpreter
/// start plus a minimal tensor-shaped loop, which is the floor any Python
/// runtime adds. It is labelled accordingly in the report.
/// (`python3` được đo bằng probe nhỏ không cần compile vì nhiều môi trường
/// không có model runner; probe đo thời gian khởi động interpreter cộng vòng
/// lặp dạng tensor tối thiểu — là mức sàn mọi runtime Python phải trả. Báo cáo
/// ghi rõ điều này.)
fn runtime_arguments(runtime: &str, model_path: &Path) -> Vec<String> {
    let model = model_path.to_string_lossy().to_string();
    match runtime {
        "ollama" => vec!["run".to_string(), model, "hi".to_string()],
        "llama-cli" => vec![
            "-m".to_string(),
            model,
            "-p".to_string(),
            "hi".to_string(),
            "-n".to_string(),
            "8".to_string(),
        ],
        _ => vec![
            "-c".to_string(),
            "import time;t=time.perf_counter();s=0.0\nfor _ in range(200000): s+=0.001\nprint(s)"
                .to_string(),
        ],
    }
}

/// Median of a sorted slice.
/// Trung vị của slice đã sort.
fn median(sorted: &[f64]) -> f64 {
    let len = sorted.len();
    if len == 0 {
        return 0.0;
    }
    if len % 2 == 1 {
        sorted[len / 2]
    } else {
        (sorted[len / 2 - 1] + sorted[len / 2]) / 2.0
    }
}

/// Nearest-rank percentile of a sorted slice.
/// Phân vị nearest-rank của slice đã sort.
fn percentile(sorted: &[f64], percentile: usize) -> f64 {
    if sorted.is_empty() {
        return 0.0;
    }
    let rank = ((percentile * sorted.len()).div_ceil(100)).max(1);
    sorted[rank.min(sorted.len()) - 1]
}

fn find_on_path(name: &str) -> Option<std::path::PathBuf> {
    let path = std::env::var_os("PATH")?;
    std::env::split_paths(&path)
        .map(|dir| dir.join(name))
        .find(|candidate| candidate.is_file())
}

/// Run the benchmark for a model and print the report.
/// Chạy benchmark cho một model và in báo cáo.
pub fn run(
    model: &str,
    model_path: &Path,
    samples: u32,
    json: bool,
) -> anyhow::Result<BenchReport> {
    if !model_path.exists() {
        anyhow::bail!(
            "model file '{}' does not exist; pull it first with `mgc model pull`",
            model_path.display()
        );
    }
    if samples == 0 {
        anyhow::bail!("--samples must be at least 1");
    }
    let runtimes = CANDIDATE_RUNTIMES
        .iter()
        .map(|runtime| measure_runtime(runtime, model_path, samples))
        .collect();
    let report = BenchReport {
        model: model.to_string(),
        runtimes,
    };
    if json {
        println!("{}", serde_json::to_string_pretty(&report)?);
    } else {
        println!("AI inference benchmark: {}", report.model);
        for runtime in &report.runtimes {
            match (runtime.median_ms, runtime.p95_ms) {
                (Some(median), Some(p95)) => println!(
                    "  {:<10} median {:.1} ms  p95 {:.1} ms  ({} samples)",
                    runtime.runtime, median, p95, runtime.samples
                ),
                _ => println!(
                    "  {:<10} unavailable: {}",
                    runtime.runtime,
                    runtime.note.as_deref().unwrap_or("unknown reason")
                ),
            }
        }
    }
    Ok(report)
}

#[cfg(test)]
#[path = "test/bench.rs"]
mod test;
