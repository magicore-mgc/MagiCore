// Model option errors — lỗi tùy chọn model.
// Validate flags shared by model source routes — kiểm tra flag dùng chung giữa các nguồn model.

use std::fmt;

#[derive(Debug)]
pub(in crate::commands::model) enum ModelOptionsError {
    PullFlagsRequireHf,
}

impl fmt::Display for ModelOptionsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::PullFlagsRequireHf => f.write_str(
                "--revision, --sha256, and --max-bytes apply only to hf:// model sources",
            ),
        }
    }
}

impl std::error::Error for ModelOptionsError {}
