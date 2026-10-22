//! Workspace layout options returned by configuration parsing.
//! Tùy chọn layout workspace được trả về từ bước phân tích cấu hình.

/// Workspace layout directories; defaults remain apps/ and packages/.
/// Thư mục layout workspace; mặc định vẫn là apps/ và packages/.
#[derive(Debug, Clone)]
pub struct DiscoverOptions {
    pub apps_dir: String,
    pub packages_dir: String,
}
