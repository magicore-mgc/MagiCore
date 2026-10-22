# Runner Workspace

Create an ephemeral project inside the purpose-specific MGC work root, write its exact package/config snapshot, validate schema, and copy a regular caller `.npmrc` when present.
Tạo project tạm trong work root MGC theo purpose, ghi package/config snapshot, kiểm schema và chép `.npmrc` caller dạng regular file nếu có.

## Files / File sở hữu

- `mod.rs` wires and re-exports this owner.
- `runner_workspace.rs`: `runner_workspace.rs` owns workspace lifetime, roots and project containment; unsafe-root diagnostics are owned by `work_root/error.rs`. `mod.rs` exports workspace creation to package runtime.
- `test/runner_workspace.rs`: `test/runner_workspace.rs` checks package/config/npmrc materialization.

`runner_workspace.rs sở hữu lifetime workspace, root và containment; lỗi root do work_root/error.rs sở hữu. mod.rs export tạo workspace.`

Errors outside this owner are propagated unchanged; this owner does not invent a local diagnostic variant.
Lỗi không thuộc owner này được propagate nguyên trạng; owner không tạo variant diagnostic giả.

## Test and limits / Test và giới hạn

`test/runner_workspace.rs` checks package/config/npmrc materialization. The `TempDir` is held in the returned record until execution ends. It does not own caller project files.

test/runner_workspace.rs kiểm materialization package/config/npmrc. TempDir được giữ đến hết execution; không sở hữu file project caller.

The registered unit command is `cargo test -p mgc --lib commands::package_runtime::`; it has not been run in this working session.
Lệnh unit đã đăng ký là `cargo test -p mgc --lib commands::package_runtime::`; chưa chạy trong phiên này.
