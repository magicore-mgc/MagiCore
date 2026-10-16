//! Integration tests for scaffold system - early validation

#![allow(clippy::unwrap_used)]

use mgc::scaffold::spec::{CoreKind, parse_scaffold_spec};

#[test]
fn all_core_scaffold_rejects_corrupted_cache_without_fallback() {
    for (command, framework, layer, enabled) in [
        (
            "create-web",
            "vanilla",
            "web/shared/partials/base",
            cfg!(feature = "web"),
        ),
        (
            "create-ai",
            "python-agent",
            "ai/python-agent",
            cfg!(feature = "ai"),
        ),
        (
            "create-app",
            "flutter",
            "app/flutter",
            cfg!(feature = "app"),
        ),
        ("create-lib", "rust", "lib/rust", cfg!(feature = "lib")),
    ] {
        if !enabled {
            continue;
        }
        let root = tempfile::tempdir().unwrap();
        let home = root.path().join("home");
        let data = home.join("data");
        let cache = home.join("cache");
        let templates = home.join("templates");
        let store = home.join("store");
        for path in [&data, &cache, &templates, &store] {
            std::fs::create_dir_all(path).unwrap();
        }
        let cache_entry = templates.join(layer);
        std::fs::create_dir_all(cache_entry.parent().unwrap()).unwrap();
        let sentinel: &[u8] = b"not-a-template-directory";
        std::fs::write(&cache_entry, sentinel).unwrap();

        let output = std::process::Command::new(env!("CARGO_BIN_EXE_mgc"))
            .args([command, framework, "blocked-project"])
            .current_dir(root.path())
            .env("HOME", &home)
            .env("USERPROFILE", &home)
            .env("MGC_DATA_DIR", &data)
            .env("MGC_CACHE_DIR", &cache)
            .env("MGC_TEMPLATES_DIR", &templates)
            .env("MAGICORE_STORE_ROOT", &store)
            .output()
            .unwrap();
        let diagnostic = format!(
            "{}\n{}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        );
        assert!(
            !output.status.success(),
            "{command} ignored the cache error: {diagnostic}"
        );
        assert!(
            diagnostic.contains("refusing unsafe, linked, or non-directory template cache path"),
            "{command} failed for an unrelated reason: {diagnostic}"
        );
        assert!(
            !root.path().join("blocked-project").exists(),
            "{command} created a project after a cache failure"
        );
        assert_eq!(std::fs::read(&cache_entry).unwrap().as_slice(), sentinel);
    }
}

#[test]
fn test_web_nextjs_laster_typo_fails_early() {
    let result = parse_scaffold_spec(CoreKind::Web, "nextjs@laster");
    assert!(result.is_err(), "Should fail on typo");

    let err_msg = result.unwrap_err().to_string();
    assert!(
        err_msg.contains("laster"),
        "Error should mention the typo: {}",
        err_msg
    );
    assert!(
        err_msg.contains("latest"),
        "Error should suggest 'latest': {}",
        err_msg
    );
}

#[test]
fn test_web_nextjs_latest_succeeds() {
    let spec = parse_scaffold_spec(CoreKind::Web, "nextjs@latest").unwrap();
    assert_eq!(spec.name, "nextjs");
    assert_eq!(spec.core, CoreKind::Web);
}

#[test]
fn test_ai_fastapi_laster_typo_fails() {
    let result = parse_scaffold_spec(CoreKind::Ai, "fastapi@laster");
    assert!(result.is_err());
    let err_msg = result.unwrap_err().to_string();
    assert!(err_msg.contains("latest"));
}

#[test]
fn test_app_flutter_stabl_typo_suggests_stable() {
    let result = parse_scaffold_spec(CoreKind::App, "flutter@stabl");
    assert!(result.is_err());
    let err_msg = result.unwrap_err().to_string();
    assert!(err_msg.contains("stable"));
}

#[test]
fn test_game_bevy_betta_typo_suggests_beta() {
    let result = parse_scaffold_spec(CoreKind::Game, "bevy@betta");
    assert!(result.is_err());
    let err_msg = result.unwrap_err().to_string();
    assert!(err_msg.contains("beta"));
}

#[test]
fn test_multi_core_version_specs() {
    let web = parse_scaffold_spec(CoreKind::Web, "react@18.2.0").unwrap();
    assert_eq!(web.name, "react");

    let ai = parse_scaffold_spec(CoreKind::Ai, "pytorch@2.1.0").unwrap();
    assert_eq!(ai.name, "pytorch");

    let app = parse_scaffold_spec(CoreKind::App, "flutter@3.16.0").unwrap();
    assert_eq!(app.name, "flutter");

    let lib = parse_scaffold_spec(CoreKind::Lib, "rust@1.75.0").unwrap();
    assert_eq!(lib.name, "rust");
}

#[test]
fn test_empty_framework_name_fails() {
    let result = parse_scaffold_spec(CoreKind::Web, "@latest");
    assert!(result.is_err());
    let err_msg = result.unwrap_err().to_string();
    assert!(err_msg.contains("cannot be empty"));
}

#[test]
fn test_version_range_not_supported() {
    let result = parse_scaffold_spec(CoreKind::Web, "nextjs@^15.0.0");
    assert!(result.is_err());
    let err_msg = result.unwrap_err().to_string();
    assert!(err_msg.contains("ranges not supported"));
}

#[test]
fn test_normalize_uppercase_framework() {
    let spec = parse_scaffold_spec(CoreKind::Web, "NextJS@latest").unwrap();
    assert_eq!(spec.normalized_name, "nextjs");
}

#[test]
fn test_normalize_underscore_framework() {
    let spec = parse_scaffold_spec(CoreKind::App, "react_native@0.73.0").unwrap();
    assert_eq!(spec.normalized_name, "react-native");
}
