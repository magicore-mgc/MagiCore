//! Select the catalog for one online Web scaffold mode.
//! Chọn catalog cho một mode scaffold Web online.

use super::types::OnlineWebGenerator;

/// Resolve a supported `(mode, framework)` pair, preserving the old fallback.
/// Tìm cặp `(mode, framework)` được hỗ trợ và giữ nguyên nhánh fallback cũ.
pub(crate) fn online_web_generator(mode: &str, framework: &str) -> Option<OnlineWebGenerator> {
    match mode {
        "frontend" => super::frontend::frontend_web_generator(framework),
        "fullstack" if crate::scaffold::processor::is_all_in_one_fullstack(framework) => {
            super::frontend::frontend_web_generator(framework)
        }
        "backend" => super::backend::backend_web_generator(framework),
        _ => None,
    }
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
