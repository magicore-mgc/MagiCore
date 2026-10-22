# Tests for `tool_check`

These focused tests are registered through the nearest owner facade. Existing sources have not been executed in this structural phase.

Test sources:
- `tool_check.rs`

The owner tests now cover the project-binary scope gate as well as the static tool gate: `adb`/`xcrun` remain DeviceControl-only and language runtimes remain limited to project execution scopes. The helper deliberately skips the static host-tool allowlist for resolved package binaries.

# Test cho `tool_check`

Các test tập trung được đăng ký qua facade gần nhất. Test source chưa chạy trong phase refactor cấu trúc.

File test:
- `tool_check.rs`

Test owner hiện bao phủ scope gate của project binary cùng static tool gate: `adb`/`xcrun` chỉ thuộc DeviceControl và runtime chỉ thuộc scope chạy project. Helper cố ý không áp allowlist tool host lên binary package đã resolve.

## Registration and change rule

The owner `mod.rs` registers these files under `#[cfg(test)]` and `#[path = "test/mod.rs"]`; keep test bodies in the named files, never inline. Keep each assertion with the narrowest behavior owner; preserve the public/process integration tests in `mgc-exec/tests/`.

Run after the structural phase: `cargo test -p mgc-exec`. This command has **not** been run as part of the current structural pass.

## Đăng ký và quy tắc thay đổi

`mod.rs` của owner đăng ký các file qua `#[cfg(test)]` và `#[path = "test/mod.rs"]`; giữ body test trong file có tên, không viết inline. Đặt assertion sát behavior owner nhất; giữ integration test public/process tại `mgc-exec/tests/`.

Chạy sau phase cấu trúc: `cargo test -p mgc-exec`. Lệnh này **chưa chạy** trong batch cấu trúc hiện tại.
