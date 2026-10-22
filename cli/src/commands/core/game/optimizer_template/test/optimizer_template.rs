//! Preserve existing end-to-end optimizer setup contracts.
//! Giữ hợp đồng setup optimizer end-to-end hiện có.
use super::super::game_optimizer_template;

#[tokio::test]
async fn optimizer_template_conflicting_value_fails_closed() {
    // Conflicting user values must survive and never be overwritten.
    // Giá trị xung đột của người dùng phải được giữ nguyên.
    let dir = tempfile::tempdir().unwrap();
    let root = dir.path();
    std::fs::write(
        root.join("Cargo.toml"),
        "[package]\nname = \"g\"\nversion = \"0.1.0\"\n\n[dependencies]\nmgc-optimizer = { path = \"./custom\" }\n",
    )
    .unwrap();
    game_optimizer_template(root)
        .await
        .expect_err("conflicting mgc-optimizer value must fail closed");
    let kept = std::fs::read_to_string(root.join("Cargo.toml")).unwrap();
    assert!(
        kept.contains("./custom"),
        "user value must survive:\n{kept}"
    );
}

#[tokio::test]
async fn optimizer_template_inserts_and_converges() {
    // A rerun converges to the same manifest bytes.
    // Chạy lại hội tụ về cùng byte của manifest.
    let dir = tempfile::tempdir().unwrap();
    let root = dir.path();
    std::fs::write(
        root.join("Cargo.toml"),
        "[package]\nname = \"g\"\nversion = \"0.1.0\"\n\n[dependencies]\n",
    )
    .unwrap();
    game_optimizer_template(root).await.unwrap();
    let once = std::fs::read_to_string(root.join("Cargo.toml")).unwrap();
    assert!(once.contains("./optimizer"), "dep must land:\n{once}");
    game_optimizer_template(root).await.unwrap();
    let twice = std::fs::read_to_string(root.join("Cargo.toml")).unwrap();
    assert_eq!(once, twice, "re-run must converge byte-identical");
}

#[tokio::test]
async fn optimizer_template_refuses_non_bevy_without_mutating_project() {
    // A Godot project must be refused before manifest or template changes.
    // Project Godot phải bị từ chối trước khi sửa manifest hay template.
    let dir = tempfile::tempdir().unwrap();
    let root = dir.path();
    std::fs::write(root.join("project.godot"), "config_version=5\n").unwrap();
    std::fs::write(
        root.join("Cargo.toml"),
        "[package]\nname = \"godot-helper\"\nversion = \"0.1.0\"\n\n[dependencies]\n",
    )
    .unwrap();
    let original_manifest = std::fs::read(root.join("Cargo.toml")).unwrap();

    let error = game_optimizer_template(root)
        .await
        .expect_err("Godot must not receive Bevy optimizer files");
    assert!(error.to_string().contains("Bevy"), "{error:#}");
    assert_eq!(
        std::fs::read(root.join("Cargo.toml")).unwrap(),
        original_manifest
    );
    assert!(!root.join("optimizer").exists());
    assert!(!root.join(".magicore").join("tmp-optimizer").exists());
}

#[tokio::test]
async fn optimizer_template_refuses_bevy_without_cargo_manifest_without_mutating() {
    // An incomplete Bevy project cannot report optimizer setup as success.
    // Project Bevy thiếu Cargo.toml không thể báo setup thành công.
    let dir = tempfile::tempdir().unwrap();
    let root = dir.path();
    std::fs::write(
        root.join("mgc.toml"),
        "name = \"demo\"\necosystem = \"game\"\n\n[game]\nengine = \"bevy\"\n",
    )
    .unwrap();
    std::fs::write(root.join(".mgc.core"), "game\n").unwrap();

    let error = game_optimizer_template(root)
        .await
        .expect_err("Bevy optimizer requires a Cargo.toml");
    assert!(error.to_string().contains("Cargo.toml"), "{error:#}");
    assert!(!root.join("optimizer").exists());
    assert!(!root.join(".magicore").join("tmp-optimizer").exists());
}
