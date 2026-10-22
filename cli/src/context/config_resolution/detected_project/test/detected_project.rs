//! Verify detected-core conflict rejection and marker creation.
//! Kiểm tra từ chối core lệch chữ ký và tạo marker khi nhận diện thành công.

use crate::context::config_resolution::resolve_config;

#[test]
fn explicit_core_override_cannot_erase_a_single_detected_signature() {
    let project = tempfile::tempdir().expect("temporary project directory must be created");
    std::fs::write(project.path().join("package.json"), "{}\n")
        .expect("web signature must be written");

    let root = project.path().to_path_buf();
    let result = resolve_config(&root, Some(&root), Some("ai"));
    let error = match result {
        Err(error) => error,
        Ok(_) => panic!("core override must not contradict an unambiguous project signature"),
    };
    assert!(
        error.to_string().contains("core override 'ai' conflicts")
            && error.to_string().contains("'web'"),
        "signature conflict should be explicit, got: {error}"
    );
    assert!(
        !root.join(".mgc.core").exists(),
        "rejected override must not write an identity marker"
    );

    resolve_config(&root, Some(&root), Some("web"))
        .expect("matching override should claim the detected core");
    assert_eq!(
        mgc_config::project::ProjectConfig::read_core_marker(&root)
            .expect("claimed signature must remain readable")
            .as_deref(),
        Some("web")
    );
}

#[test]
fn project_context_claims_the_detected_framework_core_from_nested_cwd() {
    let project = tempfile::tempdir().expect("temporary project directory must be created");
    std::fs::write(
        project.path().join("pyproject.toml"),
        "[project]\nname = \"api\"\nversion = \"0.1.0\"\ndependencies = [\"fastapi>=0.1\"]\n",
    )
    .expect("Python web manifest must be written");
    let nested = project.path().join("src").join("api");
    std::fs::create_dir_all(&nested).expect("nested source directory must be created");

    let root = mgc_config::project::ProjectConfig::find_project_root(&nested)
        .expect("parent project manifest must be discovered");
    let (_, config) = resolve_config(&nested, Some(&root), None)
        .expect("project context must select the web core from FastAPI metadata");

    assert_eq!(config.ecosystem, "web");
    assert_eq!(
        mgc_config::project::ProjectConfig::read_core_marker(&root)
            .expect("detected project marker must be readable")
            .as_deref(),
        Some("web")
    );
}
