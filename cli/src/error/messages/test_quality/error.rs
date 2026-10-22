//! Benchmark and test-runner availability diagnostics.
//! Lỗi benchmark và khả dụng test runner.

use anyhow::Error;
use anyhow::anyhow;

/// package.json missing scripts.test
pub fn package_json_missing_test_script() -> Error {
    anyhow!("package.json is missing scripts.test")
}

/// lib core has no test runner for this language yet
pub fn lib_no_test_runner() -> Error {
    anyhow!("lib core has no test runner for this language yet (07 §4 P1)")
}

/// Refuse benchmark mode that would mutate the lockfile outside the install writer lock.
/// Từ chối benchmark có thể sửa lockfile ngoài writer lock của installer.
pub fn bench_no_lock_unsafe() -> Error {
    anyhow!(
        "bench --no-lock is disabled because deleting mgc.lock outside the install writer lock can race with project mutations"
    )
}
