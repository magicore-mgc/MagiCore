//! Monorepo dev pre-flight tests — kiểm thử pre-flight dev monorepo.
//!
//! Every launched workspace target must pass dependency freshness before any
//! target script is spawned.
//! (Mọi target workspace được chạy phải qua freshness trước khi spawn script.)

#![allow(clippy::unwrap_used)]

use std::process::{Command, Output};
use tempfile::TempDir;

fn mgc_binary() -> String {
    std::env::var("CARGO_BIN_EXE_mgc").expect("CARGO_BIN_EXE_mgc must be set by cargo test")
}

fn lock_with(name: &str, version: &str) -> String {
    let mut lock = mgc_lockfile::Lockfile::new();
    if !name.is_empty() {
        let mut package = mgc_lockfile::Package::new(
            name.to_string(),
            version.to_string(),
            format!("https://registry.npmjs.org/{name}/-/{name}-{version}.tgz"),
            "sha256-test-integrity".to_string(),
        );
        package.owner_core = Some("web".to_string());
        package.ecosystem = mgc_lockfile::EcosystemTag::Web;
        package.registry = Some("https://registry.npmjs.org/".to_string());
        lock.add_package(package);
        mgc_lockfile::update_owner_root_pins(
            &mut lock,
            "web",
            mgc_lockfile::EcosystemTag::Web,
            [format!("{name}@{version}")],
        );
    }
    mgc_lockfile::writer::serialize_lockfile(&lock).unwrap()
}

fn run_mgc(args: &[&str], cwd: &std::path::Path, isolated: &TempDir) -> Output {
    let data = isolated.path().join("system-data");
    let cache = isolated.path().join("cache");
    let templates = isolated.path().join("templates");
    std::fs::create_dir_all(&data).unwrap();
    std::fs::create_dir_all(&cache).unwrap();
    std::fs::create_dir_all(&templates).unwrap();

    Command::new(mgc_binary())
        .args(args)
        .current_dir(cwd)
        .env("HOME", isolated.path())
        .env("USERPROFILE", isolated.path())
        .env("MGC_DATA_DIR", data)
        .env("MGC_CACHE_DIR", cache)
        .env("MGC_TEMPLATES_DIR", templates)
        .env_remove("MGC_POLICY_MANIFEST_URL")
        .env_remove("MGC_POLICY_MANIFEST_OCI")
        .output()
        .unwrap()
}

fn combined_output(output: &Output) -> String {
    format!(
        "{}{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    )
}

#[test]
fn stale_workspace_target_fails_before_any_dev_script_spawns() {
    let isolated = TempDir::new().unwrap();
    let root = isolated.path().join("workspace");
    let frontend = root.join("apps/frontend");
    let backend = root.join("apps/backend");
    std::fs::create_dir_all(&frontend).unwrap();
    std::fs::create_dir_all(&backend).unwrap();
    std::fs::write(
        root.join("magicore.workspace.toml"),
        "mode = \"monorepo\"\n[layout]\napps_dir = \"apps\"\n",
    )
    .unwrap();

    let script = "node mark.js";
    std::fs::write(
        frontend.join("mark.js"),
        "require('fs').writeFileSync('launched', 'yes');\n",
    )
    .unwrap();
    std::fs::write(
        backend.join("mark.js"),
        "require('fs').writeFileSync('launched', 'yes');\n",
    )
    .unwrap();
    std::fs::write(
        frontend.join("package.json"),
        serde_json::json!({
            "name": "frontend",
            "scripts": { "dev": script },
            "dependencies": { "next": "16.3.8", "left-pad": "1.3.0" }
        })
        .to_string(),
    )
    .unwrap();
    std::fs::write(frontend.join("mgc.lock"), lock_with("next", "16.3.8")).unwrap();

    std::fs::write(
        backend.join("package.json"),
        serde_json::json!({
            "name": "backend",
            "scripts": { "dev": script },
            "dependencies": {}
        })
        .to_string(),
    )
    .unwrap();
    std::fs::write(backend.join("mgc.lock"), lock_with("", "")).unwrap();

    assert!(
        mgc::commands::core::web::check_dev_freshness(&frontend).is_err(),
        "the frontend fixture itself must be stale"
    );

    let output = run_mgc(
        &["--core", "web", "dev", "--port", "43987"],
        &root,
        &isolated,
    );
    let text = combined_output(&output);
    assert!(
        !output.status.success(),
        "stale workspace target was launched:\n{text}"
    );
    assert!(
        text.contains("project dependencies changed") && text.contains("left-pad"),
        "expected freshness error for the frontend target:\n{text}"
    );
    assert!(
        !frontend.join("launched").exists() && !backend.join("launched").exists(),
        "a workspace dev script spawned before all target pre-flights completed"
    );
}
