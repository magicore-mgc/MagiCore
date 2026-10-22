use super::*;

#[test]
fn verify_build_step_rejects_cicd_instead_of_reporting_success() {
    // CI/CD definitions do not produce build artifacts; a configured build must fail explicitly.
    // Định nghĩa CI/CD không tạo build artifact; bước build được cấu hình phải báo lỗi tường minh.
    let error = ensure_build_step_supported("cicd").expect_err("CI/CD build must not be skipped");
    assert!(error.to_string().contains("cicd"));
    assert!(error.to_string().contains("no build artifact"));
    assert!(ensure_build_step_supported("web").is_ok());
}
