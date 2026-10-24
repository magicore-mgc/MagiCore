//! Tests for initializer path validation before package execution.
//! Test kiểm tra path initializer trước khi chạy package.

#[tokio::test]
async fn online_vite_generator_rejects_path_traversal_before_running_dlx() {
    let mut config = crate::scaffold::Scaffolder::infer_web_create_config("react-vite", "safe")
        .expect("valid web scaffold config");
    // Call the renamed initializer entry point: `run_online_web_generator` was
    // renamed to `run_create_initializer` when the create flow was unified on the
    // shared package-runtime engine; the traversal gate itself is unchanged and
    // still fires inside `validate_project_name` before any package is launched.
    // Gọi đúng entry point sau đổi tên: `run_online_web_generator` đã đổi thành
    // `run_create_initializer` khi luồng create hợp nhất về package-runtime dùng
    // chung; gate traversal không đổi và vẫn nổ trong `validate_project_name`
    // trước khi bất kỳ package nào được chạy.
    let error = super::run_create_initializer(
        &mut config,
        "../outside-project",
        super::super::super::lookup(super::super::super::Mode::Frontend, "react-vite")
            .expect("react-vite has an official initializer"),
    )
    .await
    .expect_err("generator must reject path traversal before launching dlx");

    assert!(error.to_string().contains("path traversal rejected"));
}
