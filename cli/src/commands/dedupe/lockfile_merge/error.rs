//! Lockfile merge errors — retains package-identity serialization as a typed source.
//! Lỗi merge lockfile — giữ lỗi serialize identity package theo kiểu có source.
#[derive(Debug, thiserror::Error)]
pub(in crate::commands::dedupe) enum LockfileMergeError {
    #[error("serialize package identity for dedupe")]
    SerializePackageIdentity(#[source] toml::ser::Error),
}
