// Scaffold output and delivery descriptor tests.
// Kiểm thử scaffold output và descriptor triển khai.
//
// MagiCore never synthesizes framework source: these tests can only assert
// the data it owns (board mapping, fail-closed scaffolding).
// MagiCore không tự sinh source framework: test ở đây chỉ khẳng định dữ liệu
// nó sở hữu (map board, scaffold fail-closed).

#[test]
fn iot_board_identifiers_resolve_to_their_registered_framework() {
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
            crate::scaffold::framework_registry::iot_framework_for_board(board),
            Some(framework),
            "board {board}"
        );
        assert!(crate::scaffold::framework_registry::is_supported(
            "iot", board
        ));
    }
    assert_eq!(
        crate::scaffold::framework_registry::iot_default_board("platformio"),
        Some("esp32dev")
    );
    assert_eq!(
        crate::scaffold::framework_registry::iot_default_board("esp32-rust"),
        Some("esp32c3")
    );
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
