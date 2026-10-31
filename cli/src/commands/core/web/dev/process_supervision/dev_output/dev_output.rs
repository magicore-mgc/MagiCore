//! Renders the dev log lines that announce each half.
//! Kết xuất các dòng log dev thông báo từng nửa.
//!
//! Both halves of a split project run at once, so the parent names both
//! endpoints up front: the user knows every URL even when a child prints nothing
//! of its own, and a compile-only backend (`cargo run`, `go run`) is explained
//! instead of looking like a hang.
//! Hai nửa của project tách rời chạy đồng thời, nên process cha nêu cả hai
//! endpoint ngay: người dùng biết mọi URL kể cả khi process con không tự in gì,
//! và backend chỉ compile (`cargo run`, `go run`) được giải thích thay vì trông
//! như treo.

/// Readiness line the parent prints once a target is launched.
/// Dòng ready process cha in khi một target đã được launch.
pub(in crate::commands::core::web::implementation) fn readiness_line(
    role: &str,
    host: &str,
    port: u16,
) -> String {
    format!("[{role}] ready on http://{host}:{port}")
}

/// Line explaining a backend whose launch buffers its own output.
/// Dòng giải thích backend mà lệnh launch tự đệm output.
///
/// The text names the condition instead of promising speed: dev only guarantees
/// the line is printed, never how long the build takes.
/// Câu chữ nêu điều kiện thay vì hứa tốc độ: dev chỉ bảo đảm dòng này được in,
/// không hứa build mất bao lâu.
pub(in crate::commands::core::web::implementation) fn backend_starting_line(
    role: &str,
    program: &str,
) -> String {
    format!("[{role}] launching {program} (server output appears when it binds)")
}

/// Backend URL shown in the dev summary.
/// URL backend hiển thị trong tổng kết dev.
///
/// The frontend dev server proxies nothing in a split project, so the backend
/// keeps its own origin; the browser reaches it directly.
/// Dev server frontend không proxy gì trong project tách rời, nên backend giữ
/// origin riêng; trình duyệt gọi thẳng.
pub(in crate::commands::core::web::implementation) const BACKEND_ROLE: &str = "backend";

#[cfg(test)]
#[path = "test/dev_output.rs"]
mod tests;
