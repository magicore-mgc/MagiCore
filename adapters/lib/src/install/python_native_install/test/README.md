# Python native install tests

This folder holds PyPI artifact-cache verification and importability-preflight tests for the Python native installer. `mod.rs` wires the test files and reuses fixtures from `install/test/support.rs`; tests remain unexecuted until the source pass is complete.

## Test install native Python

Folder này giữ test xác minh cache artifact PyPI và preflight khả năng import của installer Python. `mod.rs` wiring các file test và dùng lại fixture ở `install/test/support.rs`; các test chưa chạy đến khi source pass hoàn tất.
