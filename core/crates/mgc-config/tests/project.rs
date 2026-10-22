#![allow(clippy::unwrap_used)]
//! Integration tests for ProjectConfig — test riêng tại test/ (RULE §5)
use mgc_config::project::ProjectConfig;
use std::path::PathBuf;

fn temp_test_dir(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!(
        "magicore-mgc-config-{name}-{}",
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    std::fs::create_dir_all(&dir).unwrap();
    dir
}

#[test]
fn new_sets_version_and_fields() {
    let cfg = ProjectConfig::new("my-proj", "web");
    assert_eq!(cfg.name, "my-proj");
    assert_eq!(cfg.version, "0.1.0");
    assert_eq!(cfg.ecosystem, "web");
    assert!(cfg.mode.is_empty());
    assert!(cfg.frameworks.is_empty());
    assert_eq!(cfg.execution.architecture, "rust-first");
    assert_eq!(cfg.execution.lane, "compatibility-shell");
    assert_eq!(cfg.execution.compatibility_layer, "js");
}

#[test]
fn from_scaffold_sets_all_fields() {
    let cfg = ProjectConfig::from_scaffold(
        "my-app",
        "web",
        "frontend",
        vec!["react-vite".to_string()],
        "templates/web/frontend/react-vite",
        vec!["ts".to_string(), "tailwind".to_string()],
    );
    assert_eq!(cfg.name, "my-app");
    assert_eq!(cfg.ecosystem, "web");
    assert_eq!(cfg.mode, "frontend");
    assert_eq!(cfg.frameworks, vec!["react-vite"]);
    assert_eq!(cfg.template, "templates/web/frontend/react-vite");
    assert_eq!(cfg.features, vec!["ts", "tailwind"]);
    assert_eq!(cfg.execution.architecture, "rust-first");
    assert_eq!(cfg.execution.lane, "compatibility-shell");
    assert_eq!(cfg.execution.compatibility_layer, "ts");
    assert!(
        cfg.execution
            .native_targets
            .contains(&"frontend-executable".to_string())
    );
}

#[test]
fn load_missing_returns_none() {
    let dir = temp_test_dir("load-missing");
    assert!(ProjectConfig::load(&dir).unwrap().is_none());
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn load_valid_file() {
    let dir = temp_test_dir("load-valid");
    let cfg = ProjectConfig::new("test", "web");
    cfg.save(&dir).unwrap();
    let loaded = ProjectConfig::load(&dir).unwrap().unwrap();
    assert_eq!(loaded.name, "test");
    assert_eq!(loaded.ecosystem, "web");
    assert_eq!(loaded.version, "0.1.0");
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn load_invalid_toml_returns_err() {
    let dir = temp_test_dir("load-invalid");
    std::fs::write(dir.join("mgc.toml"), "[[[invalid").unwrap();
    assert!(ProjectConfig::load(&dir).is_err());
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn save_creates_file_at_root() {
    let dir = temp_test_dir("save-dir");
    let cfg = ProjectConfig::new("save-test", "lib");
    cfg.save(&dir).unwrap();
    let path = dir.join("mgc.toml");
    assert!(path.exists());
    let content = std::fs::read_to_string(&path).unwrap();
    assert!(content.contains("save-test"));
    assert!(content.contains("lib"));
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn save_roundtrip() {
    let dir = temp_test_dir("save-roundtrip");
    let cfg = ProjectConfig::new("roundtrip", "ai");
    cfg.save(&dir).unwrap();
    let loaded = ProjectConfig::load(&dir).unwrap().unwrap();
    assert_eq!(loaded.name, "roundtrip");
    assert_eq!(loaded.ecosystem, "ai");
    assert_eq!(loaded.version, "0.1.0");
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn find_root_detects_project_marker() {
    let dir = temp_test_dir("find-root");
    std::fs::create_dir_all(dir.join("sub")).unwrap();
    ProjectConfig::new("root-proj", "web").save(&dir).unwrap();
    let found = ProjectConfig::find_project_root(&dir.join("sub")).unwrap();
    assert_eq!(found, dir);
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn save_roundtrip_with_scaffold_fields() {
    let dir = temp_test_dir("save-scaffold");
    let cfg = ProjectConfig::from_scaffold(
        "roundtrip",
        "web",
        "frontend",
        vec!["react-vite".to_string()],
        "templates/web/frontend/react-vite",
        vec!["ts".to_string()],
    );
    cfg.save(&dir).unwrap();
    let loaded = ProjectConfig::load(&dir).unwrap().unwrap();
    assert_eq!(loaded.name, "roundtrip");
    assert_eq!(loaded.ecosystem, "web");
    assert_eq!(loaded.mode, "frontend");
    assert_eq!(loaded.frameworks, vec!["react-vite"]);
    assert_eq!(loaded.template, "templates/web/frontend/react-vite");
    assert_eq!(loaded.features, vec!["ts"]);
    assert_eq!(loaded.execution.compatibility_layer, "ts");
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn auto_detect_package_json_returns_web() {
    let dir = temp_test_dir("auto-web");
    std::fs::write(dir.join("package.json"), "{}").unwrap();
    assert_eq!(
        ProjectConfig::auto_detect(&dir).unwrap(),
        Some("web".to_string())
    );
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn auto_detect_cargo_toml_returns_lib() {
    let dir = temp_test_dir("auto-lib");
    std::fs::write(dir.join("Cargo.toml"), "").unwrap();
    assert_eq!(
        ProjectConfig::auto_detect(&dir).unwrap(),
        Some("lib".to_string())
    );
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn auto_detect_generic_pyproject_toml_returns_lib() {
    let dir = temp_test_dir("auto-ai");
    std::fs::write(dir.join("pyproject.toml"), "").unwrap();
    assert_eq!(
        ProjectConfig::auto_detect(&dir).unwrap(),
        Some("lib".to_string())
    );
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn auto_detect_ai_python_project_from_declared_framework() {
    let dir = temp_test_dir("auto-ai-framework");
    std::fs::write(
        dir.join("pyproject.toml"),
        "[project]\nname = \"model-service\"\ndependencies = [\"torch>=2\"]\n",
    )
    .unwrap();
    assert_eq!(
        ProjectConfig::auto_detect(&dir).unwrap(),
        Some("ai".to_string())
    );
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn auto_detect_no_manifest_returns_none() {
    let dir = temp_test_dir("auto-none");
    assert_eq!(ProjectConfig::auto_detect(&dir).unwrap(), None);
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn auto_detect_pubspec_returns_app() {
    let dir = temp_test_dir("auto-app");
    std::fs::write(dir.join("pubspec.yaml"), "name: a\n").unwrap();
    assert_eq!(
        ProjectConfig::auto_detect(&dir).unwrap(),
        Some("app".to_string())
    );
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn auto_detect_mgc_toml_ecosystem_wins() {
    let dir = temp_test_dir("auto-mgc");
    std::fs::write(
        dir.join("mgc.toml"),
        "name = \"x\"\necosystem = \"game\"\n[game]\nengine = \"bevy\"\n",
    )
    .unwrap();
    assert_eq!(
        ProjectConfig::auto_detect(&dir).unwrap(),
        Some("game".to_string())
    );
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn find_project_root_mgc_toml_in_cwd() {
    let dir = temp_test_dir("root-cwd-mgc");
    std::fs::write(dir.join("mgc.toml"), "").unwrap();
    assert_eq!(ProjectConfig::find_project_root(&dir), Some(dir.clone()));
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn find_project_root_package_json_in_cwd() {
    let dir = temp_test_dir("root-cwd-pkg");
    std::fs::write(dir.join("package.json"), "{}").unwrap();
    assert_eq!(ProjectConfig::find_project_root(&dir), Some(dir.clone()));
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn find_project_root_no_match_returns_none() {
    let dir = temp_test_dir("root-none");
    assert_eq!(ProjectConfig::find_project_root(&dir), None);
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn find_project_root_uses_nearest_parent_language_manifest() {
    let root = temp_test_dir("parent-package-json");
    let child = root.join("apps").join("frontend");
    std::fs::create_dir_all(&child).unwrap();
    std::fs::write(root.join("package.json"), "{}").unwrap();

    assert_eq!(ProjectConfig::find_project_root(&child), Some(root.clone()));
    let _ = std::fs::remove_dir_all(root);
}

#[test]
fn find_project_root_prefers_nearest_manifest_over_monorepo_parent() {
    let root = temp_test_dir("nearest-parent-manifest");
    let nested = root.join("apps").join("service");
    let child = nested.join("src");
    std::fs::create_dir_all(&child).unwrap();
    std::fs::write(root.join("package.json"), "{}").unwrap();
    std::fs::write(nested.join("go.mod"), "module example.com/service\n").unwrap();

    assert_eq!(
        ProjectConfig::find_project_root(&child),
        Some(nested.clone()),
        "commands inside a nested project must not attach to the monorepo package root"
    );
    let _ = std::fs::remove_dir_all(root);
}

#[test]
fn find_project_root_accepts_parent_mgc_toml() {
    let root = temp_test_dir("parent-mgc-toml");
    let child = root.join("apps").join("frontend");
    std::fs::create_dir_all(&child).unwrap();
    std::fs::write(root.join("mgc.toml"), "").unwrap();

    assert_eq!(ProjectConfig::find_project_root(&child), Some(root.clone()));
    let _ = std::fs::remove_dir_all(root);
}

#[test]
fn unsigned_artifact_escape_is_per_ecosystem_only() {
    use mgc_config::project::unsigned_artifact_allowed;
    let allowed = vec!["python".to_string(), " Go ".to_string()];
    assert!(unsigned_artifact_allowed(&allowed, "python"));
    assert!(unsigned_artifact_allowed(&allowed, "go"));
    assert!(!unsigned_artifact_allowed(&allowed, "rust"));
    assert!(!unsigned_artifact_allowed(&[], "python"));
    // A "*" wildcard never opts anything out.
    assert!(!unsigned_artifact_allowed(&["*".to_string()], "python"));
}

#[test]
fn security_config_parses_unsigned_escape_list() {
    use mgc_config::project::ProjectConfig;
    let dir = tempfile::tempdir().unwrap();
    std::fs::write(
        dir.path().join("mgc.toml"),
        "name = \"x\"\necosystem = \"lib\"\n[security]\nallow_unsigned_artifacts = [\"python\"]\n",
    )
    .unwrap();
    let cfg = ProjectConfig::load(dir.path()).unwrap().unwrap();
    let security = cfg.security.unwrap();
    assert_eq!(
        security.allow_unsigned_artifacts,
        Some(vec!["python".to_string()])
    );
}

#[test]
fn script_decision_deny_wins_from_either_source() {
    use mgc_config::project::{ScriptVerdict, ScriptsPolicy, decide_scripts};
    // Regression: a file allow must NEVER override a DB deny (this drift
    // shipped once — install ran scripts the DB explicitly denied).
    // (Hồi quy: file allow không bao giờ ghi đè DB deny.)
    let file = ScriptsPolicy {
        policy: None,
        allow: vec!["evil-pkg".to_string()],
        deny: vec![],
    };
    assert!(matches!(
        decide_scripts("evil-pkg", "1.0.0", Some(&file), Some("denied"), false),
        ScriptVerdict::Deny("mgc trust deny")
    ));
    // File deny beats DB approve.
    let file = ScriptsPolicy {
        policy: None,
        allow: vec![],
        deny: vec!["evil-pkg@1.0.0".to_string()],
    };
    assert!(matches!(
        decide_scripts("evil-pkg", "1.0.0", Some(&file), Some("approved"), false),
        ScriptVerdict::Deny("mgc.toml [scripts] deny")
    ));
    // File allow with no DB opinion allows.
    let file = ScriptsPolicy {
        policy: None,
        allow: vec!["nice-pkg".to_string()],
        deny: vec![],
    };
    assert!(matches!(
        decide_scripts("nice-pkg", "2.0.0", Some(&file), None, false),
        ScriptVerdict::Allow(_)
    ));
    // Nothing anywhere + blanket off = undecided (caller maps to
    // skip-with-hint on install, pending-review on trust pending).
    assert!(matches!(
        decide_scripts("plain-pkg", "1.0.0", None, None, false),
        ScriptVerdict::Undecided
    ));
    // Blanket on preserves historical default-allow.
    assert!(matches!(
        decide_scripts("plain-pkg", "1.0.0", None, None, true),
        ScriptVerdict::Allow(_)
    ));
    // Default-deny policy file denies unlisted packages.
    let file = ScriptsPolicy {
        policy: Some("deny".to_string()),
        allow: vec![],
        deny: vec![],
    };
    assert!(matches!(
        decide_scripts("plain-pkg", "1.0.0", Some(&file), None, true),
        ScriptVerdict::Deny(_)
    ));
}

#[cfg(unix)]
#[test]
fn scripts_policy_rejects_symlinked_project_config() {
    use mgc_config::project::load_scripts_table;

    let dir = temp_test_dir("scripts-config-symlink");
    let external_dir = temp_test_dir("scripts-config-external");
    let outside = external_dir.join("mgc.toml");
    std::fs::write(&outside, "[scripts]\npolicy = 'deny'\n").unwrap();
    std::os::unix::fs::symlink(&outside, dir.join("mgc.toml")).unwrap();

    let result = load_scripts_table(&dir);
    assert!(
        result.is_err(),
        "a symlinked project config must not supply lifecycle trust policy"
    );
    assert!(result.unwrap_err().contains("must not be a symlink"));
    let _ = std::fs::remove_dir_all(&dir);
    let _ = std::fs::remove_dir_all(&external_dir);
}

#[test]
fn script_grammar_pin_wins_over_default() {
    use mgc_config::project::default_script_grammar;
    let dir = temp_test_dir("grammar-pin");
    std::fs::write(
        dir.join("mgc.toml"),
        "name = \"g\"\necosystem = \"web\"\n[script]\ngrammar = \"web-script-v9\"\n",
    )
    .unwrap();
    let loaded = ProjectConfig::load(&dir).unwrap().unwrap();
    // Pinned version returned verbatim so the caller can fail closed on it.
    // (Pin trả nguyên văn để caller fail-closed đúng version đó.)
    assert_eq!(loaded.script_grammar(), "web-script-v9");
    assert_eq!(default_script_grammar("web"), "web-script-v1");
    assert_eq!(default_script_grammar("ai"), "ai-script-v1");
    assert_eq!(default_script_grammar("unknown-eco"), "web-script-v1");
}

#[test]
fn script_grammar_absent_falls_back_to_ecosystem_default() {
    let dir = temp_test_dir("grammar-absent");
    std::fs::write(dir.join("mgc.toml"), "name = \"g\"\necosystem = \"ai\"\n").unwrap();
    let loaded = ProjectConfig::load(&dir).unwrap().unwrap();
    assert!(loaded.script.is_none());
    assert_eq!(loaded.script_grammar(), "ai-script-v1");
}

#[test]
fn lock_schema_defaults_to_v3_and_accepts_v4() {
    let dir = temp_test_dir("lock-schema");
    std::fs::write(dir.join("mgc.toml"), "name = \"g\"\necosystem = \"web\"\n").unwrap();
    let loaded = ProjectConfig::load(&dir).unwrap().unwrap();
    assert!(loaded.lock.is_none());

    std::fs::write(
        dir.join("mgc.toml"),
        "name = \"g\"\necosystem = \"web\"\n[lock]\nschema = \"v4\"\n",
    )
    .unwrap();
    let loaded = ProjectConfig::load(&dir).unwrap().unwrap();
    assert_eq!(loaded.lock.unwrap().schema.as_deref(), Some("v4"));
}

#[test]
fn omitted_execution_uses_constructor_defaults_across_primary_cores() {
    for core in ["web", "ai", "app", "lib"] {
        let dir = tempfile::tempdir().unwrap();
        std::fs::write(
            dir.path().join("mgc.toml"),
            format!("name = \"probe\"\necosystem = \"{core}\"\n"),
        )
        .unwrap();
        let loaded = ProjectConfig::load(dir.path()).unwrap().unwrap();
        assert_eq!(
            loaded.execution,
            ProjectConfig::new("probe", core).execution
        );
    }
}

#[test]
fn explicit_execution_profile_is_preserved_when_loading_nonweb_core() {
    let dir = tempfile::tempdir().unwrap();
    std::fs::write(dir.path().join("mgc.toml"),
        "name = \"probe\"\necosystem = \"lib\"\n[execution]\narchitecture = \"custom\"\nlane = \"custom-lane\"\ncompatibility_layer = \"custom-runtime\"\n").unwrap();
    let loaded = ProjectConfig::load(dir.path()).unwrap().unwrap();
    assert_eq!(loaded.execution.architecture, "custom");
    assert_eq!(loaded.execution.lane, "custom-lane");
    assert_eq!(loaded.execution.compatibility_layer, "custom-runtime");
}
