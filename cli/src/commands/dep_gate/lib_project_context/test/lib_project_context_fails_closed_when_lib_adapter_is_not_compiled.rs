//! Existing dependency-gate test — test dependency gate hiện có.
//! This subject keeps its complete pre-refactor assertion block.
//! Giữ nguyên toàn bộ assertion trước refactor.

#[cfg(not(any(feature = "lib", feature = "ai")))]
#[test]
fn lib_project_context_fails_closed_when_lib_adapter_is_not_compiled() {
    let root = tempfile::tempdir().unwrap();
    std::fs::write(
        root.path().join("Cargo.toml"),
        "[package]\nname='sample'\nversion='0.1.0'\nedition='2024'\n",
    )
    .unwrap();

    let context = lib_project_context(root.path(), DepOp::Install);

    assert!(context.ecosystem.is_none());
    assert!(matches!(owner_for(&context), DepOwner::Unsupported));
}
