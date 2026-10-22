//! PEP 503 distribution-name normalization owner.
//! Owner chuẩn hóa tên distribution theo PEP 503.

mod distribution_name;

pub(super) use distribution_name::normalize_python_distribution_name;

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
