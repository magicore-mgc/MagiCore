# Shared package runtime / Runtime package dùng chung

S184 decomposes the package installation, verification and execution flow shared by `mgc dlx`, the Web online initializer, and managed-tool lifecycle operations. The boundary always installs through the current MagiCore executable; no external package manager is invoked.

S184 tách luồng cài, xác minh và chạy package dùng chung giữa `mgc dlx`, Web online initializer và managed-tool lifecycle. Luồng luôn cài bằng executable MagiCore hiện tại; không gọi package manager ngoài.

## File ownership / Trách nhiệm file

| Path | Responsibility |
|---|---|
| `mod.rs` | Child owner declarations and stable crate-visible re-exports. |
| `package_runtime.rs` | Compatibility facade for `run_registry_package_in`, purpose/report types, validators, and install-policy helpers. |
| `request_validation/request_validation.rs` | Registry package name and version/tag/range parsing. |
| `bin_selection/bin_selection.rs` | Declared executable name validation and deterministic selection. |
| `installed_metadata/installed_metadata.rs` | Requested package manifest identity, bin declarations, and semantic version. |
| `install_policy/install_policy.rs` | Native install arguments and selected caller policy inheritance. |
| `work_root/work_root.rs` | Purpose-specific `MGC_DATA_DIR` subpaths and symlink-resistant directory checks. |
| `node_runtime/node_runtime.rs` | JavaScript entrypoint detection, host Node lookup, and quoted temporary launcher. |
| `runner_workspace/runner_workspace.rs` | Temporary project/config/npmrc creation and containment under the MGC work root. |
| `native_install/native_install.rs` | Self-invocation of the current MGC binary for package installation. |
| `installed_binary/installed_binary.rs` | Post-install package/bin ownership and target containment verification. |
| `execute_package/execute_package.rs` | PATH/environment/scope construction and execution of the verified binary. |
| `run_registry_package/run_registry_package.rs` | Ordered parse → prepare → native install → verify → execute coordinator and caller-cwd gate. |

## Error owners / Chủ sở hữu lỗi

Each error type is in the nearest functional owner: `request_validation/error.rs`, `bin_selection/error.rs`, `installed_metadata/error.rs`, `install_policy/error.rs`, `work_root/error.rs`, `installed_binary/error.rs`, `run_registry_package/error.rs`, and `node_runtime/error.rs`. The shared `cli/src/error/messages/mod.rs` path only re-exports these factories for compatibility; it does not own the types. Invalid request/bin/policy, unsafe path, invalid package metadata, unowned executable, invalid execution directory, and missing host Node are non-retryable until the user corrects the request or environment. Native I/O and child-process errors continue through their original error chain.

Mỗi type lỗi nằm gần owner chức năng nhất: các `error.rs` ở tám thư mục ghi trên. `cli/src/error/messages/mod.rs` chỉ tái xuất factory để giữ tương thích, không sở hữu type. Request/bin/policy sai, path nguy hiểm, metadata package sai, executable không thuộc project, execution directory sai và thiếu Node đều không tự retry; cần sửa request hoặc môi trường. Lỗi I/O và process con tiếp tục giữ error chain gốc.

## Inputs, effects, and limits / Đầu vào, side effect và giới hạn

- Inputs are the package selector, optional `--bin`, script permission, args, execution cwd, and the caller project policy snapshot.
- The installer child is the current `mgc` executable with `--core web`; registry/security/lock/trust policy is inherited, while compatibility execution policy is not.
- Temporary install roots are under `MGC_DATA_DIR/cache/web/dlx` or `MGC_DATA_DIR/cache/web/create-initializers`; the initializer output cwd belongs to its caller.
- The installed package is inspected before execution. Package bin targets must remain inside the verified package/project. The runner uses the existing process scope and `.magicore/exec.log` path.
- Unit tests are registered in each owner implementation with `#[path = "test/..."]`. The source-only command is `cargo test -p mgc --lib commands::package_runtime::`; the 2 dlx product E2E targets are separate. The 43 source unit tests (35 package-runtime behavior tests and 8 owner error-contract tests) are currently `NOT_RUN`; no runtime/product pass is claimed.

Đầu vào gồm package selector, `--bin` tùy chọn, quyền lifecycle script, args, execution cwd và snapshot policy project caller. Installer con là executable `mgc` hiện tại với `--core web`; kế thừa registry/security/lock/trust nhưng không kế thừa compat execution. Workspace tạm thuộc MGC dưới `MGC_DATA_DIR/cache/web/{dlx,create-initializers}`; cwd sinh initializer thuộc caller. Package/bin được xác minh containment trước khi chạy; process scope và đường dẫn `.magicore/exec.log` được giữ. Test unit đăng ký ngay trong implementation owner; lệnh dự kiến là `cargo test -p mgc --lib commands::package_runtime::`, còn 2 dlx E2E là lớp riêng. Hiện 43 test source (35 test behavior package runtime và 8 test contract lỗi owner) đều `NOT_RUN`; không khẳng định runtime/product pass.
