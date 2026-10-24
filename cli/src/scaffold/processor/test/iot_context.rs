// IoT board and framework context tests.
// Kiểm thử board và context framework IoT.

#[test]
fn test_iot_templates_materialize_all_frameworks() {
    if !template_layer_ready("iot/esp32-rust") {
        eprintln!("skipped: iot/esp32-rust template layer not available offline (registry-first)");
        return;
    }
    let root = tempfile::tempdir().unwrap();
    for (framework, manifest, marker, board, fw_check) in [
        (
            "esp32-rust",
            "Cargo.toml",
            "esp32-hal",
            "esp32c3",
            "esp32-rust",
        ),
        (
            "platformio",
            "platformio.ini",
            "esp32dev",
            "esp32dev",
            "platformio",
        ),
        (
            "zephyr-arm",
            "west.yml",
            "zephyr",
            "nrf52dk_nrf52832",
            "zephyr",
        ),
    ] {
        let project_dir = root.path().join(format!("demo-{framework}"));
        let config = ScaffoldConfig {
            core: "iot".to_string(),
            sub_type: String::new(),
            frameworks: vec![framework.to_string()],
            project_name: project_dir.to_string_lossy().to_string(),
            features: vec![board.to_string()],
            template_dir: PathBuf::new(),
            from_local: None,
            no_local: false,
        };

        let out = Scaffolder::scaffold(&config).unwrap();
        assert_eq!(out, project_dir);
        assert!(out.join(manifest).exists(), "{framework} manifest");
        let mgc = std::fs::read_to_string(out.join("mgc.toml")).unwrap();
        assert!(
            mgc.contains("ecosystem = \"iot\""),
            "{framework} mgc.toml ecosystem"
        );
        assert!(
            mgc.contains(&format!("framework = \"{fw_check}\"")),
            "{framework} framework"
        );
        assert!(mgc.contains(board), "{framework} board");
        if framework == "esp32-rust" {
            assert!(
                mgc.contains("riscv32imac-unknown-none-elf"),
                "{framework} target"
            );
        }
        let native = std::fs::read_to_string(out.join(manifest)).unwrap();
        assert!(native.contains(marker), "{framework} marker");
    }
    let esp32_src = root
        .path()
        .join("demo-esp32-rust")
        .join("src")
        .join("main.rs");
    assert!(esp32_src.exists(), "esp32-rust source");
}
#[test]
fn scaffold_rejects_unknown_iot_board_without_leaving_staging_or_claims() {
    let root = tempfile::tempdir().unwrap();
    let target = root.path().join("unknown-board-firmware");
    let config = ScaffoldConfig {
        core: "iot".to_string(),
        sub_type: String::new(),
        frameworks: vec!["esp32-rust".to_string()],
        project_name: target.to_string_lossy().to_string(),
        features: vec!["unknown-board".to_string()],
        template_dir: PathBuf::new(),
        from_local: None,
        no_local: false,
    };

    assert!(Scaffolder::scaffold(&config).is_err());
    assert!(!target.exists(), "invalid board must not publish a project");
    let leftovers = std::fs::read_dir(root.path())
        .unwrap()
        .filter_map(|entry| entry.ok())
        .map(|entry| entry.file_name().to_string_lossy().to_string())
        .filter(|name| name != ".DS_Store")
        .collect::<Vec<_>>();
    assert!(
        leftovers.is_empty(),
        "staging or claim leaked: {leftovers:?}"
    );
}
#[test]
fn iot_scaffold_without_layer_or_local_source_fails_closed() {
    for (framework, board) in [
        ("platformio", "nodemcu-32s"),
        ("esp32-rust", "esp32c3"),
        ("zephyr-arm", "nrf52dk_nrf52832"),
    ] {
        let root = tempfile::tempdir().unwrap();
        let project_dir = root.path().join(format!("firmware-{framework}"));
        let config = ScaffoldConfig {
            core: "iot".to_string(),
            sub_type: String::new(),
            frameworks: vec![framework.to_string()],
            project_name: project_dir.to_string_lossy().to_string(),
            features: vec![board.to_string()],
            template_dir: PathBuf::new(),
            from_local: None,
            no_local: true,
        };

        // No registry layer, no local reuse: MagiCore must never synthesize
        // IoT firmware source.
        // Không có registry layer, không tái dùng local: MagiCore không bao
        // giờ tự sinh source firmware IoT.
        let error = Scaffolder::scaffold(&config).unwrap_err();
        assert!(
            error.to_string().contains("No scaffold source for core 'iot'"),
            "{framework}: {error}"
        );
        assert!(
            !project_dir.exists(),
            "{framework} failure must not leave a partial project"
        );
    }
}
