//! Verify shared error display, conversion and source-chain contracts.
//! Kiểm tra display, conversion và source chain của lỗi dùng chung.

#![cfg(test)]

use super::MgError;
use std::error::Error;

#[test]
fn display_templates_are_stable_for_named_errors() {
    let cases = [
        (
            MgError::InvalidPackageName("pkg".to_owned()),
            "invalid package name: pkg",
        ),
        (
            MgError::InvalidPackageSpec("spec".to_owned()),
            "invalid package spec: spec",
        ),
        (
            MgError::InvalidVersion("v1".to_owned()),
            "invalid version: v1",
        ),
        (
            MgError::InvalidVersionRange("range".to_owned()),
            "invalid version range: range",
        ),
        (
            MgError::DependencyConflict("graph".to_owned()),
            "dependency conflict: graph",
        ),
        (
            MgError::Network("offline".to_owned()),
            "network error: offline",
        ),
        (MgError::Store("write".to_owned()), "store error: write"),
        (
            MgError::Integrity("digest".to_owned()),
            "integrity verification failed: digest",
        ),
        (MgError::Other("custom".to_owned()), "custom"),
    ];

    for (error, expected) in cases {
        assert_eq!(error.to_string(), expected);
    }

    let unsupported = MgError::Unsupported {
        core: "web",
        capability: "install",
        guidance: "use a supported adapter".to_owned(),
    };
    assert_eq!(
        unsupported.to_string(),
        "web core does not support 'install' yet: use a supported adapter"
    );
}

#[test]
fn transparent_sources_remain_available() {
    let io_source = std::io::Error::new(std::io::ErrorKind::PermissionDenied, "io marker");
    let expected_io = io_source.to_string();
    let io_error = MgError::from(io_source);
    assert!(matches!(&io_error, MgError::Io(_)));
    assert_eq!(
        Error::source(&io_error).map(|source| source.to_string()),
        Some(expected_io)
    );

    let json_source = serde_json::from_str::<serde_json::Value>("{")
        .expect_err("the invalid JSON fixture must produce a parser error");
    let expected_json = json_source.to_string();
    let json_error = MgError::from(json_source);
    assert!(matches!(&json_error, MgError::Json(_)));
    assert_eq!(
        Error::source(&json_error).map(|source| source.to_string()),
        Some(expected_json)
    );

    let toml_source = toml::from_str::<toml::Value>("value = [")
        .expect_err("the incomplete TOML fixture must produce a parser error");
    let expected_toml = toml_source.to_string();
    let toml_error = MgError::from(toml_source);
    assert!(matches!(&toml_error, MgError::TomlDe(_)));
    assert_eq!(
        Error::source(&toml_error).map(|source| source.to_string()),
        Some(expected_toml)
    );

    let anyhow_error = MgError::from(anyhow::anyhow!("anyhow marker"));
    assert!(matches!(&anyhow_error, MgError::Anyhow(_)));
    assert_eq!(anyhow_error.to_string(), "anyhow marker");
}

#[test]
fn transparent_from_implementations_remain_available() {
    fn require_from<T: Into<MgError>>() {}

    require_from::<std::io::Error>();
    require_from::<serde_json::Error>();
    require_from::<toml::ser::Error>();
    require_from::<toml::de::Error>();
    require_from::<anyhow::Error>();
}
