//! Core-specific manifest variants — biến thể manifest theo core.
//! The variants remain explicit projections of the shared dep gate.
//! Các biến thể vẫn là projection tường minh của dep gate dùng chung.

use serde_json::{Value, json};

use super::super::operation_projection::operations_for;

/// Return the existing Lib language/manifest qualification table.
/// Trả bảng điều kiện language/manifest hiện có của Lib.
pub(in crate::commands::capabilities::dependency_ownership) fn lib_manifest_variants() -> Value {
    json!({
        "ts/package-json": operations_for("lib", Some("ts"), Some("package-json")),
        "rust/cargo-toml": operations_for("lib", Some("rust"), Some("cargo-toml")),
        "python/pep621-native": operations_for("lib", Some("python"), Some("pep621-native")),
        "python/pyproject-unsupported": operations_for("lib", Some("python"), Some("pyproject-unsupported")),
        "go/go-mod": operations_for("lib", Some("go"), Some("go-mod")),
        "go/unknown": operations_for("lib", Some("go"), None),
        "rust/unknown": operations_for("lib", Some("rust"), None),
        "ts/unknown": operations_for("lib", Some("ts"), None),
        "java/maven-pom": operations_for("lib", Some("java"), Some("maven-pom")),
        "java/gradle": operations_for("lib", Some("java"), Some("gradle")),
        "dotnet/csproj": operations_for("lib", Some("dotnet"), Some("csproj")),
        "java/unknown": operations_for("lib", Some("java"), None),
        "dotnet/unknown": operations_for("lib", Some("dotnet"), None),
    })
}

/// Return the existing AI Python manifest and foreign-lock variants.
/// Trả các biến thể manifest Python và foreign-lock hiện có của AI.
pub(in crate::commands::capabilities::dependency_ownership) fn ai_manifest_variants() -> Value {
    json!({
        "python/mgc-pyproject": operations_for("ai", Some("python"), Some("mgc-pyproject")),
        "python/foreign-lock": operations_for("ai", Some("python"), Some("foreign-lock")),
    })
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
