#![allow(clippy::unwrap_used)]

use super::super::default_framework;

#[test]
fn every_existing_core_default_and_generic_fallback_are_preserved() {
    for (core, expected) in [
        ("web", "vanilla"),
        ("game", "bevy"),
        ("ai", "python-agent"),
        ("clo", "pulumi-aws"),
        ("cicd", "github-actions"),
        ("iot", "esp32-rust"),
        ("app", "flutter"),
        ("lib", "rust"),
        ("hardware", "generic"),
        ("unknown", "generic"),
    ] {
        assert_eq!(default_framework(core), expected, "default for {core}");
    }
}
