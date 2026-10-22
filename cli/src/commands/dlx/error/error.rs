//! Typed failures owned by the `mgc dlx` command boundary.
//! Lỗi typed thuộc ranh giới lệnh `mgc dlx`.

#[derive(Debug, thiserror::Error)]
pub enum DlxError {
    #[error("{0}")]
    CurrentDirectory(#[from] std::io::Error),
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
