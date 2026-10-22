# Persistent CLI tool lifecycle / Lifecycle CLI tool toàn cục

## Purpose / Mục đích

This command manages user-scoped npm CLI tools through MagiCore's native Web install lane. It owns private data roots, recovery state, safe executable shims and update rollback. It does not invoke npm directly.

Lệnh này quản lý CLI npm theo user qua native Web install lane của MagiCore. Nó quản lý data root riêng tư, trạng thái khôi phục, shim an toàn và rollback update; không gọi npm trực tiếp.

## Owners / Các owner

- [`tool.rs`](tool.rs): stable command dispatch and nearest test binding.
- [`types`](types/README.md), [`install`](install/README.md), [`project_install`](project_install/README.md), [`update`](update/README.md), [`remove`](remove/README.md), [`list`](list/README.md).
- [`native_install`](native_install/README.md), [`managed_roots`](managed_roots/README.md), [`shim`](shim/README.md).
- [`record_lookup`](record_lookup/README.md), [`record_validation`](record_validation/README.md), [`record_write`](record_write/README.md), [`json_file`](json_file/README.md), [`atomic_file`](atomic_file/README.md).

## Errors and tests / Lỗi và kiểm thử

The implementation reuses `anyhow::Result` plus existing typed CLI errors; do not create a redundant tool-local error enum. Nearest unit tests remain in [`../test/tool.rs`](../test/tool.rs), and consumer behavior remains in [`cli/tests/global_tool_e2e.rs`](../../../tests/global_tool_e2e.rs). Both source files are preserved byte-identically during this structural pass.
