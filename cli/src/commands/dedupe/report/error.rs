//! Dedupe report errors — keeps JSON serialization errors as typed sources.
//! Lỗi report dedupe — giữ lỗi serialize JSON theo kiểu có source.
#[derive(Debug, thiserror::Error)]
pub(in crate::commands::dedupe) enum DedupeReportError {
    #[error(transparent)]
    JsonSerialization(serde_json::Error),
}
