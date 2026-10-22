//! Canonical Lib language config-name parser.
//! Parser tên config ngôn ngữ Lib chuẩn.

use crate::language::types::LibLanguage;

/// Map the currently supported language names and aliases.
/// Ánh xạ tên ngôn ngữ và alias hiện được hỗ trợ.
pub(in crate::language::detection) fn language_from_str(name: &str) -> Option<LibLanguage> {
    match name {
        "ts" | "typescript" => Some(LibLanguage::Ts),
        "rust" => Some(LibLanguage::Rust),
        "python" => Some(LibLanguage::Python),
        "go" => Some(LibLanguage::Go),
        "java" | "kotlin" => Some(LibLanguage::Java),
        "dotnet" | "csharp" | "cs" => Some(LibLanguage::DotNet),
        _ => None,
    }
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
