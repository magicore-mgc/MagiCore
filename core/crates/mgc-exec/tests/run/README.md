# mgc-exec run integration tests

This folder decomposes the existing `test_run` Cargo integration target into behavior-focused owners. `tests/run.rs` remains the target entry point; CI and the Go stress lane keep their existing filters.

- `execution_contracts/` covers dry-run, validation, audit, missing executables, and timeout behavior.
- `project_child_lookup/` covers project-local command lookup and forbidden package-manager rejection.
- `process_tree_guard/` covers escaped child and sibling detection, termination, and reaping on Unix.
- `package_manager_policy/` covers shell/script policy, child execution, and project-binary checks.
- `go_runtime_probe/` contains the explicitly ignored real-Go SDK probe.
- `test_support/` owns temporary project and Unix process fixtures shared by the groups.

## Tiếng Việt

Thư mục này chia integration target Cargo `test_run` thành các owner theo hành vi. `tests/run.rs` vẫn là entry point; CI và lane stress Go giữ nguyên filter.

Mỗi owner ghi rõ phạm vi kiểm thử trong README riêng. `test_support/` chỉ chứa fixture của integration test; không tham gia product build.
