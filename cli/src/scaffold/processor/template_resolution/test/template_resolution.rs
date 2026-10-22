//! Tests for root priority, directory selection and core aliases.
//! Kiểm thử ưu tiên root, chọn thư mục và alias core.

#![allow(clippy::unwrap_used)]

use crate::scaffold::processor::Scaffolder;
use crate::scaffold::template_root::TemplateRoot;
use crate::wizard::engine::ScaffoldConfig;
use std::path::Path;

use super::super::template_resolution::{
    resolve_web_template_dir_from_root, select_web_templates_root,
};

fn web_config(mode: &str, frameworks: &[&str]) -> ScaffoldConfig {
    ScaffoldConfig {
        core: "web".to_string(),
        sub_type: mode.to_string(),
        frameworks: frameworks
            .iter()
            .map(|value| (*value).to_string())
            .collect(),
        ..ScaffoldConfig::default()
    }
}

fn create_shared(path: &Path) {
    std::fs::create_dir_all(path.join("shared")).unwrap();
}

#[test]
fn source_root_with_shared_layer_wins_and_skips_cache_lookup() {
    let primary = tempfile::tempdir().unwrap();
    let cached = tempfile::tempdir().unwrap();
    create_shared(primary.path());
    let mut cache_lookup_called = false;

    let selected =
        select_web_templates_root(TemplateRoot::disk(primary.path().to_path_buf()), || {
            cache_lookup_called = true;
            TemplateRoot::disk(cached.path().to_path_buf())
        });

    assert_eq!(selected.path(), primary.path());
    assert!(!cache_lookup_called);
}

#[test]
fn registry_cache_with_shared_layer_is_used_when_primary_is_partial() {
    let primary = tempfile::tempdir().unwrap();
    let cached = tempfile::tempdir().unwrap();
    create_shared(cached.path());

    let selected =
        select_web_templates_root(TemplateRoot::disk(primary.path().to_path_buf()), || {
            TemplateRoot::disk(cached.path().to_path_buf())
        });

    assert_eq!(selected.path(), cached.path());
}

#[test]
fn incomplete_roots_preserve_the_primary_fallback() {
    let primary = tempfile::tempdir().unwrap();
    let cached = tempfile::tempdir().unwrap();

    let selected =
        select_web_templates_root(TemplateRoot::disk(primary.path().to_path_buf()), || {
            TemplateRoot::disk(cached.path().to_path_buf())
        });

    assert_eq!(selected.path(), primary.path());
}

#[test]
fn framework_modes_keep_their_existing_directory_layouts() {
    let root_dir = tempfile::tempdir().unwrap();
    let root = TemplateRoot::disk(root_dir.path().to_path_buf());
    let cases = [
        ("frontend", vec!["react-vite"], "frontend/react-vite"),
        ("backend", vec!["go", "gin"], "backend/go/gin"),
        (
            "fullstack",
            vec!["react-axum"],
            "fullstack/split/react-axum",
        ),
        ("fullstack", vec!["nextjs"], "fullstack/all-in-one/nextjs"),
        ("monorepo", vec!["react-vite", "fastify"], "monorepo/base"),
    ];

    for (mode, frameworks, expected) in cases {
        std::fs::create_dir_all(root.path().join(expected)).unwrap();
        let config = web_config(mode, &frameworks);
        let selected = resolve_web_template_dir_from_root(&root, &config).unwrap();
        assert_eq!(
            selected.path().strip_prefix(root.path()).unwrap(),
            Path::new(expected),
            "{mode} {frameworks:?}"
        );
    }
}

#[test]
fn explicit_template_directory_is_returned_without_registry_lookup() {
    let override_dir = tempfile::tempdir().unwrap();
    let mut config = web_config("frontend", &["react-vite"]);
    config.template_dir = override_dir.path().to_path_buf();

    let selected = Scaffolder::resolve_web_template_dir(&config).unwrap();

    assert_eq!(selected.path(), override_dir.path());
}

#[test]
fn core_template_lookup_uses_the_preflight_namespace() {
    for (core, expected) in [
        ("web", "web"),
        ("ai", "ai"),
        ("app", "app"),
        ("lib", "lib"),
        ("clo", "cloud"),
        ("cloud", "cloud"),
        ("game", "game"),
        ("iot", "iot"),
        ("cicd", "cicd"),
    ] {
        let layer = Scaffolder::core_template_layer(core, "mgc-cache-fixture");
        assert!(
            layer
                .path()
                .ends_with(Path::new(expected).join("mgc-cache-fixture")),
            "{core}"
        );
    }
}
