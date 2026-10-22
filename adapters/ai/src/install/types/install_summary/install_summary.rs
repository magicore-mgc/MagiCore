//! Public summary returned after installing an AI model.
//! Bản tóm tắt công khai trả về sau khi cài model AI.

use super::super::super::download::ModelSource;
use std::path::PathBuf;

/// Result data for one model-install operation.
/// Dữ liệu kết quả của một lần cài model.
#[derive(Debug, Clone)]
pub struct InstallSummary {
    pub model_id: String,
    pub source: ModelSource,
    pub local_path: PathBuf,
    pub verified: bool,
    pub bytes_downloaded: u64,
    pub duration_ms: u64,
}
