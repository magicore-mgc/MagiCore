# Lockfile integration tests / Test integration lockfile

Top-level `*.rs` files are Cargo integration-test targets and their names are compatibility surface. Focused policy tests live under `policy/` and are included from the original target roots so test functions remain at their original crate-root scope. Tests are authored/moved source, not evidence of execution; current verification status is NOT_RUN.

Các file `*.rs` ở gốc là Cargo integration-test target, giữ nguyên tên. Test policy được chia trong `policy/` và include từ target gốc để giữ scope crate-root. Source test đã chuyển nhưng trạng thái chạy là NOT_RUN.
