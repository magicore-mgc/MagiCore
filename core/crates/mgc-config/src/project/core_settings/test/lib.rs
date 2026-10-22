//! Library settings serde contract tests.
//! Test contract serde cho cấu hình Library.

use super::super::LibConfig;
use crate::project::ProjectConfig;

#[test]
fn defaults_and_empty_allowlist_omission_are_preserved() {
    let empty: LibConfig = serde_json::from_str("{}").expect("empty Lib config should deserialize");
    assert_eq!(empty, LibConfig::default());
    let value = serde_json::to_value(&empty).expect("default Lib config should serialize");
    assert_eq!(value, serde_json::json!({"language": ""}));
}

#[test]
fn explicit_language_and_allowlist_roundtrip() {
    let config = LibConfig {
        language: "python".to_string(),
        pip_allowed_packages: vec!["requests".to_string()],
    };
    let value = serde_json::to_value(&config).expect("Lib config should serialize");
    assert_eq!(
        value,
        serde_json::json!({"language": "python", "pip_allowed_packages": ["requests"]})
    );
    assert_eq!(
        serde_json::from_value::<LibConfig>(value).expect("Lib config should deserialize"),
        config
    );
}

#[test]
fn lib_scaffold_sets_language_and_empty_allowlist() {
    let cfg =
        ProjectConfig::from_scaffold("demo", "lib", "", vec!["python".to_string()], "", vec![]);
    let lib = cfg.lib.expect("[lib] must exist for lib core");
    assert_eq!(lib.language, "python");
    assert!(lib.pip_allowed_packages.is_empty());
}

#[test]
fn lib_scaffold_maps_ts_and_default_rust() {
    let ts =
        ProjectConfig::from_scaffold("a", "lib", "", vec!["typescript".to_string()], "", vec![]);
    assert_eq!(ts.lib.unwrap().language, "ts");
    let rust = ProjectConfig::from_scaffold("b", "lib", "", vec!["bevy".to_string()], "", vec![]);
    assert_eq!(rust.lib.unwrap().language, "rust");
}

#[test]
fn non_lib_has_no_lib_config() {
    let web = ProjectConfig::from_scaffold("c", "web", "", vec![], "", vec![]);
    assert!(web.lib.is_none());
}

#[test]
fn lib_scaffold_maps_go_dotnet_java_languages() {
    for (framework, language) in [
        ("go", "go"),
        ("golang", "go"),
        ("dotnet", "dotnet"),
        ("java", "java"),
    ] {
        let cfg = ProjectConfig::from_scaffold(
            "demo",
            "lib",
            "",
            vec![framework.to_string()],
            "",
            vec![],
        );
        assert_eq!(
            cfg.lib.unwrap().language,
            language,
            "framework {framework} must map to language {language}"
        );
    }
}
