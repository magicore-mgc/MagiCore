use std::path::{Path, PathBuf};

/// Validated destination path paired with exactly one transfer source.
/// Đường dẫn đích đã validate đi cùng đúng một source truyền.
pub(in crate::install::download::run_download) struct PreparedModelDownload<'a> {
    pub(in crate::install::download::run_download) relative_path: PathBuf,
    pub(in crate::install::download::run_download) transfer_source: PreparedTransferSource<'a>,
}

/// The source is either a borrowed local path or an owned remote URL.
/// Source là path local mượn hoặc URL remote sở hữu.
pub(in crate::install::download::run_download) enum PreparedTransferSource<'a> {
    Local(&'a Path),
    Remote(String),
}
