use super::*;

#[tokio::test]
async fn verify_test_step_rejects_cores_without_a_verified_test_path() {
    // A verify chain must fail when its test step is not implemented for a core.
    // Chuỗi verify phải lỗi nếu bước test chưa được triển khai cho core đó.
    let tmp = tempfile::tempdir().expect("temporary project directory");
    for core in ["ai", "app", "game", "iot", "clo", "cicd", "hardware"] {
        let result = run_test_step(tmp.path(), core).await;
        assert!(
            result.is_err(),
            "verify must not report a skipped test step as success for core '{core}'"
        );
        let error = result.expect_err("unsupported verify test step must fail");
        assert!(error.to_string().contains(core));
    }
}
