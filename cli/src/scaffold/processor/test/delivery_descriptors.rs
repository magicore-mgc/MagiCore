// Scaffold output and delivery descriptor tests.
// Kiểm thử scaffold output và descriptor triển khai.

#[test]
fn github_actions_scaffold_refuses_without_declared_ci_commands() {
    let root = tempfile::tempdir().unwrap();
    // No mgc.toml [ci] and no MGC_CI_* environment: the generator must refuse
    // instead of writing a placeholder workflow.
    // Không có [ci] và không có biến MGC_CI_*: generator phải từ chối.
    let error = crate::scaffold::processors::cicd::CicdProcessor::files(
        root.path(),
        "sample",
        "github-actions",
    )
    .expect_err("must refuse without ci.build/ci.test");
    assert!(error.to_string().contains("ci.build and ci.test"));
    assert!(!root.path().join(".github/workflows/ci.yml").exists());
}

#[test]
fn github_actions_scaffold_writes_declared_commands_and_least_privilege() {
    let root = tempfile::tempdir().unwrap();
    std::fs::write(
        root.path().join("mgc.toml"),
        "name = \"sample\"\nversion = \"0.1.0\"\necosystem = \"web\"\nframeworks = [\"django\"]\n\n[ci]\nbuild = \"python -m build --wheel\"\ntest = \"pytest -q\"\n",
    )
    .unwrap();
    crate::scaffold::processors::cicd::CicdProcessor::files(
        root.path(),
        "sample",
        "github-actions",
    )
    .unwrap();
    let workflow = std::fs::read_to_string(root.path().join(".github/workflows/ci.yml")).unwrap();
    assert!(!workflow.contains("echo \"MagiCore CI scaffold\""));
    assert!(workflow.contains("python -m build --wheel"));
    assert!(workflow.contains("pytest -q"));
    assert!(workflow.contains("permissions:\n  contents: read\n"));
    // Publish only appears together with a declared production artifact.
    assert!(!workflow.contains("publish:"));
}

#[test]
fn cicd_cloud_provider_scaffolds_have_native_build_descriptors() {
    let root = tempfile::tempdir().unwrap();
    for (framework, path, marker) in [
        ("aws", "buildspec.yml", "version: 0.2"),
        ("gcp", "cloudbuild.yaml", "CLOUD_LOGGING_ONLY"),
    ] {
        assert!(crate::scaffold::processors::cicd::CicdProcessor::supports(
            framework
        ));
        crate::scaffold::processors::cicd::CicdProcessor::files(root.path(), "sample", framework)
            .unwrap();
        let contents = std::fs::read_to_string(root.path().join(path)).unwrap();
        assert!(
            contents.contains(marker),
            "{framework} descriptor is incomplete"
        );
    }
}
#[test]
fn iot_board_identifiers_resolve_to_their_registered_generator() {
    for (board, framework) in [
        ("esp32", "esp32-rust"),
        ("esp32c3", "esp32-rust"),
        ("esp32s3", "esp32-rust"),
        ("esp32dev", "platformio"),
        ("nodemcu-32s", "platformio"),
        ("nrf52dk_nrf52832", "zephyr-arm"),
        ("stm32f4_disc", "zephyr-arm"),
    ] {
        assert_eq!(
            crate::scaffold::processors::iot::IotProcessor::framework_for_board(board),
            Some(framework),
            "board {board}"
        );
        assert!(crate::scaffold::processors::iot::IotProcessor::supports(
            board
        ));
    }
    assert_eq!(
        crate::scaffold::processors::iot::IotProcessor::default_board("platformio"),
        Some("esp32dev")
    );
}
#[test]
fn zephyr_scaffold_contains_a_real_build_entrypoint() {
    let root = tempfile::tempdir().unwrap();
    crate::scaffold::processors::iot::IotProcessor::files(
        root.path(),
        "sample",
        "zephyr-arm",
        "nrf52dk_nrf52832",
    )
    .unwrap();
    assert!(root.path().join("west.yml").is_file());
    assert!(root.path().join("CMakeLists.txt").is_file());
    assert!(root.path().join("prj.conf").is_file());
    let source = std::fs::read_to_string(root.path().join("src/main.c")).unwrap();
    assert!(source.contains("int main(void)"));
}
#[test]
fn web_scaffold_fails_closed_when_template_layer_is_missing() {
    let root = tempfile::tempdir().unwrap();
    let project = root.path().join("missing-template-project");
    let mut config =
        Scaffolder::infer_web_create_config("express", project.to_str().unwrap()).unwrap();
    config.template_dir = root.path().join("missing-express-template");

    assert!(
        Scaffolder::scaffold(&config).is_err(),
        "web scaffold must require a resolved template layer"
    );
    assert!(
        !project.exists(),
        "failed template resolution must not leave a partial project"
    );
}
#[test]
fn unsupported_backend_scaffolds_fail_without_creating_placeholder_projects() {
    let root = tempfile::tempdir().unwrap();
    let mut dotnet = Scaffolder::infer_web_create_config(
        "dotnet-webapi",
        root.path().join("dotnet").to_str().unwrap(),
    )
    .unwrap();
    dotnet.template_dir = root.path().join("missing-dotnet-template");

    let project = std::path::PathBuf::from(&dotnet.project_name);
    assert!(
        Scaffolder::scaffold(&dotnet).is_err(),
        "dotnet without a real template or native adapter must fail"
    );
    assert!(
        !project.exists(),
        "unsupported backend failure must not leave a partial project"
    );
}
#[test]
fn cloudflare_scaffold_has_worker_entry_source() {
    let root = tempfile::tempdir().unwrap();
    crate::scaffold::processors::clo::CloProcessor::files(root.path(), "worker-demo", "cloudflare")
        .unwrap();
    let wrangler = std::fs::read_to_string(root.path().join("wrangler.toml")).unwrap();
    assert!(wrangler.contains("main = \"src/index.ts\""));
    assert!(root.path().join("src/index.ts").is_file());
}
#[test]
fn godot_and_unreal_scaffolds_contain_engine_source() {
    let root = tempfile::tempdir().unwrap();
    let godot = root.path().join("godot");
    crate::scaffold::processors::game::GameProcessor::files(&godot, "godot-demo", "godot").unwrap();
    assert!(godot.join("Main.gd").is_file());
    assert!(
        std::fs::read_to_string(godot.join("Main.tscn"))
            .unwrap()
            .contains("res://Main.gd")
    );

    let unreal = root.path().join("unreal");
    crate::scaffold::processors::game::GameProcessor::files(&unreal, "unreal-demo", "unreal")
        .unwrap();
    let descriptor: serde_json::Value =
        serde_json::from_slice(&std::fs::read(unreal.join("unreal-demo.uproject")).unwrap())
            .unwrap();
    assert_eq!(descriptor["Modules"][0]["Name"], "MagiCoreGame");
    assert!(
        unreal
            .join("Source/MagiCoreGame/MagiCoreGame.cpp")
            .is_file()
    );
    assert!(
        unreal
            .join("Source/MagiCoreGame/MagiCoreGame.Build.cs")
            .is_file()
    );
}
#[test]
fn java_and_dotnet_library_scaffolds_include_library_source() {
    let root = tempfile::tempdir().unwrap();
    for framework in ["java", "dotnet"] {
        let project = root.path().join(framework);
        crate::scaffold::processors::lib::LibProcessor::files(&project, "sample-lib", framework)
            .unwrap();
        let source = if framework == "java" {
            project.join("src/main/java/dev/magicore/Library.java")
        } else {
            project.join("Class1.cs")
        };
        assert!(source.is_file(), "{framework} source file");
        assert!(!std::fs::read_to_string(source).unwrap().trim().is_empty());
    }
}
