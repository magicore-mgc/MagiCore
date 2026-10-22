//! Benchmark and test-runner availability diagnostics.
//! Lỗi benchmark và khả dụng test runner.

mod error;

pub use error::{bench_no_lock_unsafe, lib_no_test_runner, package_json_missing_test_script};
