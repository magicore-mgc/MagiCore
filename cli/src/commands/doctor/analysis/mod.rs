//! Diagnostic issue construction and health classification.
//! Tạo diagnostic issue và phân loại sức khỏe.

mod analysis;
mod health_status;
mod store_issues;
mod toolchain_issues;

pub(super) use analysis::build;

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
