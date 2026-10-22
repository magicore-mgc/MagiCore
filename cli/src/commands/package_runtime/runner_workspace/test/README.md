# Runner Workspace tests

- `cli/src/commands/package_runtime/runner_workspace/test/runner_workspace.rs` — Workspace package/config/npmrc materialization; the owner keeps the temporary directory alive through execution.
- Registered from `cli/src/commands/package_runtime/runner_workspace/runner_workspace.rs` through `#[path = "test/runner_workspace.rs"]`.

- `cli/src/commands/package_runtime/runner_workspace/test/runner_workspace.rs` — Kiểm materialization package/config/npmrc; owner giữ thư mục tạm đến hết execution.
- Đăng ký tại `cli/src/commands/package_runtime/runner_workspace/runner_workspace.rs` qua `#[path = "test/runner_workspace.rs"]`.

This owner propagates adjacent-owner and child-process errors unchanged, so it has no separate typed-error test. These unit tests do not replace dlx production-binary E2E and are currently `NOT_RUN`.
Owner propagate lỗi của owner liền kề và process con nguyên trạng nên không có test lỗi typed riêng. Test unit không thay dlx production-binary E2E và hiện là `NOT_RUN`.
