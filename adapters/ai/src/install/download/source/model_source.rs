//! Source variants and immutable-source builders.
//! Biến thể source và builder pin nguồn bất biến.

use std::path::PathBuf;

/// Model source variants.
/// Các biến thể nguồn model.
#[derive(Debug, Clone)]
pub enum ModelSource {
    /// HuggingFace Hub (repo_id, revision, filename)
    HuggingFace {
        repo_id: String,
        revision: Option<String>,
        filename: String,
        checksum: Option<String>,
    },
    /// Local file path
    Local(PathBuf),
    /// Remote URL
    Url {
        url: String,
        checksum: Option<String>,
    },
}

impl ModelSource {
    pub fn checksum(&self) -> Option<&str> {
        match self {
            ModelSource::HuggingFace { checksum, .. } => checksum.as_deref(),
            ModelSource::Url { checksum, .. } => checksum.as_deref(),
            ModelSource::Local(_) => None,
        }
    }

    pub fn huggingface(repo_id: &str, filename: &str) -> Self {
        ModelSource::HuggingFace {
            repo_id: repo_id.to_string(),
            revision: None,
            filename: filename.to_string(),
            checksum: None,
        }
    }

    /// Khai báo checksum để bật kiểm tra toàn vẹn sau download.
    // (Declare a checksum to enable post-download integrity verification.)
    /// Format: `"sha256:<hex>"`, `"blake3:<hex>"`, hoặc bare hex (mặc định hiểu là blake3).
    // (Format: prefixed algorithm, bare hex defaults to blake3.)
    pub fn with_checksum(mut self, checksum: impl Into<String>) -> Self {
        let value = Some(checksum.into());
        match &mut self {
            ModelSource::HuggingFace { checksum, .. } | ModelSource::Url { checksum, .. } => {
                *checksum = value;
            }
            ModelSource::Local(_) => {}
        }
        self
    }

    pub fn url(url: &str) -> Self {
        ModelSource::Url {
            url: url.to_string(),
            checksum: None,
        }
    }

    /// Pin a Hugging Face source to an immutable 40-hex commit revision.
    /// Pin nguồn Hugging Face vào revision commit bất biến 40 ký tự hex.
    pub fn with_revision(mut self, revision: impl Into<String>) -> Self {
        if let ModelSource::HuggingFace {
            revision: current, ..
        } = &mut self
        {
            *current = Some(revision.into());
        }
        self
    }
}
