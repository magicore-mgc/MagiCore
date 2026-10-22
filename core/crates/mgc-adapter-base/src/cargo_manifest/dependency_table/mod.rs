//! Mutate supported root Cargo dependency tables.
//! Cập nhật các bảng Cargo root được hỗ trợ.

mod dependency_table;

pub(super) use self::dependency_table::sync_dependency_table;

#[cfg(test)]
#[path = "test/dependency_table.rs"]
mod tests;
