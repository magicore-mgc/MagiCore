//! Existing end-to-end coverage for monorepo and fullstack layer composition.
//! Test E2E hiện có cho ghép layer monorepo và fullstack.

#![allow(clippy::unwrap_used)]

use crate::scaffold::processor::Scaffolder;
use crate::scaffold::template_root::TemplateRoot;
use crate::wizard::engine::ScaffoldConfig;
use std::path::PathBuf;

fn template_layer_ready(rel: &str) -> bool {
    let root = TemplateRoot::resolve(rel);
    root.exists("template.toml") && root.exists("sources")
}

#[test]
fn test_web_monorepo_uses_template_layers() {
    if !["web/frontend/react-vite", "web/backend/node/fastify"]
        .iter()
        .all(|rel| template_layer_ready(rel))
    {
        eprintln!("skipped: web template layers not available offline (registry-first)");
        return;
    }
    let root = tempfile::tempdir().unwrap();
    let project_dir = root.path().join("web-monorepo");
    let config = ScaffoldConfig {
        core: "web".to_string(),
        sub_type: "monorepo".to_string(),
        frameworks: vec!["react-vite".to_string(), "fastify".to_string()],
        project_name: project_dir.to_string_lossy().to_string(),
        features: vec!["schema".to_string()],
        template_dir: PathBuf::new(),
        from_local: None,
        no_local: false,
    };

    let out = Scaffolder::scaffold(&config).unwrap();
    assert!(out.join("mgc.lock").exists());
    assert!(out.join("mgc.toml").exists());
    assert!(out.join("magicore.workspace.toml").exists());
    let root_package = std::fs::read_to_string(out.join("package.json")).unwrap();
    assert!(root_package.contains("\"dev\": \"mgc --core web dev\""));
    assert!(!root_package.contains("mgc web build"));
    assert!(!root_package.contains("mgc web check"));
    let readme = std::fs::read_to_string(out.join("README.md")).unwrap();
    assert!(readme.contains("mgc install"));
    assert!(out.join("apps").join("frontend").join("README.md").exists());
    assert!(out.join("apps").join("backend").join("README.md").exists());
    assert!(out.join("packages").join("README.md").exists());
    assert!(
        out.join("apps")
            .join("frontend")
            .join("crates")
            .join("engine")
            .join("Cargo.toml")
            .exists()
    );
    assert!(
        out.join("apps")
            .join("frontend")
            .join("src")
            .join("bridges")
            .join("engine.js")
            .exists()
    );
    assert!(
        out.join("packages")
            .join("contracts")
            .join("package.json")
            .exists()
    );
    assert!(
        out.join("apps")
            .join("frontend")
            .join("vite.config.js")
            .exists()
    );
    assert!(
        out.join("apps")
            .join("backend")
            .join("src")
            .join("server.js")
            .exists()
    );
}
#[test]
fn test_fullstack_axum_falls_back_to_monorepo_composite() {
    if ![
        "web/frontend/react-vite",
        "web/monorepo/base",
        "web/monorepo/frontend/react-vite",
        "web/monorepo/backend/rust/axum",
    ]
    .iter()
    .all(|rel| template_layer_ready(rel))
    {
        eprintln!("skipped: web template layers not available offline (registry-first)");
        return;
    }
    let root = tempfile::tempdir().unwrap();
    let project_dir = root.path().join("react-axum-app");
    let config = ScaffoldConfig {
        core: "web".to_string(),
        sub_type: "fullstack".to_string(),
        frameworks: vec!["react-axum".to_string()],
        project_name: project_dir.to_string_lossy().to_string(),
        features: vec![],
        template_dir: PathBuf::new(),
        from_local: None,
        no_local: false,
    };

    let out = Scaffolder::scaffold(&config).unwrap();
    assert!(out.join("magicore.workspace.toml").exists());
    assert!(
        out.join("apps")
            .join("frontend")
            .join("package.json")
            .exists()
    );
    let back_cargo =
        std::fs::read_to_string(out.join("apps").join("backend").join("Cargo.toml")).unwrap();
    assert!(
        back_cargo.contains("axum"),
        "backend Cargo.toml should pin axum, got: {back_cargo}"
    );
    assert!(
        out.join("apps")
            .join("backend")
            .join("src")
            .join("main.rs")
            .exists()
    );
    assert!(
        !out.join("templates")
            .join("web")
            .join("fullstack")
            .join("split")
            .join("react-axum")
            .exists(),
        "no hardcoded split leaf was added"
    );
}
#[test]
fn test_fullstack_gin_falls_back_to_monorepo_composite() {
    if ![
        "web/frontend/react-vite",
        "web/monorepo/base",
        "web/monorepo/frontend/react-vite",
        "web/monorepo/backend/go/gin",
    ]
    .iter()
    .all(|rel| template_layer_ready(rel))
    {
        eprintln!("skipped: web template layers not available offline (registry-first)");
        return;
    }
    let root = tempfile::tempdir().unwrap();
    let project_dir = root.path().join("react-gin-app");
    let config = ScaffoldConfig {
        core: "web".to_string(),
        sub_type: "fullstack".to_string(),
        frameworks: vec!["react-gin".to_string()],
        project_name: project_dir.to_string_lossy().to_string(),
        features: vec![],
        template_dir: PathBuf::new(),
        from_local: None,
        no_local: false,
    };

    let out = Scaffolder::scaffold(&config).unwrap();
    assert!(out.join("magicore.workspace.toml").exists());
    assert!(out.join("apps").join("backend").join("go.mod").exists());
    let go_mod = std::fs::read_to_string(out.join("apps").join("backend").join("go.mod")).unwrap();
    assert!(
        go_mod.contains("gin"),
        "backend go.mod should pin gin, got: {go_mod}"
    );
}
