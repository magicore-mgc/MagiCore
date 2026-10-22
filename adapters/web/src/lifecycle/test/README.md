# Lifecycle tests

EN: This folder keeps each lifecycle behavior test beside its production owner. Test-only environment overrides hold the adapter-wide mutex for their full lifetime and restore the previous value on drop.

VI: Folder này đặt test hành vi lifecycle gần owner production. Override biến môi trường chỉ dùng trong test giữ mutex chung của adapter trong toàn bộ thời gian chạy và phục hồi giá trị cũ khi drop.

- `environment.rs` — project-local PATH prefix.
- `error.rs` — typed failure classification.
- `manifest.rs` — invalid manifest fails closed.
- `project_identity.rs` — hook tampering, stop-later-hook, and restoration.
- `script_execution.rs` — shell-free execution, manifest snapshot, env assignment, timeout, and nonzero exit.
- `script_policy.rs` — package-manager delegation and forbidden runtime commands.
- `helpers.rs` — package manifest fixture and scoped process environment.

These are library unit tests registered by `lifecycle/mod.rs`; no separate Cargo target is created.
