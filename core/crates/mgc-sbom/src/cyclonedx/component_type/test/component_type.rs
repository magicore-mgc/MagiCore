#![allow(clippy::unwrap_used)]
//! Component type serde tests — Test serde component type

use super::super::super::*;

#[test]
fn serializes_all_variants_in_lowercase() {
    let variants = [
        (ComponentType::Application, "application"),
        (ComponentType::Framework, "framework"),
        (ComponentType::Library, "library"),
        (ComponentType::Container, "container"),
        (ComponentType::Device, "device"),
        (ComponentType::File, "file"),
    ];

    for (variant, expected) in variants {
        assert_eq!(
            serde_json::to_string(&variant).unwrap(),
            format!("\"{expected}\"")
        );
    }
}
