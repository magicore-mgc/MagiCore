//! Manifest variant tests — test biến thể manifest.
//! Locks the exact Lib and AI variant key sets consumed by the matrix.
//! Khóa đúng tập key Lib và AI mà matrix tiêu thụ.

use std::collections::BTreeSet;

use super::super::{ai_manifest_variants, lib_manifest_variants};

fn keys(value: &serde_json::Value) -> BTreeSet<String> {
    value.as_object().unwrap().keys().cloned().collect()
}

#[test]
fn lib_manifest_variant_keys_are_stable() {
    assert_eq!(
        keys(&lib_manifest_variants()),
        BTreeSet::from([
            "dotnet/csproj".to_owned(),
            "dotnet/unknown".to_owned(),
            "go/go-mod".to_owned(),
            "go/unknown".to_owned(),
            "java/gradle".to_owned(),
            "java/maven-pom".to_owned(),
            "java/unknown".to_owned(),
            "python/pep621-native".to_owned(),
            "python/pyproject-unsupported".to_owned(),
            "rust/cargo-toml".to_owned(),
            "rust/unknown".to_owned(),
            "ts/package-json".to_owned(),
            "ts/unknown".to_owned(),
        ])
    );
}

#[test]
fn ai_manifest_variant_keys_are_stable() {
    assert_eq!(
        keys(&ai_manifest_variants()),
        BTreeSet::from([
            "python/foreign-lock".to_owned(),
            "python/mgc-pyproject".to_owned(),
        ])
    );
}
