//! Wire package-result mapping and its nearest tests.
//! Nối logic ánh xạ kết quả package với test gần nhất.

#[path = "results.rs"]
mod results;

pub(super) use results::map_and_filter_results;
