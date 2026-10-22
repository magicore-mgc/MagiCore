// Benchmark baseline path
// Xây dựng đường dẫn baseline trong thư mục .benchmarks.
use crate::bench_matrix::*;

pub(in crate::bench_matrix) fn baseline_path(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join(".benchmarks")
        .join(format!("{name}.json"))
}
