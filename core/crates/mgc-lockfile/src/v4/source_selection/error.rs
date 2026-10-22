//! Describe source-selection failures at their owning boundary.
//! Mô tả lỗi chọn source tại boundary sở hữu.

/// Source-selection errors — Lỗi chọn nguồn.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SourceSelectionError {
    /// Two different sources claim the package at the same specificity —
    /// AmbiguousSource. Hai nguồn khác nhau tuyên bố cùng độ đặc hiệu.
    AmbiguousSource { package: String },
    /// Several catch-all sources share the lowest priority —
    /// AmbiguousFallback. Nhiều nguồn fallback cùng priority thấp nhất.
    AmbiguousFallback { package: String },
    /// A public source would override a trusted specific claim —
    /// PublicOverridePrivate. Nguồn public đè claim specific tin cậy.
    PublicOverridePrivate { package: String, source: String },
}

impl std::fmt::Display for SourceSelectionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::AmbiguousSource { package } => write!(
                f,
                "ambiguous source for '{package}': multiple sources claim it at equal specificity"
            ),
            Self::AmbiguousFallback { package } => write!(
                f,
                "ambiguous fallback for '{package}': multiple catch-all sources share the lowest priority"
            ),
            Self::PublicOverridePrivate { package, source } => write!(
                f,
                "public source '{source}' would override a trusted claim for '{package}'"
            ),
        }
    }
}

impl std::error::Error for SourceSelectionError {}

#[cfg(test)]
#[path = "test/error.rs"]
mod tests;
