//! Canonical CI/CD provider name parser.
//! Parser tên provider CI/CD chuẩn.

/// Map a recognized canonical provider name (single table lives on the type).
/// Ánh xạ tên provider chuẩn (bảng duy nhất nằm trên type).
pub(crate) use crate::provider::provider_from_str;

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
