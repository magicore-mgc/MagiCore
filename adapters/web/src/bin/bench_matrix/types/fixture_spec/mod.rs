// Benchmark owner module wiring — nối implementation của owner benchmark.
// Keep wiring separate from behavior — khai báo và export, không chứa xử lý.
mod fixture_spec;
pub(in crate::bench_matrix) use fixture_spec::FixtureSpec;
