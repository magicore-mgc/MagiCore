# Tests for `compat_runner`

These focused tests are registered through the nearest owner facade. Existing sources have not been executed in this structural phase.

Test sources:
- `compat_runner.rs`

# Test cho `compat_runner`

Các test tập trung được đăng ký qua facade gần nhất. Test source chưa chạy trong phase refactor cấu trúc.

File test:
- `compat_runner.rs`

## Registration and change rule

The owner `mod.rs` registers these files under `#[cfg(test)]` and `#[path = "test/mod.rs"]`; keep test bodies in the named files, never inline. Keep each assertion with the narrowest behavior owner; preserve the public/process integration tests in `mgc-exec/tests/`.

Run after the structural phase: `cargo test -p mgc-exec`. This command has **not** been run as part of the current structural pass.

## Đăng ký và quy tắc thay đổi

`mod.rs` của owner đăng ký các file qua `#[cfg(test)]` và `#[path = "test/mod.rs"]`; giữ body test trong file có tên, không viết inline. Đặt assertion sát behavior owner nhất; giữ integration test public/process tại `mgc-exec/tests/`.

Chạy sau phase cấu trúc: `cargo test -p mgc-exec`. Lệnh này **chưa chạy** trong batch cấu trúc hiện tại.
