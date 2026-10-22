// Benchmark module wiring — khai báo nhóm module benchmark.
// Keep this file limited to child modules and owner exports — chỉ nối module.
pub(super) mod arguments;
pub(super) mod run;
pub(super) use arguments::parse_arguments;
pub(crate) use run::run;
