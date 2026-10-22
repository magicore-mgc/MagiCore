//! Progress bars and manager — progress bar và manager.
//! Quiet behavior, styles, templates, and manager behavior preserve the old implementation.
//! Giữ nguyên quiet, style, template và hành vi manager của implementation cũ.

use crate::quiet::is_quiet;
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};

/// Create a styled progress bar — tạo progress bar có style.
/// Quiet mode returns a hidden bar — quiet trả về bar ẩn.
pub fn create_progress_bar(len: u64, msg: &str) -> ProgressBar {
    if is_quiet() {
        return ProgressBar::hidden();
    }
    let pb = ProgressBar::new(len);
    pb.set_style(
        ProgressStyle::default_bar()
            .template(concat!(
                "{spinner:.green} [{elapsed_precise}] ",
                "{bar:30.cyan/blue} {pos:>3}/{len:3} {msg:.white}"
            ))
            .expect("valid progress template")
            .progress_chars("━╾─"),
    );
    pb.set_message(msg.to_string());
    pb
}

/// Create a multi-progress group — tạo nhóm multi-progress.
/// The constructor remains visible even in quiet mode — constructor vẫn hoạt động khi quiet.
pub fn create_multi_progress() -> MultiProgress {
    MultiProgress::new()
}

/// Add a progress bar to a group — thêm progress bar vào nhóm.
/// Quiet mode returns a hidden bar — quiet trả về bar ẩn.
pub fn add_multi_bar(multi: &MultiProgress, len: u64, msg: &str) -> ProgressBar {
    if is_quiet() {
        return ProgressBar::hidden();
    }
    let pb = multi.add(ProgressBar::new(len));
    pb.set_style(
        ProgressStyle::default_bar()
            .template(concat!(
                "  {spinner:.green} {msg:25.cyan/blue} ",
                "{bar:20.cyan/blue} {bytes:>7}/{total_bytes:7} {eta}"
            ))
            .expect("valid progress template")
            .progress_chars("━╾─"),
    );
    pb.set_message(msg.to_string());
    pb
}

/// Create a spinner — tạo spinner.
/// Quiet mode returns a hidden spinner — quiet trả về spinner ẩn.
pub fn create_spinner(msg: &str) -> ProgressBar {
    if is_quiet() {
        return ProgressBar::hidden();
    }
    let spinner = ProgressBar::new_spinner();
    spinner.set_style(
        ProgressStyle::default_spinner()
            .template("{spinner:.green} {msg}")
            .expect("valid progress template")
            .tick_chars("⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏"),
    );
    spinner.set_message(msg.to_string());
    spinner
}

/// Progress manager for concurrent operations — manager cho thao tác đồng thời.
/// Manager keeps ownership of a single MultiProgress value — manager sở hữu một MultiProgress.
pub struct ProgressManager {
    multi: MultiProgress,
}

impl ProgressManager {
    /// Create an empty progress manager — tạo progress manager trống.
    pub fn new() -> Self {
        Self {
            multi: MultiProgress::new(),
        }
    }

    /// Attach a bar to this manager — gắn bar vào manager.
    pub fn add(&self, pb: ProgressBar) -> ProgressBar {
        self.multi.add(pb)
    }
}

impl Default for ProgressManager {
    fn default() -> Self {
        Self::new()
    }
}
