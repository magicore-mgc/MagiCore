//! Lib language variants and stable ecosystem identifiers.
//! Variant ngôn ngữ Lib và ecosystem id ổn định.

/// Languages routed through the library adapter.
/// Ngôn ngữ được định tuyến qua adapter library.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LibLanguage {
    Ts,
    Rust,
    Python,
    Go,
    Java,
    DotNet,
}

impl LibLanguage {
    /// Return the canonical C0 firewall ecosystem identifier.
    /// Trả ecosystem id chuẩn mà firewall C0 dùng.
    pub fn ecosystem(&self) -> &'static str {
        match self {
            LibLanguage::Ts => "ts",
            LibLanguage::Rust => "rust",
            LibLanguage::Python => "python",
            LibLanguage::Go => "go",
            LibLanguage::Java => "java",
            LibLanguage::DotNet => "dotnet",
        }
    }
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
