// CAS source routing failures — lỗi định tuyến nguồn CAS.
// Reject unsupported source and unsafe size configuration — từ chối nguồn không hỗ trợ và giới hạn sai.

use std::fmt;

#[derive(Debug)]
pub(in crate::commands::model) enum CasPullError {
    MaximumBytesMustBePositive,
    UnsupportedSource(String),
}

impl fmt::Display for CasPullError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MaximumBytesMustBePositive => {
                f.write_str("--max-bytes must be greater than zero")
            }
            Self::UnsupportedSource(source) => write!(
                f,
                "unsupported model source '{source}' — use `hf://org/model/file` or `oci://registry/repo:tag`"
            ),
        }
    }
}

impl std::error::Error for CasPullError {}
