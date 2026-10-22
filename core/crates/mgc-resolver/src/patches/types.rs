//! Shared private data passed between patch parsing and application.
//! Dữ liệu riêng dùng chung giữa bước phân tích và áp dụng patch.

#[derive(Debug)]
pub(super) struct Hunk {
    pub(super) old_start: usize,
    pub(super) old_len: usize,
    pub(super) lines: Vec<(char, String)>, // (' ', '-', '+') + content
}
