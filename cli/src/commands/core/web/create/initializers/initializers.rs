//! Resolve the official initializer for one scaffold request.
//! Resolve initializer chính thức cho một yêu cầu scaffold.
//!
//! This module is the single gate between "the user asked for framework X" and
//! "a program published by that framework runs". A framework with no record is
//! rejected: MagiCore does not keep a private starter to fall back on.
//! Module này là cổng duy nhất giữa "user xin framework X" và "một chương trình
//! do framework đó phát hành chạy". Framework không có record bị từ chối:
//! MagiCore không giữ starter riêng để dự phòng.

use super::catalog::{BACKEND, FRONTEND};
use super::types::{InitializerRecord, Mode};

/// Look up the initializer for `framework` in `mode`.
///
/// Returns `None` when the framework publishes no initializer MagiCore knows
/// about; the caller must report that honestly instead of scaffolding by hand.
/// Tìm initializer cho `framework` trong `mode`. Trả `None` khi framework không
/// phát hành initializer mà MagiCore biết; caller phải báo đúng sự thật thay vì
/// scaffold thủ công.
pub(in crate::commands::core::web::implementation) fn lookup(
    mode: Mode,
    framework: &str,
) -> Option<InitializerRecord> {
    let table = match mode {
        Mode::Frontend => FRONTEND,
        Mode::Backend => BACKEND,
    };
    table
        .iter()
        .find(|(id, _)| *id == framework)
        .map(|(_, record)| *record)
}

/// Every framework id that has an official initializer in `mode`.
/// Mọi id framework có initializer chính thức trong `mode`.
#[cfg(test)]
pub(crate) fn supported(mode: Mode) -> Vec<&'static str> {
    let table = match mode {
        Mode::Frontend => FRONTEND,
        Mode::Backend => BACKEND,
    };
    table.iter().map(|(id, _)| *id).collect()
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
