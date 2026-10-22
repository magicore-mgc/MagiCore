//! Tests for Lib language-name parsing.
//! Test parser tên ngôn ngữ Lib.

use super::super::language_from_str;
use crate::language::types::LibLanguage;

#[test]
fn all_current_names_and_aliases_are_stable() {
    assert_eq!(language_from_str("ts"), Some(LibLanguage::Ts));
    assert_eq!(language_from_str("typescript"), Some(LibLanguage::Ts));
    assert_eq!(language_from_str("rust"), Some(LibLanguage::Rust));
    assert_eq!(language_from_str("python"), Some(LibLanguage::Python));
    assert_eq!(language_from_str("go"), Some(LibLanguage::Go));
    assert_eq!(language_from_str("java"), Some(LibLanguage::Java));
    assert_eq!(language_from_str("kotlin"), Some(LibLanguage::Java));
    assert_eq!(language_from_str("dotnet"), Some(LibLanguage::DotNet));
    assert_eq!(language_from_str("csharp"), Some(LibLanguage::DotNet));
    assert_eq!(language_from_str("cs"), Some(LibLanguage::DotNet));
}

#[test]
fn unsupported_language_names_are_not_normalized() {
    assert_eq!(language_from_str("TypeScript"), None);
    assert_eq!(language_from_str("assembly"), None);
}
