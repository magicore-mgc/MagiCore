// Scaffold transaction and rollback tests.
// Kiểm thử transaction scaffold và rollback.

// ===== Atomic scaffold regression (release contract §5 "fail atomically") =====
// Regression cho hợp đồng atomic scaffold: project phải xuất hiện nguyên vẹn
// hoặc KHÔNG xuất hiện — không bao giờ partial. Rename từ staging cùng FS.

#[test]
fn test_scaffold_is_atomic_no_partial_on_failure() {
    // A template that fails mid-write (unsupported core file) must leave NO
    // directory at the target path — only the staging temp is cleaned up.
    // Template fail giữa chừng phải KHÔNG để lại target directory.
    let root = tempfile::tempdir().unwrap();
    let target = root.path().join("must-not-exist");
    let config = ScaffoldConfig {
        core: "nonexistent-core-xyz".to_string(),
        sub_type: String::new(),
        frameworks: vec!["vanilla".to_string()],
        project_name: target.to_string_lossy().to_string(),
        features: vec![],
        template_dir: PathBuf::new(),
        from_local: None,
        no_local: false,
    };
    let result = Scaffolder::scaffold(&config);
    assert!(result.is_err(), "scaffold with unknown core must fail");
    assert!(
        !target.exists(),
        "target must NOT exist after atomic failure"
    );
    // No staging leftovers inside the parent either — and the claim slot
    // must be RELEASED on failure so a retry can proceed.
    // Không staging rác trong parent — và claim-slot phải ĐƯỢC GIẢI PHÓNG
    // khi fail để lần retry sau chạy được.
    assert!(
        !root.path().join(".mgc-create-must-not-exist.lock").exists(),
        "claim slot must be released after atomic failure"
    );
    let leftovers: Vec<String> = std::fs::read_dir(root.path())
        .unwrap()
        .filter_map(|e| e.ok())
        .map(|e| e.file_name().to_string_lossy().to_string())
        .filter(|n| n != ".DS_Store")
        .collect();
    assert!(
        leftovers.is_empty(),
        "no staging dirs may leak, found: {leftovers:?}"
    );
}
#[test]
fn test_claim_target_reports_existing_directory_and_releases_claim() {
    let root = tempfile::tempdir().unwrap();
    let target = root.path().join("claim-existing");
    std::fs::create_dir(&target).unwrap();
    std::fs::write(target.join("keep.txt"), "winner").unwrap();
    let claim = Scaffolder::claim_slot_path(&target);

    let error = Scaffolder::claim_target(&target).unwrap_err();

    assert!(
        error.to_string().contains("already exists"),
        "existing target must report a conflict, got: {error}"
    );
    assert!(!claim.exists(), "claim slot must be released on conflict");
    assert_eq!(
        std::fs::read_to_string(target.join("keep.txt")).unwrap(),
        "winner"
    );
}
#[test]
fn test_scaffold_rename_conflict_fails_closed() {
    // If the target directory appears between the exists-check and rename
    // (race window), scaffold must fail closed without touching the winner.
    // Nếu target xuất hiện trong cửa sổ race, scaffold phải fail-closed và
    // không đụng project đã thắng.
    let root = tempfile::tempdir().unwrap();
    let template = online_web_template_fixture();
    let target = root.path().join("race-target");
    let first = ScaffoldConfig {
        core: "web".to_string(),
        sub_type: "frontend".to_string(),
        frameworks: vec!["vanilla".to_string()],
        project_name: target.to_string_lossy().to_string(),
        features: vec![],
        template_dir: template.path().to_path_buf(),
        from_local: None,
        no_local: false,
    };
    assert!(Scaffolder::scaffold(&first).is_ok());
    let before: Vec<String> = std::fs::read_dir(&target)
        .unwrap()
        .filter_map(|e| e.ok())
        .map(|e| e.file_name().to_string_lossy().to_string())
        .collect();
    let second = ScaffoldConfig {
        project_name: target.to_string_lossy().to_string(),
        ..first
    };
    let err = Scaffolder::scaffold(&second);
    assert!(err.is_err(), "double scaffold must fail");
    let after: Vec<String> = std::fs::read_dir(&target)
        .unwrap()
        .filter_map(|e| e.ok())
        .map(|e| e.file_name().to_string_lossy().to_string())
        .collect();
    assert_eq!(
        before, after,
        "existing project content must be untouched after conflict"
    );
}
#[test]
fn test_scaffold_success_leaves_no_staging_sibling() {
    // After a successful scaffold, no hidden staging temp dir may remain
    // next to the target — only the project itself.
    // Scaffold thành công không để staging rác cạnh target.
    let root = tempfile::tempdir().unwrap();
    let template = online_web_template_fixture();
    let target = root.path().join("clean-project");
    let config = ScaffoldConfig {
        core: "web".to_string(),
        sub_type: "frontend".to_string(),
        frameworks: vec!["vanilla".to_string()],
        project_name: target.to_string_lossy().to_string(),
        features: vec![],
        template_dir: template.path().to_path_buf(),
        from_local: None,
        no_local: false,
    };
    let out = Scaffolder::scaffold(&config).unwrap();
    assert!(out.exists());
    let siblings: Vec<String> = std::fs::read_dir(root.path())
        .unwrap()
        .filter_map(|e| e.ok())
        .map(|e| e.file_name().to_string_lossy().to_string())
        .filter(|n| n != "clean-project" && n != ".DS_Store")
        .collect();
    assert!(
        siblings.is_empty(),
        "no staging siblings may remain, found: {siblings:?}"
    );
}
