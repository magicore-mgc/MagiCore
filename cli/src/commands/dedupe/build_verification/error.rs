//! Build verification errors — preserves the root refusal and build context.
//! Lỗi xác minh build — giữ nguyên từ chối root và ngữ cảnh build.
use std::io;

#[derive(Debug, thiserror::Error)]
pub(in crate::commands::dedupe) enum DedupeBuildVerificationError {
    #[error(transparent)]
    CurrentDirectory(#[from] io::Error),
    #[error(
        "dedupe verification refused: process directory '{cwd}' resolves to a different project \
         than '{project_root}' — run dedupe from the project root"
    )]
    WrongProject { cwd: String, project_root: String },
    #[error("build verification failed after dedupe")]
    BuildFailed(#[source] anyhow::Error),
}
