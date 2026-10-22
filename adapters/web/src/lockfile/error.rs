//! `error.rs` — Typed failures owned by the Web lockfile boundary.
//! Phân loại lỗi lockfile tại owner Web; giữ nguyên hợp đồng MgResult công khai.
use mgc_types::MgError;

#[derive(Debug)]
pub(crate) enum LockfileError {
    Read(String),
    Encoding(String),
    Parse(String),
    Integrity(String),
    Configuration(String),
    Ownership(String),
    Mutation(String),
    Serialization(String),
    AtomicWrite(String),
    Graph(String),
    Merge(String),
}
impl std::fmt::Display for LockfileError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let message = match self {
            Self::Read(m)
            | Self::Encoding(m)
            | Self::Parse(m)
            | Self::Integrity(m)
            | Self::Configuration(m)
            | Self::Ownership(m)
            | Self::Mutation(m)
            | Self::Serialization(m)
            | Self::AtomicWrite(m)
            | Self::Graph(m)
            | Self::Merge(m) => m,
        };
        f.write_str(message)
    }
}
impl std::error::Error for LockfileError {}
impl From<LockfileError> for MgError {
    fn from(error: LockfileError) -> Self {
        Self::Other(error.to_string())
    }
}
