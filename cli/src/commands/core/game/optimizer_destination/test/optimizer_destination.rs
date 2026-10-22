//! Validate missing and non-directory destination behavior.
//! Kiểm tra hành vi khi đích chưa có hoặc không phải thư mục.
use super::super::validate_optimizer_destination;
use crate::commands::core::game::error::GameOptimizerError;

#[test]
fn optimizer_destination_requires_a_directory_and_allows_missing_path() {
    let dir = tempfile::tempdir().unwrap();
    let missing = dir.path().join("optimizer");
    assert!(!validate_optimizer_destination(&missing).unwrap());

    std::fs::write(&missing, "not a package directory").unwrap();
    let error = validate_optimizer_destination(&missing).unwrap_err();
    let typed = error
        .downcast_ref::<GameOptimizerError>()
        .expect("typed non-directory destination error");
    assert!(typed.to_string().contains("exists but is not a directory"));
}
