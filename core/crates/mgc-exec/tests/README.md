# mgc-exec integration tests

Each integration target is declared in `core/crates/mgc-exec/Cargo.toml`. `run.rs` remains the entry point for Cargo target `test_run`; its behavior-specific modules live in `run/` with focused fixtures under `run/test_support/`. The other target files stay at this directory level until their own decomposition batch.

## Tiếng Việt

Mỗi integration target được khai báo trong `core/crates/mgc-exec/Cargo.toml`. `run.rs` vẫn là entry point của target `test_run`; các module theo hành vi nằm trong `run/`, fixture dùng chung ở `run/test_support/`. Những target khác giữ nguyên file hiện tại cho tới batch refactor riêng.
