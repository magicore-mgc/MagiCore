//! Storage budget measurement owner wiring.
//! File này chỉ nối module storage_budget và export API nội bộ.

mod storage_budget;

pub(super) use storage_budget::print_storage_budget;
#[cfg(test)]
pub(super) use storage_budget::{combined_path_size, measured_path_size};
