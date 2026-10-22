//! Tests for Web layer order and monorepo fallback decisions.
//! Kiểm thử thứ tự Web layer và quyết định fallback monorepo.

#![allow(clippy::unwrap_used)]

use crate::scaffold::template_root::TemplateRoot;
use crate::wizard::engine::ScaffoldConfig;
use std::path::Path;

use super::super::implementation::{assemble_web_mode_layers, fullstack_uses_monorepo_fallback};

fn create_layer(root: &Path, rel: &str, contract: bool) -> TemplateRoot {
    let path = root.join(rel);
    std::fs::create_dir_all(&path).unwrap();
    if contract {
        std::fs::create_dir_all(path.join("sources")).unwrap();
        std::fs::write(path.join("template.toml"), "[files]\n").unwrap();
    }
    TemplateRoot::disk(path)
}

fn relative_layers(layers: &[TemplateRoot], root: &Path) -> Vec<String> {
    layers
        .iter()
        .map(|layer| {
            layer
                .path()
                .strip_prefix(root)
                .unwrap()
                .to_string_lossy()
                .replace('\\', "/")
        })
        .collect()
}

#[test]
fn react_frontend_keeps_shared_foundation_and_leaf_order() {
    let root_dir = tempfile::tempdir().unwrap();
    let root = TemplateRoot::disk(root_dir.path().to_path_buf());
    let leaf = create_layer(root.path(), "frontend/react-vite", true);
    let base = create_layer(root.path(), "shared/partials/base", false);

    let layers =
        assemble_web_mode_layers(&root, "frontend", "react-vite", leaf, vec![base]).unwrap();

    assert_eq!(
        relative_layers(&layers, root.path()),
        vec![
            "shared/partials/base".to_string(),
            "shared/partials/frontend-foundation".to_string(),
            "shared/partials/frontend-rust-ready".to_string(),
            "shared/partials/frontend-common".to_string(),
            "shared/partials/frontend".to_string(),
            "frontend/react-vite".to_string(),
        ]
    );
}

#[test]
fn fullstack_leaf_contract_controls_shared_partial_selection() {
    let root_dir = tempfile::tempdir().unwrap();
    let root = TemplateRoot::disk(root_dir.path().to_path_buf());
    let leaf = create_layer(root.path(), "fullstack/split/react-express", true);
    let base = create_layer(root.path(), "shared/partials/base", false);

    let layers =
        assemble_web_mode_layers(&root, "fullstack", "react-express", leaf, vec![base]).unwrap();

    assert_eq!(
        relative_layers(&layers, root.path()),
        vec![
            "shared/partials/base".to_string(),
            "shared/partials/frontend-foundation".to_string(),
            "shared/partials/frontend-rust-ready".to_string(),
            "shared/partials/frontend-common".to_string(),
            "fullstack/split/react-express".to_string(),
        ]
    );
}

#[test]
fn fullstack_leaf_without_contract_uses_mode_partial() {
    let root_dir = tempfile::tempdir().unwrap();
    let root = TemplateRoot::disk(root_dir.path().to_path_buf());
    let leaf = create_layer(root.path(), "fullstack/split/react-axum", false);
    let base = create_layer(root.path(), "shared/partials/base", false);

    let layers =
        assemble_web_mode_layers(&root, "fullstack", "react-axum", leaf, vec![base]).unwrap();

    assert_eq!(
        relative_layers(&layers, root.path()),
        vec![
            "shared/partials/base".to_string(),
            "shared/partials/fullstack".to_string(),
            "fullstack/split/react-axum".to_string(),
        ]
    );
}

fn fullstack_config(framework: &str) -> ScaffoldConfig {
    ScaffoldConfig {
        core: "web".to_string(),
        sub_type: "fullstack".to_string(),
        frameworks: vec![framework.to_string()],
        ..ScaffoldConfig::default()
    }
}

#[test]
fn split_non_node_backend_falls_back_only_when_its_leaf_is_missing() {
    let root_dir = tempfile::tempdir().unwrap();
    let root = TemplateRoot::disk(root_dir.path().to_path_buf());
    let config = fullstack_config("react-axum");

    assert!(fullstack_uses_monorepo_fallback(
        &root,
        &config,
        "fullstack",
        "react-axum"
    ));

    std::fs::create_dir_all(root.path().join("fullstack/split/react-axum")).unwrap();
    assert!(!fullstack_uses_monorepo_fallback(
        &root,
        &config,
        "fullstack",
        "react-axum"
    ));
}

#[test]
fn all_in_one_fullstack_framework_never_uses_split_fallback() {
    let root_dir = tempfile::tempdir().unwrap();
    let root = TemplateRoot::disk(root_dir.path().to_path_buf());
    let config = fullstack_config("nextjs");

    assert!(!fullstack_uses_monorepo_fallback(
        &root,
        &config,
        "fullstack",
        "nextjs"
    ));
}
