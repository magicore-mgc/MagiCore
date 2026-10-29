#![allow(clippy::unwrap_used)]

use super::*;

/// Statistics come from a sorted sample, never from a guess.
/// (Thống kê đến từ mẫu đã sort, không đoán.)
#[test]
fn median_and_percentile_follow_nearest_rank() {
    let sorted = [1.0, 2.0, 3.0, 4.0, 100.0];
    assert_eq!(median(&sorted), 3.0);
    assert_eq!(percentile(&sorted, 95), 100.0);
    assert_eq!(percentile(&sorted, 50), 3.0);
    let even = [10.0, 20.0, 30.0, 40.0];
    assert_eq!(median(&even), 25.0);
    assert_eq!(median(&[]), 0.0);
}

/// E-4: a runtime that is not installed is reported unavailable with a reason,
/// never as a fabricated latency.
/// (E-4: runtime chưa cài được báo unavailable kèm lý do, không bao giờ bịa số
/// đo.)
#[test]
fn missing_runtime_is_reported_as_unavailable() {
    let model = std::path::Path::new("/nonexistent/model.gguf");
    let result = measure_runtime("mgc-definitely-not-a-runtime", model, 1);
    assert!(!result.available);
    assert_eq!(result.samples, 0);
    assert!(result.median_ms.is_none());
    assert!(
        result
            .note
            .as_deref()
            .unwrap_or_default()
            .contains("not found"),
        "reason must name the cause: {:?}",
        result.note
    );
}

/// A measurable runtime produces real samples and both statistics.
/// (Runtime đo được sinh mẫu thật và cả hai thống kê.)
#[cfg(unix)]
#[test]
fn python_probe_produces_measured_latency() {
    let root = tempfile::tempdir().unwrap();
    let model = root.path().join("model.gguf");
    std::fs::write(&model, b"stub").unwrap();

    let result = measure_runtime("python3", &model, 3);
    if !result.available {
        // Some CI images ship no python3; an honest "unavailable" is still a
        // correct outcome, so this only asserts the shape.
        // (Một số image CI không có python3; "unavailable" vẫn là kết quả
        // đúng, nên test chỉ kiểm hình dạng.)
        assert!(result.note.is_some());
        return;
    }
    assert_eq!(result.samples, 3);
    let median = result.median_ms.expect("median");
    let p95 = result.p95_ms.expect("p95");
    assert!(median > 0.0, "a measured runtime must take real time");
    assert!(p95 >= median, "p95 cannot be below the median");
}

/// Benchmarking a model that was never pulled fails with guidance.
/// (Benchmark model chưa pull báo lỗi có hướng dẫn.)
#[test]
fn missing_model_path_is_refused() {
    let error = run(
        "demo",
        std::path::Path::new("/nonexistent/model.gguf"),
        1,
        true,
    )
    .expect_err("a missing model must fail");
    assert!(error.to_string().contains("does not exist"));
}
