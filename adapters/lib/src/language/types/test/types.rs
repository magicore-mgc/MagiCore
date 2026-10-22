//! Tests for Lib language ecosystem identifiers.
//! Test ecosystem id của ngôn ngữ Lib.

use super::super::LibLanguage;

#[test]
fn canonical_ecosystem_ids_are_stable() {
    assert_eq!(LibLanguage::Ts.ecosystem(), "ts");
    assert_eq!(LibLanguage::Rust.ecosystem(), "rust");
    assert_eq!(LibLanguage::Python.ecosystem(), "python");
    assert_eq!(LibLanguage::Go.ecosystem(), "go");
    assert_eq!(LibLanguage::Java.ecosystem(), "java");
    assert_eq!(LibLanguage::DotNet.ecosystem(), "dotnet");
}
