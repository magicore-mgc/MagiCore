//! Focused local dependency edit and conflict preservation tests.
//! Test tập trung cho sửa dependency local và giữ giá trị xung đột.
use super::super::game_edit_optimizer_dep;

#[tokio::test]
async fn dependency_edit_inserts_and_converges() {
    let dir = tempfile::tempdir().unwrap();
    let root = dir.path();
    std::fs::write(
        root.join("Cargo.toml"),
        "[package]\nname = \"g\"\nversion = \"0.1.0\"\n\n[dependencies]\n",
    )
    .unwrap();

    game_edit_optimizer_dep(root).await.unwrap();
    let once = std::fs::read_to_string(root.join("Cargo.toml")).unwrap();
    assert!(once.contains("./optimizer"));

    game_edit_optimizer_dep(root).await.unwrap();
    let twice = std::fs::read_to_string(root.join("Cargo.toml")).unwrap();
    assert_eq!(once, twice);
}

#[tokio::test]
async fn dependency_edit_preserves_conflicting_user_value() {
    let dir = tempfile::tempdir().unwrap();
    let root = dir.path();
    let manifest = "[package]\nname = \"g\"\nversion = \"0.1.0\"\n\n[dependencies]\nmgc-optimizer = { path = \"./custom\" }\n";
    std::fs::write(root.join("Cargo.toml"), manifest).unwrap();

    assert!(game_edit_optimizer_dep(root).await.is_err());
    assert_eq!(
        std::fs::read_to_string(root.join("Cargo.toml")).unwrap(),
        manifest
    );
}
