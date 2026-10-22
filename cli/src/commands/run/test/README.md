# run tests

These are the nine existing `run` unit tests, moved next to the `run` command owner and grouped by behavior. `fixture_writer/` owns the shared temporary-file writer; `mod.rs` only wires modules and imports. `#[cfg(unix)]` stays on the explicit compat-runner test.

- `script_resolution.rs`: four manifest lookup and precedence cases.
- `lifecycle.rs`: three pre/main/post ordering and failure-precedence cases.
- `execution_policy.rs`: two package-manager/compat-runner gate cases.

The suite has not been run in the source-first phase. Source-static preservation does not prove compilation or runtime behavior.

# run tests (VI)

Đây là chín unit test cũ của `run`, được chuyển sát owner và chia theo hành vi. `fixture_writer/` sở hữu helper ghi file tạm; `mod.rs` chỉ wiring module/import. `#[cfg(unix)]` vẫn gắn với test compat-runner tương ứng.

- `script_resolution.rs`: bốn ca đọc manifest và ưu tiên nguồn.
- `lifecycle.rs`: ba ca thứ tự pre/main/post và ưu tiên lỗi.
- `execution_policy.rs`: hai ca chặn package manager và cho phép compat-runner.

Suite chưa chạy trong giai đoạn source-first. Giữ nguyên source không chứng minh code biên dịch hoặc chạy đúng.
